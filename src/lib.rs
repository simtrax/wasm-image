extern crate cfg_if;
extern crate wasm_bindgen;
extern crate web_sys;
extern crate geojson;

mod utils;

use wasm_bindgen::prelude::*;
use web_sys::console;
use geojson::GeoJson;

// #[wasm_bindgen]
pub struct Pixel {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

#[wasm_bindgen]
pub struct Image {
    width: u32,
    height: u32,
    pixels: Vec<Pixel>,
}

/// Public methods, exported to JavaScript.
#[wasm_bindgen]
impl Image {

    pub fn parse_geometry(geojson_str: String) {
        
        // console::log_1(&geojson_str.into());

        let geojson = geojson_str.parse::<GeoJson>().unwrap();

    }

    pub fn new() -> Image {
        let width = 500;
        let height = 500;

        let pixels = (0..width * height)
            .map(|i| {
                if i % 2 == 0 || i % 7 == 0 {
                    Pixel {
                        r: 0,
                        g: 0,
                        b: (i * width / height * 255) as u8,
                        a: 255
                    }
                } else {
                    Pixel {
                        r: (i * width / height * 255) as u8,
                        g: 255,
                        b: 255,
                        a: 255
                    }
                }
            })
            .collect();

        Image {
            width,
            height,
            pixels,
        }
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn pixels(&self) -> *const Pixel {
        self.pixels.as_ptr()
    }
}
