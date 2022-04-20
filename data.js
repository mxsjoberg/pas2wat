function consoleLogString(offset, length) {
  var bytes = new Uint8Array(memory.buffer, offset, length);
  var string = new TextDecoder('utf8').decode(bytes);
  console.log(string);
}

function consoleLog(value) {
  console.log(value);
}

var memory = new WebAssembly.Memory({ initial: 1 });
const wasmInstance = new WebAssembly.Instance(wasmModule, {console: { log: consoleLog, logString: consoleLogString }, js: { mem: memory } });
const { test } = wasmInstance.exports;
test();
