const wasmInstance = new WebAssembly.Instance(wasmModule, {});
const { main } = wasmInstance.exports;
console.log(main());