#![no_std]
#![no_main]

// pick a panicking behavior
use panic_halt as _; // you can put a breakpoint on `rust_begin_unwind` to catch panics
// use panic_abort as _; // requires nightly
// use panic_itm as _; // logs messages over ITM; requires ITM support
// use panic_semihosting as _; // logs messages to the host stderr; requires a debugger

use core::fmt::Write;

use cortex_m_rt::entry;
use tm4c123x_hal as hal;
use tm4c123x_hal::prelude::*;
use tm4c123x_hal::serial::{NewlineMode, Serial};
use tm4c123x_hal::sysctl;

#[entry]
fn main() -> ! {
    let p = hal::Peripherals::take().unwrap();
    let cp = hal::CorePeripherals::take().unwrap();    

    let mut sc = p.SYSCTL.constrain();
    sc.clock_setup.oscillator = sysctl::Oscillator::Main(
        sysctl::CrystalFrequency::_16mhz,
        sysctl::SystemClock::UsePll(sysctl::PllOutputFrequency::_80_00mhz)
    );

    let clocks = sc.clock_setup.freeze();

    let mut porta = p.GPIO_PORTA.split(&sc.power_control);

    let mut uart = Serial::uart0(
        p.UART0,
        porta.pa1.into_af_push_pull::<hal::gpio::AF1>(&mut porta.control),
        porta.pa0.into_af_push_pull::<hal::gpio::AF1>(&mut porta.control),
        (),
        (),
        115200_u32.bps(),
        NewlineMode::SwapLFtoCRLF,
        &clocks,
        &sc.power_control
    );
    
    loop {
        writeln!(uart, "Hello World!!\r\n").unwrap();
    }
}