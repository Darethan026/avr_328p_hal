//! A safe, bare-metal USART driver for the ATmega328P written in bare metal rust using the Atmega328p datasheet registers.

// Copyright (c) 2026 [Darell Ethan Kiganga]
// SPDX-License-Identifier: MIT

#![allow(dead_code)]
#[repr(u8)]
#[derive(Copy, Clone)]
pub enum Mode {
	NormalAsynchronous,
	DoubleSpeedAsynchronous,
	MasterSynchronous,
}

pub struct USART0 {
	_priv: (),
}

static mut USART0_TAKING: bool = false;

use crate::gpio;

use gpio::*;

impl USART0 {
	const PRR: *mut u8 = 0x64 as *mut u8; // Power reduction register
	const UBRR0H: *mut u8 = 0xC5 as *mut u8; // USART Baud rate register High
	const UBRR0L: *mut u8 = 0xC4 as *mut u8; // USART Baud rate register low
	const CPU_SPEED: u32 = 16000000; // Cpu frequency in Hz
	const UCSR0A: *mut u8 = 0xC0 as *mut u8; // USART Control and status register A
	const UCSR0B: *mut u8 = 0xC1 as *mut u8; // USART Control and status register B
	const UCSR0C: *mut u8 = 0xC2 as *mut u8; // USART Control and status register C
	const UDR0: *mut u8 = 0xC6 as *mut u8;

	/// Take the USART0
    pub fn take() -> Option<Self> {
       	unsafe {
            if USART0_TAKING {
                None
            } else {
                USART0_TAKING = true;
                Some(USART0 { _priv: () })
            }
        }
    }

	/// Start the USART on 1 start bit, with 8 data bits and 1 stop bit, with no parity
	pub fn start(&self) {
		unsafe {
			// Read the power reduction register
			let val = core::ptr::read_volatile(Self::PRR);

			// Clear bit 1 (PRUSART0) to wake the USART
			core::ptr::write_volatile(Self::PRR, val & !(1 << 1 as u8));
		}
	}

	/// Set the baud rate after starting the USART
	pub fn set_baud_rate(&self, baud_rate: u32, mode: Mode) {
		unsafe {
			match mode {
				Mode::NormalAsynchronous => {
					// Read the value on UCSR0A
					let val = core::ptr::read_volatile(Self::UCSR0A);
					let other_val = core::ptr::read_volatile(Self::UCSR0C);

					// Clear bit 2 (U2X0)
					core::ptr::write_volatile(Self::UCSR0A, val & !(1 << 2 as u8) & !(1 << 3 as u8) & !(1 << 4 as u8) | (1 << 1 as u8));

					// Set asynchronous USART by clearing bit 6 and 7
					core::ptr::write_volatile(Self::UCSR0C, other_val & !(1 << 6 as u8) & !(1 << 7 as u8));

					let ubrrn = ((Self::CPU_SPEED) / (16 * baud_rate)) - 1;

					if ubrrn <= 4095 {
						core::ptr::write_volatile(Self::UBRR0H, (ubrrn >> 8) as u8);
						core::ptr::write_volatile(Self::UBRR0L, (ubrrn) as u8);
					} else {
						panic!();
					}
				}

				Mode::DoubleSpeedAsynchronous => {
					// Read the value on UCSR0A
					let val = core::ptr::read_volatile(Self::UCSR0A);
					let other_val = core::ptr::read_volatile(Self::UCSR0C);

					// Set the U2X0 bit to 1
					core::ptr::write_volatile(Self::UCSR0A, val & !(1 << 2 as u8) & !(1 << 3 as u8) & !(1 << 4 as u8) | (1 << 1 as u8));

					// Set asynchronous USART by clearing bit 6 and 7
					core::ptr::write_volatile(Self::UCSR0C, other_val & !(1 << 6 as u8) & !(1 << 7 as u8));

					let ubrrn = ((Self::CPU_SPEED) / (8 * baud_rate)) - 1;

					if ubrrn <= 4095 {
						core::ptr::write_volatile(Self::UBRR0H, (ubrrn >> 8) as u8);
						core::ptr::write_volatile(Self::UBRR0L, (ubrrn) as u8);
					} else {
						panic!();
					}
				}

				Mode::MasterSynchronous => {
					let xck = PortD::take().unwrap();

					// Read the value on UCSR0C
					let val = core::ptr::read_volatile(Self::UCSR0C);

					// Clear and set bits for Synchronous USART
					core::ptr::write_volatile(Self::UCSR0C, val & !(1 << 7 as u8) | (1 << 6 as u8) | (1 << 0 as u8));

					// Set the pin as output to ensure it's master
					xck.set_output(PinD::PD4);

					let ubrrn = ((Self::CPU_SPEED) / (2 * baud_rate)) - 1;

					if ubrrn <= 4095 {
						core::ptr::write_volatile(Self::UBRR0H, (ubrrn >> 8) as u8);
						core::ptr::write_volatile(Self::UBRR0L, (ubrrn) as u8);
					} else {
						panic!();
					}
				}
			}
		}
	}


	/// Set the frame format after setting the baud rate
	pub fn set_frame_format(&self) {
		unsafe {
			// Read the value on the UCSR0B and UCSR0C registers
			let val = core::ptr::read_volatile(Self::UCSR0B);
			let other_val = core::ptr::read_volatile(Self::UCSR0C);

			// Set the frame format by setting 1 start bit, 8 data bits, and 1 stop bit with no parity
			core::ptr::write_volatile(Self::UCSR0B, val & !(1 << 2 as u8));
			core::ptr::write_volatile(Self::UCSR0C, other_val & !(1 << 3 as u8) & !(1 << 4 as u8) & !(1 << 5 as u8) | (1 << 1 as u8) | (1 << 2 as u8));
		}
	}

	/// Enable TX and RX to wake up the transmitter and receiver
	pub fn enable_tx_rx(&self) {
		unsafe {
			// Read the value on the UCSR0B register
			let val = core::ptr::read_volatile(Self::UCSR0B);

			// Enable TX and RX
			core::ptr::write_volatile(Self::UCSR0B, val | (1 << 3 as u8) | (1 << 4 as u8));
		}
	}

	/// Transmit data in the form of a char after enabling TX and RX
	pub fn transmit_char(&self, character: char) {
		unsafe {
			// Check the UCSR0A register to see whether the UDREn (USART Data Register Empty) is empty
			while core::ptr::read_volatile(Self::UCSR0A) & (1 << 5 as u8) == 0 {
				// Wait...
			}

			// Since it's empty, we load the character to the UDRO register so that it's loaded to the transmit buffer
			core::ptr::write_volatile(Self::UDR0, character as u8);
		}
	}

	/// Send a string
	// (Can only send similar characters or single characters at once)
	pub fn send_string(&self, string: &str) {
		for val in string.as_bytes() {
			self.transmit_char(*val as char);
		}
	}

	/// Set the usart to receive
	pub fn usart_receive(&self) {
		unsafe {
			// Read the value on UCSR0B
			let val = core::ptr::read_volatile(Self::UCSR0B);

			// Write a 1 to the receive enable bit (RXEn) in UCSR0B
			core::ptr::write_volatile(Self::UCSR0B, val | (1 << 4 as u8));

			while core::ptr::read_volatile(Self::UCSR0A) & (1 << 7 as u8) !=0 {
				// Wait...
			}			

			// Read UDR0 to release the contents of RXB
			core::ptr::read_volatile(Self::UDR0);
		}
	}
}

// pg 276 - register descr and pg 159

//pg 148 usart0

// GET THE PROJECT ON GITHUB ASAP