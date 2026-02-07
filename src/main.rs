#![no_std]
#![no_main]
#![feature(abi_avr_interrupt)]

mod std;

use panic_halt as _;

use arduino_hal::prelude::*;
use crate::std::global_timer::GlobalTimer;
use crate::std::std::enable_interrupts;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let timer = GlobalTimer::new(&dp.TC0);

    enable_interrupts();
    
    
    let pins: arduino_hal::Pins = arduino_hal::pins!(dp);

    
    let mut led = pins.d13.into_output();
    let mut last = timer.millis();
    let mut on = false;

    loop {
        let now = timer.millis();
        if now.wrapping_sub(last) >= 200 {
            last = now;
            on = !on;
            if on { led.set_high(); } else { led.set_low(); }
        }
    }
}
