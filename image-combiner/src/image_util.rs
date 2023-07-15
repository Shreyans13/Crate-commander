use std::{fs::File, io::BufReader, vec};

use image::{
    imageops::FilterType::Triangle, io::Reader, DynamicImage, GenericImageView, ImageFormat,
};

#[derive(Debug)]
pub enum ImageDataErrors {
    DifferentImageFormats,
    BufferTooSmall,
}

pub fn find_image_from_path(path: String) -> (DynamicImage, ImageFormat) {
    let image_reader: Reader<BufReader<File>> = Reader::open(path).unwrap();
    let image_format: ImageFormat = image_reader.format().unwrap();
    let image: Result<DynamicImage, image::ImageError> = image_reader.decode();
    match image {
        Ok(image) => (image, image_format),
        Err(e) => panic!("Image Error {}", e),
    }
}

fn get_smallest_dimensions(dim_1: (u32, u32), dim_2: (u32, u32)) -> (u32, u32) {
    let pix_1 = dim_1.0 * dim_1.1;
    let pix_2 = dim_2.0 * dim_2.1;
    return if pix_1 > pix_2 { dim_1 } else { dim_2 };
}

pub fn standardise_size(
    image_1: DynamicImage,
    image_2: DynamicImage,
) -> (DynamicImage, DynamicImage) {
    let (width, height) = get_smallest_dimensions(image_1.dimensions(), image_2.dimensions());
    println!("width: {} height: {}", width, height);

    if (width, height) == image_1.dimensions() {
        (image_1, image_2.resize(width, height, Triangle))
    } else {
        (image_1.resize(width, height, Triangle), image_2)
    }
}

pub struct FloatingImage {
    pub width: u32,
    pub height: u32,
    pub data: Vec<u8>,
    pub name: String,
}

impl FloatingImage {
    pub fn new(width: u32, height: u32, name: String) -> Self {
        // let buffer_capacity = 3_655_744;
        FloatingImage {
            width,
            height,
            data: Vec::with_capacity(3_655_744),
            name,
        }
    }
    pub fn set_data(&mut self, data: Vec<u8>) -> Result<(), ImageDataErrors> {
        if data.len() > self.data.capacity() {
            return Err(ImageDataErrors::BufferTooSmall);
        }
        self.data = data;
        Ok(())
    }
}

fn alternate_pixels(vec_1: Vec<u8>, vec_2: Vec<u8>) -> Vec<u8> {
    let mut combined_data = vec![0u8; vec_1.len()];

    let mut i = 0;

    while i < vec_1.len() {
        if i % 8 == 0 {
            combined_data.splice(i..=i + 3, set_rgba(&vec_1, i, i + 3));
        } else {
            combined_data.splice(i..=i + 3, set_rgba(&vec_2, i, i + 3));
        }
        i += 4;
    }

    combined_data
}

pub fn combine_images(image_1: DynamicImage, image_2: DynamicImage) -> Vec<u8> {
    alternate_pixels(
        image_1.into_rgb8().into_vec(),
        image_2.into_rgb8().into_vec(),
    )
}

fn set_rgba(vec: &Vec<u8>, start: usize, end: usize) -> Vec<u8> {
    let mut rgba = Vec::new();
    for i in start..=end {
        rgba.push(match vec.get(i) {
            Some(d) => *d,
            None => panic!("Index out of bounds "),
        })
    }
    rgba
}
