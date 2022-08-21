use crate::error::{AppError, Result};
use exif::{In, Tag, Reader};
use image::{ImageFormat, DynamicImage};
use std::fs::File;
use std::io::Cursor;
use uuid::Uuid;


pub fn img_upload(img_bytes: Vec<u8>, save_path: &'static str) -> (Result<()>, String) {
    let orientation = get_orientation(&img_bytes);
    println!("orientation: {}", orientation);
    let (format, ext) = get_format_and_ext(&img_bytes).unwrap();
    let file_name = create_file_name(&ext);

    match image::load_from_memory_with_format(&img_bytes, format) {
        Ok(img) => {
            let mut new_img = img.thumbnail(300, 300);
            new_img = img_rotate(new_img, orientation);
            let mut output = File::create(format!("{}/{}", save_path, file_name)).unwrap();
            new_img.write_to(&mut output, format).unwrap();
        },
        Err(_) => return (Err(AppError::InvalidFileFormat), String::from("Invalid file format")),
    };

    (Ok(()), file_name)
}


pub fn get_orientation(img_bytes: &Vec<u8>) -> u32 {
    let mut buf = Cursor::new(img_bytes);
    let mut orientation = 1;
    if let Ok(exif) = Reader::new().read_from_container(&mut buf) {
        if let Some(o) = exif.get_field(Tag::Orientation, In::PRIMARY) {
            if let Some(v @ 1..=8) = o.value.get_uint(0) {
                orientation = v;
            }
        }
    }
    orientation
}


pub fn get_format_and_ext(img_bytes: &Vec<u8>) -> Result<(ImageFormat, &'static str)> {
    match image::guess_format(img_bytes) {
        Ok(ImageFormat::Png) => return Ok((ImageFormat::Png, "png")),
        Ok(ImageFormat::Jpeg) => return Ok((ImageFormat::Jpeg, "jpeg")),
        Ok(ImageFormat::Gif) => return Ok((ImageFormat::Gif, "gif")),
        _ => return Err(AppError::InvalidFileFormat),
    };
}


pub fn create_file_name(ext: &'static str) -> String {
    format!("{}.{}", Uuid::new_v4().as_hyphenated(), ext)
}


pub fn img_rotate(img: DynamicImage, orientation: u32) -> DynamicImage {
    let img = match orientation {
        1 => img,
        2 => img.flipv(),
        3 => img.rotate180(),
        4 => img.fliph(),
        5 => img.flipv().rotate270(),
        6 => img.rotate90(),
        7 => img.fliph().rotate90(),
        8 => img.rotate270(),
        _ => img,
    };
    img
}