#![no_std]
#![no_main]

// pick a panicking behavior
use panic_halt as _; // you can put a breakpoint on `rust_begin_unwind` to catch panics
// use panic_abort as _; // requires nightly
// use panic_itm as _; // logs messages over ITM; requires ITM support
// use panic_semihosting as _; // logs messages to the host stderr; requires a debugger

use core::fmt::Write;

use cortex_m_rt::entry;
use cortex_m_semihosting::{debug, hio};
use cortex_m::peripheral::{syst, Peripherals};

#[entry]
fn main() -> ! {
    let peripherals = Peripherals::take().unwrap();
    let mut systick = peripherals.SYST;
    systick.set_clock_source(syst::SystClkSource::Core);
    systick.set_reload(1_000);
    systick.clear_current();
    systick.enable_counter();

    while !systick.has_wrapped() {}

    let mut stdout = hio::hstdout().unwrap();
    writeln!(stdout, "clock is done").unwrap();
    debug::exit(debug::EXIT_SUCCESS);

    loop {}
}
