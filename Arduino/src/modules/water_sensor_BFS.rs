use arduino_hal::port::{mode, Pin, PinOps};
use arduino_hal::adc::{AdcChannel, Channel};
use arduino_hal::hal::Atmega;
use arduino_hal::pac::ADC as AdcPeriph;
use arduino_hal::port::mode::{Analog, Output, PullUp};
use crate::std::global_timer::GlobalTimer;
use crate::std::io::IoUno;
use crate::std::std::enable_interrupts;

const MAX_INPUT_VALUE: u16 = 1024;
pub struct WaterSensorBFS<PW, OT> {
    power_pin: Pin<mode::Output, PW>,
    output_pin: Pin<Analog, OT>,

    last_data: u16,
    read_rate: u32,
    time: u32,


    is_read: bool
}

impl<PW, OT> WaterSensorBFS<PW, OT>
where
    PW: PinOps,
    OT: PinOps,
    Pin<Analog, OT>: AdcChannel<Atmega, AdcPeriph>,
{
    pub fn new(power_pin: Pin<Output, PW>, output_pin: Pin<Analog, OT>, read_rate: u32) -> Self {
        Self{
            power_pin,
            output_pin,
            last_data: 0,
            read_rate,
            time: 0,
            is_read: false,
        }
    }

    pub fn update(&mut self, time: u32, adc: &mut arduino_hal::Adc) {
        if (time.wrapping_sub(self.time) >= self.read_rate as u32){
            self.power_pin.set_high();
            self.last_data = adc.read_blocking(&self.output_pin);
            self.power_pin.set_low();

            self.time = time;
            self.is_read = true;
        }else{
            self.is_read = false;
        }
    }

    pub fn is_read(&self) -> bool{
        self.is_read
    }

    pub fn last_data(&self) -> u16 {
        self.last_data
    }

    pub fn percent(&self)-> f32{
        return self.last_data as f32 / MAX_INPUT_VALUE as f32;
    }
}



//Example how to use
// #[arduino_hal::entry]
// fn main() -> ! {
//     let dp = arduino_hal::Peripherals::take().unwrap();
//     let mut adc = arduino_hal::Adc::new(dp.ADC, Default::default());
//
//     let pins: arduino_hal::Pins = arduino_hal::pins!(dp);
//     let timer = GlobalTimer::new(&dp.TC0);
//     let mut io = IoUno::new(dp.USART0, pins.d0, pins.d1, 115200);
//     enable_interrupts();
//
//
//     let mut power = pins.d7.into_output();
//     let analog0 = pins.a0.into_analog_input(&mut adc);
//
//     let mut water_sensor = WaterSensorBFS::new(power, analog0, 500);
//
//
//     loop {
//         let now = timer.millis();
//         water_sensor.update(now, &mut adc);
//
//         if (water_sensor.is_read()){
//             writeln!(io.str(), "Water Level: {}", water_sensor.last_data());
//             io.log();
//         }
//     }
// }
