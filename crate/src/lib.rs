use wasm_bindgen::prelude::*;

mod utils;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub struct Foo {
    test: String
}

#[wasm_bindgen]
impl Foo {
    pub fn go_riders(&self, x: &str) -> String {
        let mut s = self.test.clone();
        s.push_str(x);
        s
    }
}

#[wasm_bindgen]
pub fn bar(x: &str) -> Foo {
    let foo = Foo { test: "hello world".to_owned() };
    foo
}
