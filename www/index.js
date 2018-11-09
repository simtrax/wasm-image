// import { image } from "image";
// import * from "image";
// import { memory } from "image/image_bg";

// const image = Image.new();
// const width = image.width();
// const height = image.height();

// const canvas = document.getElementById('canvas');
// const ctx = canvas.getContext('2d');

// canvas.width  = width;
// canvas.height = height;

// const pixelsPtr = image.pixels();
// const pixels = new Uint8ClampedArray(memory.buffer, pixelsPtr, width * height * 4);

// // console.log(pixels);

// const imageData = new ImageData(pixels, width, height);

// ctx.putImageData(imageData, 0, 0);

import { draw } from "image";

const canvas = document.getElementById('canvas');
const ctx = canvas.getContext('2d');

draw(ctx, 1000, 1000, 2900, 0.6, 0.7)

console.log("draw")
