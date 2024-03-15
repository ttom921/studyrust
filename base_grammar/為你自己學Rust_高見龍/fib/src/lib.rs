//參考：https://wasmbyexample.dev/examples/hello-world/hello-world.rust.en-us.html
// The wasm-pack uses wasm-bindgen to build and generate JavaScript binding file.
// Import the wasm-bindgen crate.
use wasm_bindgen::prelude::*;

// Our function
// wasm-pack requires "exported" functions
// to include #[wasm_bindgen]
#[wasm_bindgen]
pub fn fib(n: i32) -> u64 {
    if n <= 0 {
        panic!("不能小於或等於零");
    }

    match n {
        1 => 1,
        2 => 1,
        3 => 2,
        _ => fib(n - 1) + fib(n - 2),
    }
}

#[test]
fn test_fib() {
    //1,1,2,3,5,8
    let result = fib(5);
    assert_eq!(result, 5);
}
