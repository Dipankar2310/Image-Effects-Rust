use wasm_bindgen::prelude::*;
use web_sys::console::log_1 as log;
use base64::{decode,encode};
use image::load_from_memory;
use image::ImageOutputFormat::Png;
#[wasm_bindgen]
pub fn grayscale(encoded_image:&str) -> String{
    log(&"Fun started".into());
   let base64_to_vector = decode(encoded_image);
   let mut decoded_image: Vec<u8> = vec![];

    match base64_to_vector {
        Ok(value) => {
            log(&"Decoding successful".into());
            decoded_image = value;
        }
        Err(e) => {
            log(&"Error while decoding".into());
            log(&format!("Decoding error: {}", e).into());
            return "Failed".into();
        }
    }

    // Load the image from the decoded vector
    let image = load_from_memory(&decoded_image);
    let mut img = match image {
        Ok(value) => value,
        Err(e) => {
            log(&"Error while loading image".into());
            log(&format!("Image loading error: {}", e).into());
            return "Failed".into();
        }
    };

    // Convert the image to grayscale
    let output_img = img.grayscale();

 log(&"Grayscale applied".into());

 let mut buffer = vec![];
 output_img.write_to(&mut buffer, Png).unwrap();
 log(&"New Image written".into());

 let encoded_img= encode(&buffer);
 let dataUrl = format!("data:image/png;base64,{}",encoded_img);

 dataUrl
 


}