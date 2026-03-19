/* @ts-self-types="./markov_generator.d.ts" */

import * as wasm from "./markov_generator_bg.wasm";
import { __wbg_set_wasm } from "./markov_generator_bg.js";
__wbg_set_wasm(wasm);
wasm.__wbindgen_start();
export {
    greet, spit_out
} from "./markov_generator_bg.js";
