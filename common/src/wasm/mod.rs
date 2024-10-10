const PAINT_THE_MOON_CODE: &[u8] = include_bytes!("paint-the-moon-sc.wasm");
const PAINT_HARVEST_CODE: &[u8] = include_bytes!("paint-harvest-sc.wasm");

pub struct ContractCode {
    pub paint_the_moon: &'static [u8],
    pub paint_harvest: &'static [u8],
}

impl ContractCode {
    pub const fn new() -> Self {
        ContractCode {
            paint_the_moon: PAINT_THE_MOON_CODE,
            paint_harvest: PAINT_HARVEST_CODE,
        }
    }
}

impl Default for ContractCode {
    fn default() -> Self {
        Self::new()
    }
}

// export a static instance for other crates to use
pub const CONTRACT_CODE: ContractCode = ContractCode::new();
