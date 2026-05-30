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
        usart.print_string("TESTING");
        portb.set_high(PinB::PB0);
        timer1.wait_for_match();

        portb.set_low(PinB::PB0);
        timer1.wait_for_match();
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    unsafe {
        // Force PB5 (Pin 13) to Output mode by writing directly to DDRB (0x24)
        let ddrb = 0x24 as *mut u8;
        core::ptr::write_volatile(ddrb, core::ptr::read_volatile(ddrb) | (1 << 5));

        let portb = 0x25 as *mut u8;
        loop {
            // Blink the LED using raw memory blocks to avoid ownership locks
            core::ptr::write_volatile(portb, core::ptr::read_volatile(portb) ^ (1 << 5));
            
            // Simple busy-loop delay because we cannot take the Timer singleton
            for _ in 0..40000 { core::hint::black_box(()); }
        }
    }
}