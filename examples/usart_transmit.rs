#![no_std]
#![no_main]

use core::panic::PanicInfo;

use avr_328p_hal::gpio;
use avr_328p_hal::timers;
use avr_328p_hal::usart0;

use gpio::*;
use timers::{Timer0, Timer1, Prescaler};
use usart0::{USART0, Mode};

#[unsafe(no_mangle)]
pub extern "C" fn main() -> ! {

    let portb = PortB::take().unwrap();
    let timer1 = Timer1::take().unwrap();
    let usart = USART0::take().unwrap();

    timer1.start();
    timer1.set_ctc_mode();
    timer1.set_top_value(62499);
    timer1.set_prescaler(Prescaler::Prescaler256);

    usart.start();
    usart.set_baud_rate(9600, Mode::DoubleSpeedAsynchronous);
    usart.set_frame_format();
    usart.enable_tx_rx();

    portb.set_output(PinB::PB0);

    loop {
        usart.transmit_char('A');
        portb.set_high(PinB::PB0);
        timer1.wait_for_match();

        portb.set_low(PinB::PB0);
        timer1.wait_for_match();
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    let portb = PortB::take().unwrap();
    let timer = Timer0::take().unwrap();

    portb.set_output(PinB::PB5);
    
    portb.set_high(PinB::PB5);
        
    timer.start();
    
    // If the program panics, loop forever to stop execution
    loop {
        portb.set_high(PinB::PB5);
        timer.delay_ms(100);
        portb.set_low(PinB::PB5);
    }
}
