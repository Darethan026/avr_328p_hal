#![no_std]
#![no_main]

use core::panic::PanicInfo;

use avr_328p_hal::gpio;
use avr_328p_hal::timers;

use gpio::*;
use timers::{Timer0, Timer1, Prescaler};

#[unsafe(no_mangle)]
pub extern "C" fn main() -> ! {

    let portb = PortB::take().unwrap();
    let portd = PortD::take().unwrap();

    let timer1 = Timer1::take().unwrap();

    portb.set_output(PinB::PB1); // PWM Pin
    portb.set_output(PinB::PB0); // Direction pin one
    portd.set_output(PinD::PD6); // Direction pin 2

    timer1.start();
    timer1.set_fast_pwm(999);
    timer1.set_duty_cycle(60);
    timer1.set_prescaler(Prescaler::Prescaler8);

    portb.set_high(PinB::PB1);

    portb.set_high(PinB::PB0);
    portd.set_low(PinD::PD6);

    loop {

    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    let portb = PortB::take().unwrap();
    let timer = Timer0::take().unwrap();

    portb.set_output(PinB::PB3);
    portb.set_output(PinB::PB5);
    
    portb.set_high(PinB::PB5);
        
    timer.start();
    
    // If the program panics, loop forever to stop execution
    loop {
        portb.toggle(PinB::PB3);
        timer.delay_ms(100);
    }
}
