#![no_std]
#![no_main]
#![feature(abi_avr_interrupt)]
use core::fmt::Write;
use panic_halt as _;
use crate::joystick::Joystick;
use crate::light_sensor::LightSensor;

mod std;
mod joystick;
mod water_sensor;
mod light_sensor;

use crate::std::global_timer::GlobalTimer;
use crate::std::io::IoUno;
use crate::std::std::enable_interrupts;
use crate::water_sensor::WaterSensorBFS;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let mut adc = arduino_hal::Adc::new(dp.ADC, Default::default());

    let pins: arduino_hal::Pins = arduino_hal::pins!(dp);
    let timer = GlobalTimer::new(&dp.TC0);
    let mut io = IoUno::new(dp.USART0, pins.d0, pins.d1, 115200);
    enable_interrupts();


    let analog0 = pins.a0.into_analog_input(&mut adc);
    let mut power = pins.d7.into_output();

    let mut light_sensor = LightSensor::new(Some(power), analog0, 500);

    light_sensor.set_power(true);
    loop {
        let now = timer.millis();

        light_sensor.update(now, &mut adc);
        
        if (light_sensor.is_read()) {
            if (light_sensor.percent() > 0.3){
                light_sensor.set_power(false);
            }
        }
        writeln!(io.str(), "Light Level: {}", light_sensor.last_data());
        io.log();
    }
}
