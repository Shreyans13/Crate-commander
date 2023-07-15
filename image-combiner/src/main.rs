mod args;

use ::image::GenericImageView;
use args::Args;

use crate::image_util::{
    combine_images, find_image_from_path, standardise_size, FloatingImage, ImageDataErrors,
};

mod image_util;

fn main() -> Result<(), ImageDataErrors> {
    let args = Args::new();
    println!("User Input {:?}", args);

    let (image_1, image1_format) = find_image_from_path(args.image1);
    let (image_2, image2_format) = find_image_from_path(args.image2);

    println!("Image 1 {:?}", image_1);
    println!("Image 2 {:?}", image_2);

    if image1_format != image2_format {
        return Err(ImageDataErrors::DifferentImageFormats);
    }

    let (image_1, image_2) = standardise_size(image_1, image_2);
    let mut final_image = FloatingImage::new(image_1.width(), image_2.height(), args.output);
    let combined_data = combine_images(image_1, image_2);

    final_image.set_data(combined_data)?;

    image::save_buffer_with_format(
        final_image.name,
        &final_image.data,
        final_image.width,
        final_image.height,
        image::ColorType::Rgba8,
        image1_format,
    )
    .unwrap();
    Ok(())
}
