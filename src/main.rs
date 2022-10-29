mod args;
use args::Args;
use image::{ io::Reader, DynamicImage, ImageFormat};
use std::io::BufReader;

enum ImageDataErrors{
    DifferentImageFormats,
}

fn main() -> Result<(), ImageDataErrors>{
    let args:Args = Args::create_from_arg();
    let(image_1, image_format_1) = find_image_from_path(args.image_1);
    let(image_2, image_format_2) = find_image_from_path(args.image_2);

    if image_1_format != image_2_format {
        return Err(ImageDataErrors::DifferentImageFormats);
    }
    Ok(())
}


fn find_image_from_path(path:String)->(DynamicImage, ImageFormat) {
let image_reader: Reader<BufReader<file>> = Reader::open(path).unwrap();
let image_format: ImageFormat = image_reader.format().unwrap();
let image: DynamicImage = image_reader.decode().unwrap();
}