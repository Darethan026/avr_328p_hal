//! A bare-metal driver for 16 pin LCDs, compatible with the Hitachi HD44780, JHD659 or LCDs with the SPLC780D controller

// Copyright (c) 2026 [Darell Ethan Kiganga]
// SPDX-License-Identifier: MIT

#![allow(unused_mut)]
#![allow(unused)]

pub struct LCD {
	rs: PinD,
	rw: PinD,
	db4: PinB,
	db5: PinB,
	db6: PinB,
	db7: PinB,
	enable: PinD,
	timer0: Timer0,
	lcd_portb: PortB,
	lcd_portd: PortD,
	cursor_idx: u8,
	_priv: (),
}

use crate::timers::*;
use crate::gpio::*;

static mut LCD_TAKING: bool = false;

impl LCD {
	/// Take the LCD
	pub fn take() -> Option<Self> {
		unsafe {
			if LCD_TAKING {
				None
			} else {
				LCD_TAKING = true;

				let timer0 = Timer0::take().unwrap();

				let lcd_portb = PortB::take().unwrap();
				let lcd_portd = PortD::take().unwrap();

				let rs = lcd_portd.set_output(PinD::PD5);
				let rw = lcd_portd.set_output(PinD::PD6);
				let db4 = lcd_portb.set_output(PinB::PB0);
				let db5 = lcd_portb.set_output(PinB::PB1);
				let db6 = lcd_portb.set_output(PinB::PB2);
				let db7 = lcd_portb.set_output(PinB::PB3);
				let enable = lcd_portd.set_output(PinD::PD7);

				let mut cursor_index = 0 as u32;

				Some(LCD {
					lcd_portb,
					lcd_portd,
					rs,
					rw,
					db4,
					db5,
					db6,
					db7,
					enable,
					timer0,
					cursor_idx: cursor_index as u8,
					_priv: (),
				})
			}
		}
	}

	/// Initialise the Lcd
	pub fn init(&self) {
		self.timer0.start();
		self.timer0.delay_ms(15);

		self.lcd_portd.set_low(self.rs);
		self.lcd_portd.set_low(self.rw);

		// Force 8-bit mode
		self.lcd_portb.set_high(self.db4);
		self.lcd_portb.set_high(self.db5);
		self.lcd_portb.set_low(self.db6);
		self.lcd_portb.set_low(self.db7);

		self.lcd_portd.set_high(self.enable);
		self.timer0.delay_ms(1);
		self.lcd_portd.set_low(self.enable);

		self.timer0.delay_ms(5);

		self.lcd_portb.set_high(self.db4);
		self.lcd_portb.set_high(self.db5);
		self.lcd_portb.set_low(self.db6);
		self.lcd_portb.set_low(self.db7);

		self.lcd_portd.set_high(self.enable);
		self.timer0.delay_ms(1);
		self.lcd_portd.set_low(self.enable);

		self.timer0.delay_ms(1);

		self.lcd_portb.set_high(self.db4);
		self.lcd_portb.set_high(self.db5);
		self.lcd_portb.set_low(self.db6);
		self.lcd_portb.set_low(self.db7);

		self.lcd_portd.set_high(self.enable);
		self.timer0.delay_ms(1);
		self.lcd_portd.set_low(self.enable);

		self.timer0.delay_ms(1);

		// Set 4-bit mode
		self.lcd_portb.set_low(self.db4);
		self.lcd_portb.set_high(self.db5);
		self.lcd_portb.set_low(self.db6);
		self.lcd_portb.set_low(self.db7);

		self.lcd_portd.set_high(self.enable);
		self.timer0.delay_ms(1);
		self.lcd_portd.set_low(self.enable);

		self.timer0.delay_ms(1);

		// Function set
		self.lcd_portb.set_low(self.db4);
		self.lcd_portb.set_high(self.db5);
		self.lcd_portb.set_low(self.db6);
		self.lcd_portb.set_low(self.db7);

		self.lcd_portd.set_high(self.enable);
		self.timer0.delay_ms(1);
		self.lcd_portd.set_low(self.enable);

		self.lcd_portb.set_low(self.db4);
		self.lcd_portb.set_low(self.db5);
		self.lcd_portb.set_low(self.db6);
		self.lcd_portb.set_high(self.db7);

		self.lcd_portd.set_high(self.enable);
		self.timer0.delay_ms(1);
		self.lcd_portd.set_low(self.enable);
		self.timer0.delay_ms(1);

		// Clear display
		self.lcd_portb.set_low(self.db4);
		self.lcd_portb.set_low(self.db5);
		self.lcd_portb.set_low(self.db6);
		self.lcd_portb.set_low(self.db7);

		self.lcd_portd.set_high(self.enable);
		self.timer0.delay_ms(1);
		self.lcd_portd.set_low(self.enable);

		self.lcd_portb.set_high(self.db4);
		self.lcd_portb.set_low(self.db5);
		self.lcd_portb.set_low(self.db6);
		self.lcd_portb.set_low(self.db7);

		self.lcd_portd.set_high(self.enable);
		self.timer0.delay_ms(3);
		self.lcd_portd.set_low(self.enable);

		self.timer0.delay_ms(1);

		// Set entry mode
		self.lcd_portb.set_low(self.db4);
		self.lcd_portb.set_low(self.db5);
		self.lcd_portb.set_low(self.db6);
		self.lcd_portb.set_low(self.db7);

		self.lcd_portd.set_high(self.enable);
		self.timer0.delay_ms(1);
		self.lcd_portd.set_low(self.enable);

		self.lcd_portb.set_low(self.db4);
		self.lcd_portb.set_high(self.db5);
		self.lcd_portb.set_high(self.db6);
		self.lcd_portb.set_low(self.db7);

		self.lcd_portd.set_high(self.enable);
		self.timer0.delay_ms(1);
		self.lcd_portd.set_low(self.enable);
		self.timer0.delay_ms(2);

		// Turn on the display
		self.lcd_portb.set_low(self.db4);
		self.lcd_portb.set_low(self.db5);
		self.lcd_portb.set_low(self.db6);
		self.lcd_portb.set_low(self.db7);

		self.lcd_portd.set_high(self.enable);
		self.timer0.delay_ms(1);
		self.lcd_portd.set_low(self.enable);

		self.timer0.delay_ms(1);

		self.lcd_portb.set_high(self.db4);
		self.lcd_portb.set_high(self.db5);
		self.lcd_portb.set_high(self.db6);
		self.lcd_portb.set_high(self.db7);

		self.lcd_portd.set_high(self.enable);
		self.timer0.delay_ms(1);
		self.lcd_portd.set_low(self.enable);
		self.timer0.delay_ms(1);
	}

