use core::fmt::{self, Write};
use core::panic::PanicInfo;

struct PanicWriter;

impl Write for PanicWriter {
    fn write_str(&mut self, message: &str) -> fmt::Result {
        crate::raw::_write(2, message.as_bytes())
            .map(|_| ())
            .map_err(|_| fmt::Error)
    }
}

#[panic_handler]
fn panic(info: &PanicInfo<'_>) -> ! {
    let _ = core::writeln!(PanicWriter, "panic: {info}");
    crate::raw::_exit(1)
}
