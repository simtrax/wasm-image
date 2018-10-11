/* tslint:disable */
import * as wasm from './image_bg';

let cachedTextDecoder = new TextDecoder('utf-8');

let cachegetUint8Memory = null;
function getUint8Memory() {
    if (cachegetUint8Memory === null || cachegetUint8Memory.buffer !== wasm.memory.buffer) {
        cachegetUint8Memory = new Uint8Array(wasm.memory.buffer);
    }
    return cachegetUint8Memory;
}

function getStringFromWasm(ptr, len) {
    return cachedTextDecoder.decode(getUint8Memory().subarray(ptr, ptr + len));
}

export function __wbg_alert_1bd7c6874b324324(arg0, arg1) {
    let varg0 = getStringFromWasm(arg0, arg1);
    alert(varg0);
}
/**
* @returns {void}
*/
export function greet() {
    return wasm.greet();
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

export function __wbindgen_throw(ptr, len) {
    throw new Error(getStringFromWasm(ptr, len));
}

