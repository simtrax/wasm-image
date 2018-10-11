extern crate cfg_if;
extern crate wasm_bindgen;

mod utils;

use wasm_bindgen::prelude::*;

// #[wasm_bindgen]
pub struct Pixel {
    r: u32,
    g: u32,
    b: u32,
}

#[wasm_bindgen]
pub struct Image {
    width: u32,
    height: u32,
    pixels: Vec<u8>,
}

// Private methods
impl Image {

    // fn get_index(&self, row: u32, column: u32) -> usize {
    //     (row * self.width + column) as usize
    // }
    
    // fn live_neighbor_count(&self, row: u32, column: u32) -> u8 {
    //     let mut count = 0;
    //     for delta_row in [self.height - 1, 0, 1].iter().cloned() {
    //         for delta_col in [self.width - 1, 0, 1].iter().cloned() {
    //             if delta_row == 0 && delta_col == 0 {
    //                 continue;
    //             }

    //             let neighbor_row = (row + delta_row) % self.height;
    //             let neighbor_col = (column + delta_col) % self.width;
    //             let idx = self.get_index(neighbor_row, neighbor_col);
    //             count += self.cells[idx] as u8;
    //         }
    //     }
    //     count
    // }

}

/// Public methods, exported to JavaScript.
#[wasm_bindgen]
impl Image {
    // pub fn tick(&mut self) {
    //     // let mut next = self.cells.clone();

    //     for row in 0..self.height {
    //         for col in 0..self.width {
                
    //         }
    //     }

    // }

    pub fn new() -> Image {
        let width = 20;
        let height = 20;

        // let mut pixels: Vec<Pixel> = Vec::new();
        let mut pixels = Vec::new();

        for index in (0..width * height) {
            // let pixel: Pixel = if index % 2 == 0 || index % 7 == 0 {
            //     Pixel {
            //         r: 0,
            //         g: 0,
            //         b: 0
            //     };
            // } else {
            //     Pixel {
            //         r: 255,
            //         g: 255,
            //         b: 255
            //     };
            // };

            // pixels.push(Pixel {
            //     r: 137,
            //     g: 80,
            //     b: 78
            // });
            pixels.push(137);
            pixels.push(80);
            pixels.push(78);
            pixels.push(255);
        }

        // let pixels = (0..width * height)
        //     .map(|i| {
        //         if i % 2 == 0 || i % 7 == 0 {
        //             Pixel {
        //                 r: 0,
        //                 g: 0,
        //                 b: 0
        //             };
        //         } else {
        //             Pixel {
        //                 r: 255,
        //                 g: 255,
        //                 b: 255
        //             };
        //         }
        //     })
        //     .collect();

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

    // pub fn pixels(&self) -> *const Pixel {
    //     self.pixels.as_ptr()
    // }
    pub fn pixels(&self) -> *const u8 {
        self.pixels.as_ptr()
    }
}

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, I'm an application that can create images!");
}
