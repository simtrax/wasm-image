/* tslint:disable */
import * as wasm from './image_bg';

let cachedTextEncoder = new TextEncoder('utf-8');

let cachegetUint8Memory = null;
function getUint8Memory() {
    if (cachegetUint8Memory === null || cachegetUint8Memory.buffer !== wasm.memory.buffer) {
        cachegetUint8Memory = new Uint8Array(wasm.memory.buffer);
    }
    return cachegetUint8Memory;
}

function passStringToWasm(arg) {

    const buf = cachedTextEncoder.encode(arg);
    const ptr = wasm.__wbindgen_malloc(buf.length);
    getUint8Memory().set(buf, ptr);
    return [ptr, buf.length];
}

function freeImage(ptr) {

    wasm.__wbg_image_free(ptr);
}
/**
*/
export class Image {

    static __wrap(ptr) {
        const obj = Object.create(Image.prototype);
        obj.ptr = ptr;

        return obj;
    }

    free() {
        const ptr = this.ptr;
        this.ptr = 0;
        freeImage(ptr);
    }

    /**
    * @param {string} arg0
    * @returns {void}
    */
    static parse_geometry(arg0) {
        const [ptr0, len0] = passStringToWasm(arg0);
        return wasm.image_parse_geometry(ptr0, len0);
    }
    /**
    * @returns {Image}
    */
    static new() {
        return Image.__wrap(wasm.image_new());
    }
    /**
    * @returns {number}
    */
    width() {
        return wasm.image_width(this.ptr);
    }
    /**
    * @returns {number}
    */
    height() {
        return wasm.image_height(this.ptr);
    }
    /**
    * @returns {number}
    */
    pixels() {
        return wasm.image_pixels(this.ptr);
    }
}

let cachedTextDecoder = new TextDecoder('utf-8');

function getStringFromWasm(ptr, len) {
    return cachedTextDecoder.decode(getUint8Memory().subarray(ptr, ptr + len));
}

export function __wbindgen_throw(ptr, len) {
    throw new Error(getStringFromWasm(ptr, len));
}

