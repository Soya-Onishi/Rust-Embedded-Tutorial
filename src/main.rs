#![no_std]
#![no_main]

// pick a panicking behavior
use panic_halt as _; // you can put a breakpoint on `rust_begin_unwind` to catch panics
// use panic_abort as _; // requires nightly
// use panic_itm as _; // logs messages over ITM; requires ITM support
// use panic_semihosting as _; // logs messages to the host stderr; requires a debugger

use core::fmt::Write;


use cortex_m_rt::entry;
use volatile_register::{RW, RO};
use cortex_m_semihosting::{debug, hio};

#[derive(Debug)]
struct Foo {
    num: u32
}

struct FooBuilder {
    a: u32,
    b: u32
}

impl FooBuilder {
    fn new(starter: u32) -> Self {
        Self { a: starter, b: starter }
    }

    fn double_a(&self) -> Self {
        Self {
            a: self.a * 2,
            b: self.b
        }
    }

    fn into_foo(&self) -> Foo {
        Foo { num: self.a + self.b }
    }
}

struct SysTick {
    p: &'static mut RegisterBlock
}

#[repr(C)]
struct RegisterBlock {
    pub csr: RW<u32>,
    pub rvr: RW<u32>,
    pub cvr: RW<u32>,
    pub calib: RO<u32>
}

impl SysTick {
    pub fn new() -> SysTick {
        SysTick {
            p: unsafe { &mut *(0xE000_E010 as *mut RegisterBlock) }
        }
    }

    pub fn get_time(&self) -> u32 {
        self.p.cvr.read()
    }

    pub fn set_reload(&mut self, reload_value: u32) {
        unsafe { self.p.rvr.write(reload_value); }
    }
}

#[entry]
fn main() -> ! {
    let mut stdout = hio::hstdout().unwrap();
    let mut syst = SysTick::new();
    syst.set_reload(0x0012_0000);

    for _ in 0..100 {
        writeln!(stdout, "time: {}", syst.get_time()).unwrap();
    }

    let builder = FooBuilder::new(2);
    let builder = builder.double_a();
    let foo = builder.into_foo();

    writeln!(stdout, "{:?}", foo).unwrap();

    debug::exit(debug::EXIT_SUCCESS);

    loop {}
}