#![no_std]
#![no_main]

// pick a panicking behavior
use panic_halt as _; // you can put a breakpoint on `rust_begin_unwind` to catch panics
// use panic_abort as _; // requires nightly
// use panic_itm as _; // logs messages over ITM; requires ITM support
// use panic_semihosting as _; // logs messages to the host stderr; requires a debugger

use core::fmt::Write;

use cortex_m::peripheral::syst::SystClkSource;
use cortex_m_rt::{entry, exception};
use cortex_m_semihosting::{
    debug,
    hio::{self, HStdout}
};

#[repr(C)]
struct SysTick {
    pub csr: u32,
    pub rvr: u32,
    pub cvr: u32,
    pub calib: u32
}

#[entry]
fn main() -> ! {
    let systick = unsafe { &mut *(0xE000_E010 as *mut SysTick) };
    let time =  unsafe { core::ptr::read_volatile(&mut systick.cvr)};
    let mut stdout = hio::hstdout().unwrap();

    writeln!(stdout, "time: {}", time).unwrap();

    loop {}
}