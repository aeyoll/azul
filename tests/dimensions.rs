extern crate azul;

use azul::Mosaic;
use image::DynamicImage;
use std::path::PathBuf;

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
