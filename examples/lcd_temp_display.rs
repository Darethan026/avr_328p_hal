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