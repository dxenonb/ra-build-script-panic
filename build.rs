use std::path::PathBuf;
use std::{fs, env};

fn main() {
    // uncomment this line to see a failure
    // panic!("haha lol im a terrible build script");

    let mut out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    out_path.push("bindings.rs");
    fs::write(out_path, "pub fn generated_fn(arg: i32) {}")
        .unwrap();
}
