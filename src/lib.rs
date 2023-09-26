mod image_array;
mod utils;

use image::*;
use image_array::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

type Uint8ArrayImage = Vec<u8>;

#[wasm_bindgen]
pub fn scale_by_width(_array: &[u8], _width: u32) -> Uint8ArrayImage {
    let image = load_image_from_array(_array);

    get_image_as_array(image.resize(
        _width,
        (_width as f32 / image.width() as f32 * image.height() as f32) as u32,
        imageops::FilterType::Nearest,
    ))
}

#[wasm_bindgen]
pub fn scale_by_height(_array: &[u8], _height: u32) -> Uint8ArrayImage {
    let image = load_image_from_array(_array);

    get_image_as_array(image.resize(
        (_height as f32 / image.height() as f32 * image.width() as f32) as u32,
        _height,
        imageops::FilterType::Nearest,
    ))
}

#[wasm_bindgen]
pub fn resize(_array: &[u8], _width: u32, _height: u32) -> Uint8ArrayImage {
    let image = load_image_from_array(_array);

    get_image_as_array(image.resize_to_fill(_width, _height, imageops::FilterType::Nearest))
}
