use wasm_bindgen::prelude::*;
#[wasm_bindgen]
pub fn reverb(name: &str){
    print!("{}",name)
}
pub fn main(){
    reverb("hellow")
}