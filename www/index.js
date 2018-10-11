import { Image, Pixel } from "image";
import { memory } from "image/image_bg";

const image = Image.new();
const width = image.width();
const height = image.height();

const pixelsPtr = image.pixels();
const pixels = new Uint8Array(memory.buffer, pixelsPtr, width * height);

console.log(pixels);

const blob = new Blob([pixels], {type: 'image/jpeg'});
const blobURL = URL.createObjectURL(blob);
const img = document.createElement('img');
img.src = blobURL;
document.body.appendChild(img)