// mod moon_color;
mod ptm_interact_cli;
mod ptm_interact_config;
mod ptm_interact_image;
mod ptm_interact_state;

use clap::Parser;
use image::{GenericImage, ImageReader};
use moon_color::MoonColor;
use multiversx_sc_snippets::sdk::gateway::GatewayAsyncService;
use multiversx_sc_snippets::{imports::*, sdk::gateway::GetAccountStorageRequest};
use paint_the_moon_sc::pixel_block::PixelBlockData8;
use paint_the_moon_sc::{paint_proxy, PixelBlock};
pub use ptm_interact_config::Config;
use ptm_interact_state::State;

pub type Map = [[MoonColor; 512]; 1024];

pub struct Point {
    pub x: usize,
    pub y: usize,
    pub color: MoonColor,
}

const CODE_PATH: MxscPath = MxscPath::new("../paint-the-moon-sc/output/paint-the-moon.mxsc.json");

pub async fn main_cli() {
    env_logger::init();

    let config = Config::load_config();

    let mut basic_interact = MoonInteract::init(config).await;

    let cli = ptm_interact_cli::InteractCli::parse();
    match &cli.command {
        Some(ptm_interact_cli::InteractCliCommand::Deploy) => {
            basic_interact.deploy().await;
        }
        Some(ptm_interact_cli::InteractCliCommand::Render) => {
            basic_interact.render().await;
        }
        Some(ptm_interact_cli::InteractCliCommand::PaintImg) => {
            basic_interact.paint_image().await;
        }
        _ => {}
    }
}

#[allow(unused)]
pub struct MoonInteract {
    pub interactor: Interactor,
    pub owner_address: Bech32Address,
    pub state: State,
    pub config: Config,
    pub current_map: Option<Box<Map>>,
}

impl MoonInteract {
    pub async fn init(config: Config) -> Self {
        let mut interactor =
            Interactor::new(config.gateway_uri(), config.use_chain_simulator()).await;

        let owner_address = interactor
            .register_wallet(Wallet::from_pem_file("paint-owner.pem").unwrap())
            .await;

        // generate blocks until ESDTSystemSCAddress is enabled
        interactor.generate_blocks_until_epoch(1).await.unwrap();

        Self {
            interactor,
            owner_address: owner_address.into(),
            state: State::load_state(),
            config,
            current_map: None,
        }
    }

    pub async fn set_state(&mut self) {
        println!("wallet address: {}", self.owner_address);
        self.interactor.retrieve_account(&self.owner_address).await;
    }

    pub async fn deploy(&mut self) {
        // warning: multi deploy not yet fully supported
        // only works with last deployed address

        self.set_state().await;

        let new_address = self
            .interactor
            .tx()
            .from(&self.owner_address)
            .gas(6_000_000)
            .typed(paint_proxy::PaintTheMoonScProxy)
            .init()
            .code(CODE_PATH)
            .code_metadata(CodeMetadata::UPGRADEABLE)
            .returns(ReturnsNewBech32Address)
            .run()
            .await;

        println!("new address: {new_address}");
    }

    async fn retrieve_map(&self) -> Box<Map> {
        let account_storage = self
            .interactor
            .proxy
            .request(GetAccountStorageRequest::new(
                &self.config.moon_address.clone().into_address(),
            ))
            .await
            .unwrap_or_else(move |err| {
                panic!(
                    "failed to retrieve storage for address {}: {err}",
                    &self.config.moon_address
                )
            });

        let mut map = [[MoonColor::TRANSPARENT; 512]; 1024];

        let key_prefix = hex::encode("blocks");

        for (key, value) in account_storage.iter() {
            if let Some(key_xy) = key.strip_prefix(&key_prefix) {
                assert_eq!(key_xy.len(), 16, "bad length for blocks storage key");
                let mut x_bytes = [0u8; 4];
                hex::decode_to_slice(&key_xy[0..8], &mut x_bytes)
                    .expect("error decoding x coord from storage key");
                let block_x = u32::from_be_bytes(x_bytes) as usize;
                let mut y_bytes = [0u8; 4];
                hex::decode_to_slice(&key_xy[8..16], &mut y_bytes)
                    .expect("error decoding x coord from storage key");
                let block_y = u32::from_be_bytes(y_bytes) as usize;

                println!("x: {block_x}, y: {block_y}, value: {value}");

                let block_bytes =
                    hex::decode(&value).expect("could not hex-decode storage value for block");

                let block = PixelBlock::<PixelBlockData8>::from_bytes(&block_bytes);
                for x in 0..8 {
                    for y in 0..8 {
                        let raw_pixel = block.get_raw_pixel(x, y);
                        map[block_x * 8 + x][block_y * 8 + y] =
                            MoonColor::try_new(raw_pixel).expect("bad pixel");
                    }
                }
            }
        }

        // self.current_map = Some(map);
        Box::new(map)
    }

    pub async fn refresh_map(&mut self) {
        let map = self.retrieve_map().await;
        self.current_map = Some(map);
    }

    pub async fn get_map(&mut self) -> Box<Map> {
        if let Some(map) = &self.current_map {
            map.clone()
        } else {
            let map = self.retrieve_map().await;
            self.current_map = Some(map.clone());
            map
        }
    }

    pub async fn render(&mut self) {
        self.render_map().await.expect("could not render map");
    }

    pub async fn render_map(&mut self) -> anyhow::Result<()> {
        let map = self.retrieve_map().await;

        let image = ImageReader::open("lroc_color_poles_2k.tif")?.decode()?;
        let mut image = image.resize(1024, 512, image::imageops::FilterType::Nearest);

        for x in 0..1024 {
            for y in 0..512 {
                let map_color = map[x][y];
                if !map_color.is_transparent() {
                    image.put_pixel(x as u32, y as u32, map_color.rgba_array().into());
                }
            }
        }

        image.save("rendered_flat.png")?;

        Ok(())
    }

    pub async fn paint_image(&mut self) {
        let changed_points = self.compose_image().await.expect("failed to compose image");

        let mut total_gas_used = 0;

        for (batch_index, window) in changed_points.chunks(100).enumerate() {
            println!(
                "Starting batch of points #{batch_index}, {} points ...",
                window.len()
            );

            let mut buffer = self.interactor.homogenous_call_buffer();
            for point in window {
                buffer.push_tx(|tx| {
                    tx.from(&self.owner_address)
                        .to(&self.config.moon_address)
                        .typed(paint_proxy::PaintTheMoonScProxy)
                        .paint(point.x, point.y, point.color.as_byte())
                        .gas(3_000_000)
                        .returns(ReturnsGasUsed)
                });
            }

            let result = buffer.run().await;
            let batch_gas: u64 = result.iter().sum();
            let batch_avg = batch_gas / window.len() as u64;

            total_gas_used += batch_gas;

            println!("Finished batch of points");
            println!("Batch gas:        {batch_gas}");
            println!("Batch avg gas/tx: {batch_avg}");
            println!("Cumulated gas:    {total_gas_used}");
        }
    }
}
