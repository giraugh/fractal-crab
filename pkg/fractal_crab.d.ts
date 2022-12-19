/* tslint:disable */
/* eslint-disable */
/**
*/
export class FractalBuilder {
  free(): void;
}
/**
*/
export class Image {
  free(): void;
/**
* Generate a new image with random black/white pixels
* @param {number} width
* @param {number} height
* @returns {Image}
*/
  static random(width: number, height: number): Image;
/**
* @param {number} width
* @param {number} height
* @param {Float64Array} real_range
* @param {Float64Array} im_range
* @returns {Image}
*/
  static fractal(width: number, height: number, real_range: Float64Array, im_range: Float64Array): Image;
/**
* @returns {Uint8Array}
*/
  rgba_array(): Uint8Array;
/**
* @returns {ImageData}
*/
  image_data(): ImageData;
/**
* Render the image to a canvas
* @param {string} canvas_id
*/
  render_to_canvas(canvas_id: string): void;
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly __wbg_image_free: (a: number) => void;
  readonly image_random: (a: number, b: number) => number;
  readonly image_fractal: (a: number, b: number, c: number, d: number, e: number, f: number) => number;
  readonly image_rgba_array: (a: number, b: number) => void;
  readonly image_image_data: (a: number) => number;
  readonly image_render_to_canvas: (a: number, b: number, c: number) => void;
  readonly __wbg_fractalbuilder_free: (a: number) => void;
  readonly __wbindgen_malloc: (a: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number) => number;
  readonly __wbindgen_add_to_stack_pointer: (a: number) => number;
  readonly __wbindgen_free: (a: number, b: number) => void;
  readonly __wbindgen_exn_store: (a: number) => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;
/**
* Instantiates the given `module`, which can either be bytes or
* a precompiled `WebAssembly.Module`.
*
* @param {SyncInitInput} module
*
* @returns {InitOutput}
*/
export function initSync(module: SyncInitInput): InitOutput;

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {InitInput | Promise<InitInput>} module_or_path
*
* @returns {Promise<InitOutput>}
*/
export default function init (module_or_path?: InitInput | Promise<InitInput>): Promise<InitOutput>;
