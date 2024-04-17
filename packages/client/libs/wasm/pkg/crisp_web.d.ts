/* tslint:disable */
/* eslint-disable */
/**
*/
export class Encrypt {
  free(): void;
/**
*/
  constructor();
/**
* @param {bigint} vote
* @param {Uint8Array} public_key
* @returns {Uint8Array}
*/
  encrypt_vote(vote: bigint, public_key: Uint8Array): Uint8Array;
/**
*/
  static test(): void;
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly __wbg_encrypt_free: (a: number) => void;
  readonly encrypt_new: () => number;
  readonly encrypt_encrypt_vote: (a: number, b: number, c: number, d: number, e: number) => void;
  readonly encrypt_test: () => void;
  readonly __wbindgen_add_to_stack_pointer: (a: number) => number;
  readonly __wbindgen_malloc: (a: number, b: number) => number;
  readonly __wbindgen_free: (a: number, b: number, c: number) => void;
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
export default function __wbg_init (module_or_path?: InitInput | Promise<InitInput>): Promise<InitOutput>;
