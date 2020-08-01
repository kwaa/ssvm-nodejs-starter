use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn sb(s: &str) -> String {
  let l = s.parse::<i32>().unwrap();
  let r = "a".repeat(l as usize - 1) + if l & 1 != 0 { "a" } else { "b" };
  println!("The Rust function sb() received {}", r);
  return r;
}