	fn write_nibble(&self, nibble: u8) {
		let high = (nibble >> 4) as u8;
		let low = nibble;

		self.lcd_portd.set_high(self.rs);
		self.lcd_portd.set_low(self.rw);

		// Check the high bits (nibble)
		if (high & 1) !=0 {
			self.lcd_portb.set_high(self.db4);
		} else {
			self.lcd_portb.set_low(self.db4);
		}

		if (high & 2) !=0 {
			self.lcd_portb.set_high(self.db5);
		} else {
			self.lcd_portb.set_low(self.db5);
		}

		if (high & 4) !=0 {
			self.lcd_portb.set_high(self.db6);
		} else {
			self.lcd_portb.set_low(self.db6);
		}

		if (high & 8) !=0 {
			self.lcd_portb.set_high(self.db7);
		} else {
			self.lcd_portb.set_low(self.db7);
		}

		self.lcd_portd.set_high(self.enable);
		self.timer0.delay_ms(1);
		self.lcd_portd.set_low(self.enable);

		// Check the low bits (nibble)
		if (low & 1) !=0 {
			self.lcd_portb.set_high(self.db4);
		} else {
			self.lcd_portb.set_low(self.db4);
		}

		if (low & 2) !=0 {
			self.lcd_portb.set_high(self.db5);
		} else {
			self.lcd_portb.set_low(self.db5);
		}

		if (low & 4) !=0 {
			self.lcd_portb.set_high(self.db6);
		} else {
			self.lcd_portb.set_low(self.db6);
		}

		if (low & 8) !=0 {
			self.lcd_portb.set_high(self.db7);
		} else {
			self.lcd_portb.set_low(self.db7);
		}

		self.lcd_portd.set_high(self.enable);
		self.timer0.delay_ms(1);
		self.lcd_portd.set_low(self.enable);

		self.timer0.delay_ms(1);
	}

	pub fn write_char(&mut self, character: char) {
		self.write_nibble(character as u8);

		self.cursor_idx += 1;

		if self.cursor_idx == 16 {
			// Shift the cursor
			self.lcd_portd.set_low(self.rs);
			self.lcd_portd.set_low(self.rw);
			self.lcd_portb.set_low(self.db6);
			self.lcd_portb.set_low(self.db7);

			// Move it to line 2  
			self.lcd_portb.set_low(self.db4);
			self.lcd_portb.set_low(self.db5);
			self.lcd_portb.set_high(self.db6);
			self.lcd_portb.set_high(self.db7);

			self.lcd_portd.set_high(self.enable);
			self.timer0.delay_ms(1);
			self.lcd_portd.set_low(self.enable);

			self.lcd_portb.set_low(self.db4);
			self.lcd_portb.set_low(self.db5);
			self.lcd_portb.set_low(self.db6);
			self.lcd_portb.set_low(self.db7);

			self.lcd_portd.set_high(self.enable);
			self.timer0.delay_ms(1);
			self.lcd_portd.set_low(self.enable);

			self.timer0.delay_ms(2);
		} else if self.cursor_idx == 32 {
			self.clear_display();
			self.cursor_idx = 0;
		} else {
			//
		}
	}

	pub fn clear_display(&mut self) {
		self.lcd_portd.set_low(self.rs);
		self.lcd_portd.set_low(self.rw);
		self.lcd_portb.set_low(self.db4);
		self.lcd_portb.set_low(self.db5);
		self.lcd_portb.set_low(self.db6);
		self.lcd_portb.set_low(self.db7);

		self.lcd_portd.set_high(self.enable);
		self.timer0.delay_ms(2);
		self.lcd_portd.set_low(self.enable);

		self.timer0.delay_ms(1);

		self.lcd_portb.set_high(self.db4);
		self.lcd_portb.set_low(self.db5);
		self.lcd_portb.set_low(self.db6);
		self.lcd_portb.set_low(self.db7);

		self.lcd_portd.set_high(self.enable);
		self.timer0.delay_ms(2);
		self.lcd_portd.set_low(self.enable);

		self.cursor_idx = 0;

		self.timer0.delay_ms(2);
	}

	pub fn print_number(&mut self, num: u16) {
		let mut buffer = [' '; 5];

		let thousands = (num / 1000) % 10;
		let hundreds = (num / 100) % 10;
		let tens = (num / 10) % 10;
		let ones = num % 10;

		buffer[0] = (thousands as u8 + b'0') as char;
		buffer[1] = (hundreds as u8 + b'0') as char;
		buffer[2] = (tens as u8 + b'0') as char;
		buffer[3] = (ones as u8 + b'0') as char;

		for c in buffer {
			self.write_char(c);
		}
	}
}

impl Drop for LCD {
    fn drop(&mut self) {
        unsafe { LCD_TAKING = false; }
    }
}

// pg 8 lcd datasheet