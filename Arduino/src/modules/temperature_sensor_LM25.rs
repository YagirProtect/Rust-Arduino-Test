use arduino_hal::adc::AdcChannel;
use arduino_hal::hal::Atmega;
use arduino_hal::pac::ADC as AdcPeriph;
use arduino_hal::port::mode::Analog;
use arduino_hal::port::{Pin, PinOps};

const MAX_INPUT_VALUE: u16 = 1023;
pub struct TemperatureSensorLM25<OT> {
    output_pin: Pin<Analog, OT>,

    last_data: u16,
    read_rate: u32,
    time: u32,

    is_read: bool,
}

impl<OT> TemperatureSensorLM25<OT>
where
    OT: PinOps,
    Pin<Analog, OT>: AdcChannel<Atmega, AdcPeriph>,
{
    pub fn new(output_pin: Pin<Analog, OT>, read_rate: u32) -> Self {
        Self {
            output_pin,
            last_data: 0,
            read_rate,
            time: 0,
            is_read: false,
        }
    }
    pub fn update(&mut self, time: u32, adc: &mut arduino_hal::Adc) {
        if (time.wrapping_sub(self.time) >= self.read_rate as u32) {
            self.last_data = adc.read_blocking(&self.output_pin);
            self.time = time;
            self.is_read = true;
        } else {
            self.is_read = false;
        }
    }

    pub fn is_read(&self) -> bool {
        self.is_read
    }

    pub fn last_data(&self) -> u16 {
        self.last_data
    }

    ///value/frac
    pub fn to_celsius(&self) -> (u32, u32) {
        let raw = self.last_data() as u32;

        let mv = raw * 5000 / MAX_INPUT_VALUE as u32;

        let t_int = mv / 10;
        let t_frac = mv % 10;


        (t_int, t_frac)
    }

    pub fn to_fahrenheit(&self) -> (u32, u32) {
        let (c_int, c_frac) = self.to_celsius();
        let c10 = c_int * 10 + c_frac; // °C * 10

        let f10 = (c10 * 9 + 2) / 5 + 320;

        let f_int = f10 / 10;
        let f_frac = f10 % 10;
        (f_int, f_frac)
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
//
//     let mut temperature_sensor = TemperatureSensorLM25::new(analog0, 500);
//
//     loop {
//         let now = timer.millis();
//
//         temperature_sensor.update(now, &mut adc);
//
//         let (val, frac) = temperature_sensor.to_celsius();
//
//         let _ = writeln!(io.str(), "Temperature: {}.{}C", val, frac);
//         io.log();
//     }
// }
