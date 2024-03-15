// import init, { fib } from "./pkg/fib.js";

// init().then(() => {
//     let result = fib(20);
//     console.log(result);
//     // Set the result onto the body
//     document.body.textContent = `Hello rust fib = ${result}`;
// });

//另一種寫法
//Import our outputted wasm ES6 module
// Which, export default's, an initialization function
import init from "./pkg/fib.js";

const runWasm = async () => {
    // Instantiate our wasm module
    const fiblib = await init("./pkg/fib_bg.wasm");

    // Call the fib function export from wasm, save the result
    const result = fiblib.fib(20);

    // Set the result onto the body
    document.body.textContent = `runWasm rust fib = ${result}`;
};
runWasm();