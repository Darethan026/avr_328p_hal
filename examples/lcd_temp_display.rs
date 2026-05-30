//! An example using the JHD659 LCD to print temperature

#![no_std]
#![no_main]
#![allow(unused_imports)]

use core::panic::PanicInfo;

use avr_328p_hal::{timers, gpio, adc, lcd, usart0};

use lcd::*;
use gpio::*;
use timers::*;
use usart0::*;
use adc::*;

#[unsafe(no_mangle)]
pub extern "C" fn main() -> ! {
    let mut lcd = LCD::take().expect("LCD TAKEN!!");

    let timer = Timer1::take().unwrap(); // Timer1
    let tmp_sensor = ADC::take().unwrap();

    tmp_sensor.start();
    tmp_sensor.select_input_channel(InputChannel::AnalogPin0);
    tmp_sensor.set_reference(VoltageReference::Internal);
    
    let offset_voltage_mv = 500 as u32;
    let scaling_in_mv = 10 as u32;

    let mut sum = 0 as u32;

    timer.start();
    timer.set_ctc_mode();
    timer.set_top_value(62499);
    timer.set_prescaler(Prescaler::Prescaler256);

    lcd.init();

    loop {
        for _val in 0..100 {
            sum += tmp_sensor.start_conversion(ADCprescaler::Prescaler128) as u32;
        }

        let temp_val_mv = sum / 100;

        let voltage = (temp_val_mv - offset_voltage_mv) / scaling_in_mv;

        lcd.print("Temp: ");
        lcd.print_number(voltage as u16);
        lcd.write_char(0xDF as char);
        lcd.write_char('C');
        timer.wait_for_match();

        sum = 0;

        lcd.clear_display();
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    let timer = Timer1::take().unwrap();
    let portb = PortB::take().unwrap();

    portb.set_output(PinB::PB5);
        
    timer.start();
    timer.set_ctc_mode();
    timer.set_top_value(24999);
    timer.set_prescaler(Prescaler::Prescaler64);
    
    // If the program panics, blink the PB5 LED
    loop {
        portb.set_high(PinB::PB5);
        timer.wait_for_match();
        portb.set_low(PinB::PB5);
    }
}