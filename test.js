/* this file is generated */
const wasmInstance = new WebAssembly.Instance(wasmModule, {});
const { test } = wasmInstance.exports;
console.log(test());