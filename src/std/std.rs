pub fn enable_interrupts() {
    unsafe { avr_device::interrupt::enable(); }
}