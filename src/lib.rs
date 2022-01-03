use image::{DynamicImage, GenericImage, ImageBuffer, Rgba};

pub struct Mosaic {
    images: Vec<DynamicImage>,
    size: u32,
}

impl Mosaic {
    pub fn new(images: Vec<DynamicImage>, size: u32) -> Self {
        Mosaic { images, size }
    }

    pub fn get_per_line(&self) -> u32 {
        let length: u32 = self.images.len() as u32;
        let mut per_line = 2;

        if length % 3 == 0 {
            per_line = length / 3;
        } else if length % 2 == 0 {
            per_line = length / 2;
        }

        per_line
    }

    pub fn get_per_row(&self) -> u32 {
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
