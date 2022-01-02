use globset::Glob;
use image::{DynamicImage, GenericImage, ImageBuffer, Rgba};
use std::{fs, path::PathBuf};
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "basic")]
struct Opt {
    /// Input folder
    #[structopt(short, long, parse(from_os_str))]
    input: PathBuf,

    /// Output file
    #[structopt(short, long, parse(from_os_str), default_value = "cover-mosaic.png")]
    output: PathBuf,

    /// The max size
    #[structopt(short = "s", long, default_value = "1000")]
    size: u32,
}

pub struct Mosaic {
    images: Vec<DynamicImage>,
    size: u32,
}

impl Mosaic {
    pub fn new(images: Vec<DynamicImage>, size: u32) -> Self {
        Mosaic { images, size }
    }

    fn get_per_line(&self) -> u32 {
        let length: u32 = self.images.len() as u32;
        let mut per_line = 2;

        if length % 3 == 0 {
            per_line = length / 3;
        } else if length % 2 == 0 {
            per_line = length / 2;
        }

        per_line
    }

    fn get_per_row(&self) -> u32 {
        let length: u32 = self.images.len() as u32;
        let per_row;
        let per_line = self.get_per_line();

        per_row = length / per_line;

        per_row
    }

    pub fn concat(&self) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
        let per_line = self.get_per_line();
        let per_row = self.get_per_row();

        let single_image_width = self.size / per_line;
        let single_image_height = single_image_width;

        let img_width_out = single_image_width * per_line;
        let img_height_out = single_image_height * per_row;

        let mut imgbuf = image::ImageBuffer::new(img_width_out, img_height_out);

        let mut current_line = 1;
        let mut current_row = 1;

        println!("per row: {}, per line : {}", per_row, per_line);

        // Copy each input image at the correct location in the output image.
        for img in &self.images {
            println!(
                "current row: {}, current line: {}",
                current_row, current_line
            );
            if current_row > per_line {
                current_line += 1;
                current_row = 1;
            }

            let scaled = img.thumbnail(single_image_width, single_image_height);
            imgbuf
                .copy_from(
                    &scaled,
                    (current_row - 1) * single_image_width,
                    (current_line - 1) * single_image_height,
                )
                .unwrap();

            current_row += 1;
        }

        imgbuf
    }
}

fn main() {
    let opt = Opt::from_args();

    // Get the size of the resulting image from the command line
    let size: u32 = opt.size;

    let input: PathBuf = opt.input;
    let output: PathBuf = opt.output;

    let glob = Glob::new("*.{jpeg,jpg,png}").unwrap().compile_matcher();

    let images: Vec<DynamicImage> = fs::read_dir(input)
        .unwrap()
        .into_iter()
        .filter_map(Result::ok)
        .filter(|entry| glob.is_match(entry.file_name()))
        .map(|entry| image::open(entry.path()).unwrap())
        .collect();

    if images.len() % 2 != 0 && images.len() % 3 != 0 {
        panic!("The input folder must have a multiple of 2 or 3 images.");
    }

    let m = Mosaic::new(images, size);
    m.concat().save(output).unwrap();
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;

    const BASE_PATH: [&str; 2] = [".", "tests"];
    const IMAGE_DIR: &str = "images";

    fn build_images(length: u32) -> Vec<DynamicImage> {
        let mut path: PathBuf = BASE_PATH.iter().collect();
        path.push(IMAGE_DIR);

        let mut images = vec![];
        let mut images_length: u32 = 0;

        while images_length < length {
            let mut img_path = path.clone();
            img_path.push("140x100.png");
            images.push(image::open(img_path).unwrap());
            images_length = images.len() as u32;
        }

        images
    }

    #[test]
    fn get_correct_rows_and_lines_for_4_images() {
        let size: u32 = 1000;

        let images = build_images(4);
        let m = Mosaic::new(images, size);

        assert_eq!(m.get_per_line(), 2);
        assert_eq!(m.get_per_row(), 2);
    }

    #[test]
    fn get_correct_rows_and_lines_for_6_images() {
        let size: u32 = 1000;

        let images = build_images(6);
        let m = Mosaic::new(images, size);

        assert_eq!(m.get_per_line(), 2);
        assert_eq!(m.get_per_row(), 3);
    }

    #[test]
    fn get_correct_rows_and_lines_for_8_images() {
        let size: u32 = 1000;

        let images = build_images(8);
        let m = Mosaic::new(images, size);

        assert_eq!(m.get_per_line(), 4);
        assert_eq!(m.get_per_row(), 2);
    }

    #[test]
    fn get_correct_rows_and_lines_for_9_images() {
        let size: u32 = 1000;

        let images = build_images(9);
        let m = Mosaic::new(images, size);

        assert_eq!(m.get_per_line(), 3);
        assert_eq!(m.get_per_row(), 3);
    }

    #[test]
    fn get_correct_rows_and_lines_for_10_images() {
        let size: u32 = 1000;

        let images = build_images(10);
        let m = Mosaic::new(images, size);

        assert_eq!(m.get_per_line(), 5);
        assert_eq!(m.get_per_row(), 2);
    }

    #[test]
    fn get_correct_rows_and_lines_for_12_images() {
        let size: u32 = 1000;

        let images = build_images(12);
        let m = Mosaic::new(images, size);

        assert_eq!(m.get_per_line(), 4);
        assert_eq!(m.get_per_row(), 3);
    }
}
