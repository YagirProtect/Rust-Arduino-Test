use arduino_hal::hal::port::{PD0, PD1};
use arduino_hal::hal::Usart;
use arduino_hal::port::mode::{Floating, Input, Output};
use arduino_hal::port::{Pin, D0, D1};
use arduino_hal::usart::Baudrate;
use arduino_hal::DefaultClock;
use avr_device::atmega328p::USART0;
use heapless::String;
use ufmt::uwriteln;

pub struct IoUno {
    serial: Usart<USART0, Pin<Input, PD0>, Pin<Output, PD1>, DefaultClock>,
    string: String<64>
}


impl IoUno {
    pub fn new(usart0: USART0, rx: Pin<Input<Floating>, D0>, tx: Pin<Input<Floating>, D1>, val: i32) -> Self {

        let serial = arduino_hal::Usart::new(
            usart0,
            rx,
            tx.into_output(),
            Baudrate::new(val as u32),
        );

        Self{
            serial,
            string: String::new()
        }
    }

    pub fn str(&mut self) -> &mut String<64>{

        self.string.clear();
        return &mut self.string;
    }


    pub fn get_serial(&mut self) -> &mut Usart<USART0, Pin<Input, PD0>, Pin<Output, PD1>, DefaultClock>{
        return &mut self.serial;
    }
    pub fn log(&mut self) {
        let _ = uwriteln!(&mut self.serial, "{}", self.string.as_str());
    }
}