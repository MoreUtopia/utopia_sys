#![no_std]
#![no_main]

use utopia_sys::println;

#[unsafe(no_mangle)]
extern "Rust" fn main() {
    println!("Hello, world!");
}