use multiversx_sc::{api::ManagedTypeApi, imports::*};

pub trait PixelBlockData: Default {
    const SIZE_BITS: usize;
    const RAW_DATA_LEN: usize;

    fn raw_data(&self) -> &[u8];

    fn raw_data_mut(&mut self) -> &mut [u8];
}

macro_rules! pixel_block_data {
    ($struct_name:ident, $size_in_bits:expr, $raw_data_size:expr) => {
        #[derive(Clone)]
        pub struct $struct_name {
            data: [u8; $raw_data_size],
        }

        impl Default for $struct_name {
            fn default() -> Self {
                Self {
                    data: [0u8; $raw_data_size],
                }
            }
        }

        impl PixelBlockData for $struct_name {
            const SIZE_BITS: usize = $size_in_bits;
            const RAW_DATA_LEN: usize = $raw_data_size;

            #[inline]
            fn raw_data(&self) -> &[u8] {
                &self.data[..]
            }

            #[inline]
            fn raw_data_mut(&mut self) -> &mut [u8] {
                &mut self.data[..]
            }
        }
    };
}

pixel_block_data!(PixelBlockData4, 2, 8);
pixel_block_data!(PixelBlockData8, 3, 32);
pixel_block_data!(PixelBlockData16, 4, 128);
pixel_block_data!(PixelBlockData32, 5, 512);
pixel_block_data!(PixelBlockData64, 6, 2048);

#[derive(Default, Clone)]
pub struct PixelBlock<Data: PixelBlockData> {
    data: Data,
}

impl<Data: PixelBlockData> PixelBlock<Data> {
    pub const fn size_bits() -> usize {
        Data::SIZE_BITS
    }

    pub const fn size() -> usize {
        1 << Data::SIZE_BITS
    }

    pub const fn num_pixels() -> usize {
        1 << (Data::SIZE_BITS * 2)
    }

    pub fn split_coord(coord: usize) -> (usize, usize) {
        (coord / Self::size(), coord % Self::size())
    }

    fn raw_data_offset(x: usize, y: usize) -> usize {
        let x_offset = y << (Data::SIZE_BITS - 1); // y * SIZE
        let half_x = x >> 1;
        x_offset + half_x
    }

    pub fn get_raw_pixel(&self, x: usize, y: usize) -> u8 {
        let data_offset = Self::raw_data_offset(x, y);
        let double_pixel = self.data.raw_data()[data_offset];
        if x & 1 == 0 {
            // first pixel, x % 2 == 0
            get_first_pixel(double_pixel)
        } else {
            //second, x % 2 == 1
            get_second_pixel(double_pixel)
        }
    }

    pub fn set_raw_pixel(&mut self, x: usize, y: usize, color: u8) {
        let data_offset = Self::raw_data_offset(x, y);
        let double_pixel = &mut self.data.raw_data_mut()[data_offset];
        if x & 1 == 0 {
            set_first_pixel(double_pixel, color);
        } else {
            set_second_pixel(double_pixel, color);
        }
    }

    pub fn to_managed_buffer<Api: ManagedTypeApi>(&self) -> ManagedBuffer<Api> {
        self.data.raw_data().into()
    }

    pub fn from_managed_buffer<Api: ManagedTypeApi>(buffer: &ManagedBuffer<Api>) -> Self {
        let mut block = Self::default();
        let _ = buffer.load_slice(0, block.data.raw_data_mut());
        block
    }
}

fn get_first_pixel(double_pixel: u8) -> u8 {
    double_pixel >> 4
}

fn get_second_pixel(double_pixel: u8) -> u8 {
    double_pixel & 0x0f
}

fn set_first_pixel(double_pixel: &mut u8, pixel: u8) {
    *double_pixel &= 0x0f;
    *double_pixel |= pixel << 4;
}

fn set_second_pixel(double_pixel: &mut u8, pixel: u8) {
    *double_pixel &= 0xf0;
    *double_pixel |= pixel;
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_pixel_block_size<Data: PixelBlockData>() {
        assert_eq!(Data::RAW_DATA_LEN, 1 << (Data::SIZE_BITS * 2) - 1);
        assert_eq!(PixelBlock::<Data>::num_pixels(), Data::RAW_DATA_LEN * 2);
    }

    #[test]
    fn test_pixel_block_sizes() {
        test_pixel_block_size::<PixelBlockData4>();
        test_pixel_block_size::<PixelBlockData8>();
        test_pixel_block_size::<PixelBlockData16>();
        test_pixel_block_size::<PixelBlockData32>();
        test_pixel_block_size::<PixelBlockData64>();
    }

    #[test]
    fn test_double_pixel() {
        let mut double_pixel = 0u8;
        for a in 0..16 {
            for b in 0..16 {
                set_first_pixel(&mut double_pixel, a);
                set_second_pixel(&mut double_pixel, b);
                assert_eq!(get_first_pixel(double_pixel), a);
                assert_eq!(get_second_pixel(double_pixel), b);

                set_second_pixel(&mut double_pixel, a);
                set_first_pixel(&mut double_pixel, b);
                assert_eq!(get_first_pixel(double_pixel), b);
                assert_eq!(get_second_pixel(double_pixel), a);
            }
        }
    }

    #[test]
    fn test_read_write() {
        let mut block = PixelBlock::<PixelBlockData8>::default();
        for x in 0..8 {
            for y in 0..8 {
                block.set_raw_pixel(x, y, (x + y % 16) as u8);
            }
        }

        assert_eq!(
            block.data.data,
            [
                1, 35, 69, 103, 18, 52, 86, 120, 35, 69, 103, 137, 52, 86, 120, 154, 69, 103, 137,
                171, 86, 120, 154, 188, 103, 137, 171, 205, 120, 154, 188, 222
            ]
        );

        for x in 0..8 {
            for y in 0..8 {
                assert_eq!(block.get_raw_pixel(x, y), (x + y % 16) as u8);
            }
        }
    }
}
