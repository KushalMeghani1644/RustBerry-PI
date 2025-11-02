#![no_std]
#![no_main]

use core::fmt::{self, Write};

#[unsafe(no_mangle)]
pub extern "C" fn rust_main() -> ! {
    uart::init();
    let mut uart = uart::Uart;
    let _ = writeln!(uart, "[BOOT] RustBerry PI starting...");
    let _ = writeln!(uart, "[UART] Hello from RustBerry PI");

    loop {
        // spin forever
    }
}

/// PANIC HANDLER
use core::panic::PanicInfo;
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    let mut uart = uart::Uart;
    let _ = writeln!(uart, "PANIC: {}", info);
    loop {}
}

mod uart {
    use core::fmt::{self, Write};
    const UART0: *mut u8 = 0x1000_0000 as *mut u8;

    pub fn init() {}

    pub struct Uart;

    impl Write for Uart {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            for &b in s.as_bytes() {
                unsafe {
                    core::ptr::write_volatile(UART0, b);
                }
            }
            Ok(())
        }
    }
}
