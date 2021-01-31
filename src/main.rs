#![no_std]
#![no_main]

extern crate panic_halt;
use arduino_uno::prelude::*;
use arduino_uno::hal::port::portb::PB5;
use arduino_uno::hal::port::mode::Output;

#[arduino_uno::entry]
fn main() -> ! {
    let peripherals = arduino_uno::Peripherals::take().unwrap();

    let mut pins = arduino_uno::Pins::new(
        peripherals.PORTB,
        peripherals.PORTC,
        peripherals.PORTD,
    );

    let mut led = pins.d13.into_output(&mut pins.ddr);
    let mut serial = arduino_uno::Serial::new(
        peripherals.USART0,
        pins.d0,
        pins.d1.into_output(&mut pins.ddr),
        57600.into_baudrate(),
    );

    ufmt::uwriteln!(&mut serial, "Hello from Arduino!\r").void_unwrap();

    loop {
        stutter_blink(&mut led, 25);
        let b = nb::block!(serial.read()).void_unwrap();

        ufmt::uwriteln!(&mut serial, "Got {}!\r", b).void_unwrap();
    }
}

fn stutter_blink(led: &mut PB5<Output>, times: usize) {
    for i in 0..times {
        led.toggle().void_unwrap();
        arduino_uno::delay_ms((i*10) as u16);
    }
}
