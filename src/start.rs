#[unsafe(no_mangle)]
extern "C" fn _start() -> ! {
    unsafe extern "Rust" {
        fn main();
    }

    unsafe {
        main();
    }

    crate::raw::_exit(0)
}
