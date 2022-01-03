use globset::Glob;
use image::DynamicImage;
use std::{fs, path::PathBuf};
use structopt::StructOpt;

mod lib;
mod opt;

use crate::lib::Mosaic;
use crate::opt::Opt;

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
