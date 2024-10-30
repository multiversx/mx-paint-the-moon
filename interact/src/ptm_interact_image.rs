use std::convert::Infallible;

use anyhow::anyhow;
use image::{ColorType, DynamicImage, GenericImage, GenericImageView, ImageReader, Rgba};
use moon_color::MoonColor;

use crate::{Map, MoonInteract, Point};

impl MoonInteract {
    pub async fn compose_image(&mut self) -> anyhow::Result<Vec<Point>> {
        let map = self.get_map().await;

        let image = ImageReader::open("egld-logo.png")?.decode()?;
        println!("image size: {} x {}", image.width(), image.height());
        let egld_logo = image.resize(100, 100, image::imageops::FilterType::Nearest);
        println!("resized:    {} x {}", egld_logo.width(), egld_logo.height());

        let changed_points = add_overlay(&egld_logo, 512, 128, &map).await?;

        println!("points to change: {}", changed_points.len());

        Ok(changed_points)
    }
}

fn color_is_transparent(color: Rgba<u8>) -> bool {
    color.0[3] < 255
}

pub async fn add_overlay(
    image: &DynamicImage,
    start_x: usize,
    start_y: usize,
    map: &Map,
) -> anyhow::Result<Vec<Point>> {
    let mut changed_points = Vec::new();

    for (x, y, color) in image.pixels() {
        if color_is_transparent(color) {
            continue;
        }

        let normalized_color =
            MoonColor::closest_color_euclidian(color.0[0], color.0[1], color.0[2]);
        let target_x = start_x + x as usize;
        let target_y = start_y + y as usize;
        if target_x > 1024 {
            return Err(anyhow!("x coordinate exceeded"));
        }
        if target_y > 512 {
            return Err(anyhow!("y coordinate exceeded"));
        }
        if map[target_x][target_y] != normalized_color {
            changed_points.push(Point {
                x: target_x,
                y: target_y,
                color: normalized_color,
            });
        }
    }

    Ok(changed_points)
}

pub fn save_sphere(size: u32, long0: f32, source: &DynamicImage, file_name: &str) -> anyhow::Result<()> {
    let mut rendered = DynamicImage::new(size, size, ColorType::Rgb8);

    let _ = sphere::render_sphere(size, long0, source, |x, y, r, g, b| {
        rendered.put_pixel(x, y, [r, g, b, 255u8].into());
        Result::<(), Infallible>::Ok(())
    });

    rendered.save(file_name)?;

    Ok(())
}
