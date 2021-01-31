#![no_std]
#![no_main]

extern crate panic_halt;
use arduino_uno::prelude::*;
use arduino_uno::hal::port::portb::PB5;
use arduino_uno::hal::port::mode::Output;

const DOT_MS: u16 = 200;
const DASH_MS: u16 = DOT_MS * 3;
const CODES: [&str; 36] = [
                            "01","1000","1010","100","0","0010","110","0000",
                            "00","0111","101","0100","11","10","111","0110",
                            "1101","010","000","1","001","0001","011","1001",
                            "1011","1100","11111","01111","00111","00011",
                            "00001","00000","10000","11000","11100","11110"
                           ];

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
        // let b = nb::block!(serial.read()).void_unwrap();

        // ufmt::uwriteln!(&mut serial, "Got {}!\r", b).void_unwrap();
        blink(&mut led, "0110");
    }
}

fn blink(led: &mut PB5<Output>, code: &str) {
    led.set_low().void_unwrap();
    for i in code.chars() {
        led.set_high().void_unwrap();
        match i {
            '0' => arduino_uno::delay_ms(DOT_MS),
            '1' => arduino_uno::delay_ms(DASH_MS),
            _ => (),
        }
        led.set_low().void_unwrap();
        arduino_uno::delay_ms(DOT_MS);
    }
    arduino_uno::delay_ms(DASH_MS);
}

