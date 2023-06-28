#![allow(non_snake_case)]
#![allow(dead_code)]

mod window;
mod engine;

fn main() {
    println!("Booting v{};", env!("CARGO_PKG_VERSION"));
    engine::run(320, 240, 2, 10);
}
