#![no_std]
#![no_main]
#![feature(abi_avr_interrupt)]
use core::fmt::Write;
use modules::light_sensor_resistor::LightSensorResistor;
use panic_halt as _;
use crate::modules::temperature_sensor_LM25::TemperatureSensorLM25;

mod std;
mod modules;

use crate::std::global_timer::GlobalTimer;
use crate::std::io::IoUno;
use crate::std::std::enable_interrupts;
#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let mut adc = arduino_hal::Adc::new(dp.ADC, Default::default());

    let pins: arduino_hal::Pins = arduino_hal::pins!(dp);
    let timer = GlobalTimer::new(&dp.TC0);
    let mut io = IoUno::new(dp.USART0, pins.d0, pins.d1, 115200);
    enable_interrupts();


    let analog0 = pins.a0.into_analog_input(&mut adc);

    let mut temperature_sensor = TemperatureSensorLM25::new(analog0, 500);

    loop {
        let now = timer.millis();

        temperature_sensor.update(now, &mut adc);

        let (val, frac) = temperature_sensor.to_celsius();

        let _ = writeln!(io.str(), "Temperature: {}.{}C", val, frac);
        io.log();
    }
}
