/* this file is generated */
const wasmInstance = new WebAssembly.Instance(wasmModule, {console});
const { test } = wasmInstance.exports;
test();