#![no_std]
pub extern crate alloc;

#[path = "impls/mod.rs"]
mod impls;
pub use impls::implementation::*;

pub struct HashIndexedArray {
    pub(crate) indices_array: [[u8; 4]; 64],
}

pub trait Hashing {
    fn update(&mut self, pixel_feed: &[[u8; 4]]);
    fn fetch(&mut self, hash: u8) -> [u8; 4];
    fn push(&mut self, pixel: [u8; 4]) -> ([u8; 4], u8);
    fn new() -> Self;
}

pub mod common {
    use core::{array::IntoIter, iter::Iterator};

    use image::{DynamicImage, GenericImageView};

    pub const MAGIC_QOIF: [u8; 4] = *b"qoif";
    pub const QOI_OP_RGBA: u8 = 0xff_u8;
    pub const QOI_OP_RGB: u8 = 0xfe_u8;
    pub const QOI_OP_INDEX: u8 = 0b00_000000_u8;
    pub const QOI_OP_DIFF: u8 = 0b01_000000_u8;
    pub const QOI_OP_LUMA: u8 = 0b10_000000_u8;
    pub const QOI_OP_RUN: u8 = 0b11_000000_u8;
    pub const END_8: [u8; 8] = [0, 0, 0, 0, 0, 0, 0, 1];

    #[derive(Clone, Copy)]
    pub struct QOIHeader {
        pub width: u32,
        pub height: u32,
        pub has_alpha: bool,
        pub linear_rgb: bool,
    }

    impl From<&DynamicImage> for QOIHeader {
        fn from(img: &DynamicImage) -> Self {
            let dims = img.dimensions();
            Self {
                width: dims.0,
                height: dims.1,
                has_alpha: img.color().has_alpha(),
                linear_rgb: false, // TODO detect this somehow
            }
        }
    }

    impl From<[u8; 14]> for QOIHeader {
        fn from(bytes: [u8; 14]) -> Self {
            assert_eq!(
                bytes[0..4],
                MAGIC_QOIF,
                "Data is not a QOI image (magic bytes \"qoif\" not found)"
            );
            Self {
                width: u32::from_be_bytes(bytes[4..8].try_into().unwrap()),
                height: u32::from_be_bytes(bytes[8..12].try_into().unwrap()),
                has_alpha: bytes[12] - 3 == 1,
                linear_rgb: bytes[13] > 0,
            }
        }
    }

    impl QOIHeader {
        pub fn to_bytes(&self) -> impl Iterator<Item = u8> {
            let w_bytes: IntoIter<u8, 4> = self.width.to_be_bytes().into_iter();
            let h_bytes: IntoIter<u8, 4> = self.height.to_be_bytes().into_iter();
            let other: [u8; 2] = [self.has_alpha as u8 + 3u8, self.linear_rgb as u8];
            w_bytes.chain(h_bytes).chain(other.into_iter())
        }

        pub fn image_size(&self) -> usize {
            self.width as usize * self.height as usize
        }
    }
}
