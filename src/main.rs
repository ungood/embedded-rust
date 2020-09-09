#![no_std]
#![no_main]

// pick a panicking behavior
extern crate panic_halt; // you can put a breakpoint on `rust_begin_unwind` to catch panics
// extern crate panic_abort; // requires nightly
// extern crate panic_itm; // logs messages over ITM; requires ITM support
// extern crate panic_semihosting; // logs messages to the host stderr; requires a debugger

extern crate cortex_m;
extern crate stm32f30x_hal as hal;

use cortex_m_rt::entry;
use stm32f30x_hal as hal;

use f3;


#[entry]
fn main() -> ! {
    let p = hal::Peripherals::take().unwrap();
    let cp = hal::CorePeripherals::take().unwrap();

    let mut timer = enableTimer(p);

    loop {
        while !timer.has_wrapped() {}
        // your code goes here
    }
}

fn enableTimer(peripherals: hal::Peripherals) -> hal::SYST {
    let ticks_per_sec = hal::SYST::get_ticks_per_10ms() * 100;
    let mut timer = peripherals.SYST;
    timer.set_clock_source(hal::SystClkSource::Core);
    timer.set_reload(ticks_per_sec);
    timer.clear_current();
    timer.enable_counter();

    return timer;
}
