#![no_std]

const COLORS: [(&str, u8, u8, u8, u8); 16] = [
    ("transparent", 0, 0, 0, 0),
    ("white", 255, 255, 255, 255),
    ("black", 0, 0, 0, 255),
    ("red", 255, 0, 0, 255),
    ("green", 0, 255, 0, 255),
    ("blue", 0, 0, 255, 255),
    ("yellow", 255, 255, 0, 255),
    ("cyan", 0, 255, 255, 255),
    ("magenta", 255, 0, 255, 255),
    ("maroon", 128, 0, 0, 255),
    ("grey", 128, 128, 128, 255),
    ("orange", 255, 165, 0, 255),
    ("purple", 180, 0, 210, 255),
    ("teal", 0, 128, 128, 255),
    ("pink", 255, 105, 180, 255),
    ("mint", 35, 247, 221, 255),
];

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct MoonColor(u8);

impl MoonColor {
    pub const TRANSPARENT: MoonColor = MoonColor(0);
    pub const WHITE: MoonColor = MoonColor(1);
    pub const BLACK: MoonColor = MoonColor(2);
    pub const RED: MoonColor = MoonColor(3);
    pub const GREEN: MoonColor = MoonColor(4);
    pub const BLUE: MoonColor = MoonColor(5);
    pub const YELLOW: MoonColor = MoonColor(6);
    pub const CYAN: MoonColor = MoonColor(7);
    pub const MAGENTA: MoonColor = MoonColor(8);
    pub const MAROON: MoonColor = MoonColor(9);
    pub const GREY: MoonColor = MoonColor(10);
    pub const ORANGE: MoonColor = MoonColor(11);
    pub const PURPLE: MoonColor = MoonColor(12);
    pub const TEAL: MoonColor = MoonColor(13);
    pub const PINK: MoonColor = MoonColor(14);
    pub const MINT: MoonColor = MoonColor(15);

    pub fn try_new(value: u8) -> Option<Self> {
        if value < 16 {
            Some(MoonColor(value))
        } else {
            None
        }
    }

    pub fn as_byte(self) -> u8 {
        self.0
    }

    pub fn name(self) -> &'static str {
        COLORS[self.0 as usize].0
    }

    pub fn rgb(self) -> (u8, u8, u8) {
        let (_, r, g, b, _) = COLORS[self.0 as usize];
        (r, g, b)
    }

    pub fn rgba_array(self) -> [u8; 4] {
        let (_, r, g, b, a) = COLORS[self.0 as usize];
        [r, g, b, a]
    }

    pub fn is_transparent(self) -> bool {
        self.0 == 0
    }

    pub fn closest_color_euclidian(r: u8, g: u8, b: u8) -> Self {
        let mut best_color = 0;
        let mut best_distance = i32::MAX;
        for i in 1..16 {
            let (_, color_r, color_g, color_b, _) = unsafe { COLORS.get_unchecked(i) };
            let dr = *color_r as i32 - r as i32;
            let dg = *color_g as i32 - g as i32;
            let db = *color_b as i32 - b as i32;
            let distance = dr * dr + dg * dg + db * db;
            if distance < best_distance {
                best_distance = distance;
                best_color = i;
            }
        }
        MoonColor(best_color as u8)
    }

    pub fn all_colors_iter() -> impl Iterator<Item = MoonColor> {
        (0..15).map(|i| MoonColor(i))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn closest_color_euclidian_self_test() {
        for color in MoonColor::all_colors_iter().skip(1) {
            let (r, g, b) = color.rgb();
            assert_eq!(color, MoonColor::closest_color_euclidian(r, g, b));
        }
    }

    #[test]
    fn color_name_test() {
        assert_eq!(MoonColor::TRANSPARENT.name(), "transparent");
        assert_eq!(MoonColor::WHITE.name(), "white",);
        assert_eq!(MoonColor::BLACK.name(), "black");
        assert_eq!(MoonColor::RED.name(), "red");
        assert_eq!(MoonColor::GREEN.name(), "green");
        assert_eq!(MoonColor::BLUE.name(), "blue");
        assert_eq!(MoonColor::YELLOW.name(), "yellow");
        assert_eq!(MoonColor::CYAN.name(), "cyan");
        assert_eq!(MoonColor::MAGENTA.name(), "magenta");
        assert_eq!(MoonColor::MAROON.name(), "maroon");
        assert_eq!(MoonColor::GREY.name(), "grey");
        assert_eq!(MoonColor::ORANGE.name(), "orange");
        assert_eq!(MoonColor::PURPLE.name(), "purple");
        assert_eq!(MoonColor::TEAL.name(), "teal");
        assert_eq!(MoonColor::PINK.name(), "pink");
        assert_eq!(MoonColor::MINT.name(), "mint");
    }
}
