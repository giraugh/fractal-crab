/* tslint:disable */
/* eslint-disable */
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
* @returns {Image}
*/
  static mandelbrot(width: number, height: number): Image;
/**
* Render the image to a canvas
* @param {string} canvas_id
*/
  render_to_canvas(canvas_id: string): void;
}
/**
*/
export class MandelbrotBuilder {
  free(): void;
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly __wbg_image_free: (a: number) => void;
  readonly image_random: (a: number, b: number) => number;
  readonly image_mandelbrot: (a: number, b: number) => number;
  readonly image_render_to_canvas: (a: number, b: number, c: number) => void;
  readonly __wbg_mandelbrotbuilder_free: (a: number) => void;
  readonly __wbindgen_malloc: (a: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number) => number;
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
