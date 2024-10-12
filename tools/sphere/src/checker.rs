use image::{ColorType, DynamicImage, GenericImage};

fn xor(a: bool, b: bool) -> bool {
    (a && !b) || (!a && b)
}

pub fn generate_checker(width: u32, height: u32) -> DynamicImage {
    let mut checker = DynamicImage::new(width, height, ColorType::Rgb8);
    let checker_size = 25;
    let half_width = width / 2;
    let half_height = height / 2;
    for x in 0..width {
        for y in 0..height {
            if xor(x / checker_size % 2 == 0, y / checker_size % 2 == 0) {
                let pixel = if xor(x < half_width, y < half_height) {
                    let c = (x * 256 / width) as u8;
                    [c, 0, 0, 255u8].into()
                } else {
                    let c = (x * 256 / width) as u8;
                    [0, c, 0, 255u8].into()
                };
                checker.put_pixel(x, y, pixel);
            }
        }
    }

    checker
}
