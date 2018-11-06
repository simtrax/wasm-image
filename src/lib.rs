extern crate cfg_if;
extern crate wasm_bindgen;
extern crate web_sys;

mod utils;

use wasm_bindgen::prelude::*;
use wasm_bindgen::Clamped;
use web_sys::{CanvasRenderingContext2d, ImageData, console};

#[wasm_bindgen]
pub fn draw(ctx: &CanvasRenderingContext2d,
    width: u32,
    height: u32,
    magnification: u32,
    pan_x: f32,
    pan_y: f32
) -> Result<(), JsValue> {
    let mut data = Vec::new();

    // console::log_1(&"Calculating Mandelbrot".into());
    for x in 0..width {
        for y in 0..height {

            let belongs_to_set = check_if_belongs_to_mandelbrot(
                x as f32 / magnification as f32 - pan_x, 
                y as f32 / magnification as f32 - pan_y
            );

            if belongs_to_set {
                data.push(255);
                data.push(255);
                data.push(255);
                data.push(255);
            } else {
                data.push(0);
                data.push(0);
                data.push(0);
                data.push(255);
            }
        }
    }

    let data = ImageData::new_with_u8_clamped_array_and_sh(Clamped(&mut data), width, height)?;
    ctx.put_image_data(&data, 0.0, 0.0)
}

fn check_if_belongs_to_mandelbrot(x: f32, y: f32) -> bool {
    let mut real_component_of_result = x;
    let mut im_component_of_result = y;

    for _i in 0..100 {
        let temp_real_component = real_component_of_result * real_component_of_result
                                - im_component_of_result * im_component_of_result
                                + x;

        let temp_im_component = 2.0 * real_component_of_result * im_component_of_result
                                + y;

        real_component_of_result = temp_real_component;
        im_component_of_result = temp_im_component;
    }

    let mut answer = false;
    if real_component_of_result * im_component_of_result < 5.0 {
        answer = true;
    }

    answer
}