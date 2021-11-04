extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;
// use wasm_bindgen::JsCast;
// use web_sys::Window;

#[wasm_bindgen]
extern{
    pub fn alert(s: &str);
}

#[wasm_bindgen]
pub fn output(s: &str){
    alert(s);
}

#[wasm_bindgen]
pub struct Hoge;

#[wasm_bindgen]
impl Hoge{
    fn new(){
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        
    }
}

#[wasm_bindgen]
pub fn alert_window_name(){
    let window = web_sys::window().unwrap();
    alert(&window.name().unwrap());
}
