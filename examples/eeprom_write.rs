#![no_std]
#![no_main]

use core::panic::PanicInfo;

use avr_328p_hal::eeprom::EEPROM;
use avr_328p_hal::usart0::*;
use avr_328p_hal::timers::Timer0;

#[unsafe(no_mangle)]

pub extern "C" fn main() -> ! {
	let eeprom = EEPROM::take().unwrap(); // Take EEPROM
	let usart = USART0::take().unwrap(); // Take USART
	let timer = Timer0::take().unwrap(); // Take Timer 0

	let data = 0x0A; // Use number 10 (in hexadecimal) as data

	eeprom.eeprom_write(3, data); // Write number 10 to address 3 of 1023 on the EEPROM

	usart.init(); // Start the USART
	usart.set_baud_rate(115200, Mode::DoubleSpeedAsynchronous); // Set baud rate to 115200

	loop {
		let data = eeprom.eeprom_read(3); // Read address 3 of 1023 on the EEPROM

		usart.transmit_char(data as char); // Transmit the data to Serial
		timer.delay_ms(500); // Delay for half a second
	}

}


#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {

    }
}
