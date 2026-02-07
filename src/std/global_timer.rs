use core::cell::Cell;
use avr_device::interrupt::Mutex;
static MILLIS: Mutex<Cell<u32>> = Mutex::new(Cell::new(0));

fn millis() -> u32 {
    avr_device::interrupt::free(|cs| MILLIS.borrow(cs).get())
}

pub struct GlobalTimer{}

impl GlobalTimer {

    pub fn new(tc0: &arduino_hal::pac::TC0) -> Self {
        const OCR0A_1MS: u8 = 249;

        tc0.tccr0a().write(|w| w.wgm0().ctc());
        tc0.ocr0a().write(|w| unsafe { w.bits(OCR0A_1MS) });
        tc0.tccr0b().write(|w| w.cs0().prescale_64());
        tc0.timsk0().write(|w| w.ocie0a().set_bit());


        Self{}
    }

    pub fn millis(&self) -> u32 {
        return millis();
    }
}


//INTERRUPTS

#[avr_device::interrupt(atmega328p)]
fn TIMER0_COMPA() {
    avr_device::interrupt::free(|cs| {
        let c = MILLIS.borrow(cs);
        c.set(c.get().wrapping_add(1));
    });
}
