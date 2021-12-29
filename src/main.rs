include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

fn main() {
    println!("Hello, world!");

    generated_fn(32);
}
