use arduino_hal::adc::AdcChannel;
use arduino_hal::hal::Atmega;
use arduino_hal::pac::ADC as AdcPeriph;
use arduino_hal::port::mode::{Analog, Output};
use arduino_hal::port::{mode, Pin, PinOps};
use crate::std::global_timer::GlobalTimer;
use crate::std::io::IoUno;
use crate::std::std::enable_interrupts;

const MAX_INPUT_VALUE: u16 = 512;
pub struct LightSensor<PW, OT> {
    power_pin: Option<Pin<mode::Output, PW>>,
    output_pin: Pin<Analog, OT>,

    last_data: u16,
    read_rate: u32,
    time: u32,


    is_read: bool
}

impl<PW, OT> LightSensor<PW, OT>
where
    PW: PinOps,
    OT: PinOps,
    Pin<Analog, OT>: AdcChannel<Atmega, AdcPeriph>,
{
    pub fn new(power_pin: Option<Pin<Output, PW>>, output_pin: Pin<Analog, OT>, read_rate: u32) -> Self {
        Self{
            power_pin,
            output_pin,
            last_data: 0,
            read_rate,
            time: 0,
            is_read: false,
        }
    }

    pub fn set_power(&mut self, state: bool){
        if let Some(pin) = self.power_pin.as_mut() {
            if (state) {
                pin.set_high();
            }else {
                pin.set_low();
            }
        }
    }

    pub fn update(&mut self, time: u32, adc: &mut arduino_hal::Adc) {
        if (time.wrapping_sub(self.time) >= self.read_rate as u32){
            self.last_data = adc.read_blocking(&self.output_pin);
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
//     let analog0 = pins.a0.into_analog_input(&mut adc);
//     let mut power = pins.d7.into_output();
//
//     let mut light_sensor = LightSensor::new(Some(power), analog0, 500);
//
//     light_sensor.set_power(true);
//     loop {
//         let now = timer.millis();
//
//         light_sensor.update(now, &mut adc);
//
//         if (light_sensor.is_read()) {
//             if (light_sensor.percent() > 0.3){
//                 light_sensor.set_power(false);
//             }
//         }
//         writeln!(io.str(), "Light Level: {}", light_sensor.last_data());
//         io.log();
//     }
// }
