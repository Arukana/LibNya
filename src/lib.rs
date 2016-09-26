#![crate_type = "dylib"]

#[no_mangle]
pub extern "C" fn start() {
  println!("Nya, hello!");
}
