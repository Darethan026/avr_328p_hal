//! A safe, bare-metal driver for the AVR-Atmega328p's ADC, using the Atmega328p datasheet registers

// Copyright (c) 2026 [Darell Ethan Kiganga]
// SPDX-License-Identifier: MIT

#![allow(dead_code)]

use crate::gpio::{self, PortC};

use gpio::PinC;

#[repr(u8)]
pub enum VoltageReference {
	External, // For custom voltage
	AVcc, // Max is the Arduino's 5v
	Internal, // For small signals
}

pub enum InputChannel {
	AnalogPin0,
	AnalogPin1,
	AnalogPin2,
	AnalogPin3,
	AnalogPin4,
	AnalogPin5,
}

#[repr(u8)]
/// ADC clock = (CPU clock) / (Prescaler)
pub enum ADCprescaler {
	Prescaler2 = 1,
	Prescaler4 = 2,
	Prescaler8 = 3,
	Prescaler16 = 4,
	Prescaler32 = 5,
	Prescaler64 = 6,
	Prescaler128 = 7,
}

/// AVR-Atmega328p ADC
pub struct ADC {
	_priv: (),
}

static mut ADC_TAKING: bool = false;

impl ADC {
	const PRR: *mut u8 = 0x64 as *mut u8; // Power Reduction Register
	const ADMUX: *mut u8 = 0x7C as *mut u8; // ADC Multiplexer Selection Register
	const ADCSRA: *mut u8 = 0x7A as *mut u8; // ADC Control and Status Rggister A
	const ADCH: *mut u8 = 0x79 as *mut u8; // ADC Data Register High
	const ADCL: *mut u8 = 0x78 as *mut u8; // ADC Data Register Low

	/// Take ADC
    pub fn take() -> Option<Self> {
        unsafe {
            if ADC_TAKING {
                None
            } else {
                ADC_TAKING = true;
                Some(ADC { _priv: () })
            }
        }
    }

    /// Start the ADC by waking it from the PRR
    pub fn start(&self) {
    	unsafe {
    		// Read the PRR and write a 1 to bit 0 (PRADC)
    		let val = core::ptr::read_volatile(Self::PRR);

    		core::ptr::write_volatile(Self::PRR, val & !(1 << 0 as u8));
    	}
    }

    /// Select input channel (Pin)
    pub fn select_input_channel(&self, channel: InputChannel) {
    	unsafe {
    		// Read the ADMUX register
    		let val = core::ptr::read_volatile(Self::ADMUX);
    		
    		let analog_pins = PortC::take().unwrap();

    		// Select channel
    		match channel {
    			InputChannel::AnalogPin0 => {
    				//analog_pins.set_input(PinC::PC0, false);

    				core::ptr::write_volatile(Self::ADMUX, val & !(1 << 0 as u8) & !(1 << 1 as u8) & !(1 << 2 as u8) & !(1 << 3 as u8));
    			}

    			InputChannel::AnalogPin1 => {
    				analog_pins.set_input(PinC::PC1, false);

    				core::ptr::write_volatile(Self::ADMUX, val & !(1 << 1 as u8) & !(1 << 2 as u8) & !(1 << 3 as u8) | (1 << 0 as u8));
    			}

    			InputChannel::AnalogPin2 => {
    				analog_pins.set_input(PinC::PC2, false);

    				core::ptr::write_volatile(Self::ADMUX, val & !(1 << 0 as u8) & !(1 << 1 as u8) & !(1 << 3 as u8) | (1 << 1 as u8));
    			}

    			InputChannel::AnalogPin3 => {
    				analog_pins.set_input(PinC::PC3, false);

    				core::ptr::write_volatile(Self::ADMUX, val & !(1 << 1 as u8) & !(1 << 3 as u8) | (1 << 0 as u8) | (1 << 1 as u8));
    			}

    			InputChannel::AnalogPin4 => {
    				analog_pins.set_input(PinC::PC4, false);

    				core::ptr::write_volatile(Self::ADMUX, val & !(1 << 0 as u8) & !(1 << 1 as u8) & !(1 << 3 as u8) | (1 << 2 as u8));
    			}

    			InputChannel::AnalogPin5 => {
    				analog_pins.set_input(PinC::PC5, false);

    				core::ptr::write_volatile(Self::ADMUX, val & !(1 << 1 as u8) & !(1 << 3 as u8) | (1 << 0 as u8) | (1 << 2 as u8));
    			}
    		}
    	}
    }

    /// Set voltage reference after selecting input channel
    pub fn set_reference(&self, reference: VoltageReference) {
    	unsafe {
    		match reference {
    			VoltageReference::External => {
    				// Read the register value
    				let refr = core::ptr::read_volatile(Self::ADMUX);

    				// Set the voltage reference
    				core::ptr::write_volatile(Self::ADMUX, refr & !(1 << 6 as u8) & !(1 << 7 as u8));
    			}

    			VoltageReference::AVcc => {
    				// Read the register value
    				let refr = core::ptr::read_volatile(Self::ADMUX);

    				// Set the voltage reference
    				core::ptr::write_volatile(Self::ADMUX, refr & !(1 << 7 as u8) | (1 << 6 as u8));
    			}

    			VoltageReference::Internal => {
    				// Read the register value
    				let refr = core::ptr::read_volatile(Self::ADMUX);

    				// Set the voltage reference
    				core::ptr::write_volatile(Self::ADMUX, refr | (1 << 6 as u8) | (1 << 7 as u8));
    			}
    		}
    	}
    }

    /// Enable ADC and start conversion
    pub fn start_conversion(&self, prescaler: ADCprescaler) -> u16 {
    	unsafe {
    		// Read the register and enable ADEN
    		let val = core::ptr::read_volatile(Self::ADCSRA);

    		core::ptr::write_volatile(Self::ADCSRA, val | (1 << 7 as u8));

    		match prescaler {
    			ADCprescaler::Prescaler2 => {
    				let prcl2 = core::ptr::read_volatile(Self::ADCSRA);

    				core::ptr::write_volatile(Self::ADCSRA, prcl2 & !(1 << 1 as u8) & !(1 << 2 as u8) | (1 << 0 as u8) | (1 << 6 as u8));
    			}

    			ADCprescaler::Prescaler4 => {
    				let prcl4 = core::ptr::read_volatile(Self::ADCSRA);

    				core::ptr::write_volatile(Self::ADCSRA, prcl4 & !(1 << 0 as u8) & !(1 << 2 as u8) | (1 << 1 as u8) | (1 << 6 as u8));
    			}

    			ADCprescaler::Prescaler8 => {
    				let prcl8 = core::ptr::read_volatile(Self::ADCSRA);

    				core::ptr::write_volatile(Self::ADCSRA, prcl8 & !(1 << 2 as u8) | (1 << 0 as u8) | (1 << 1 as u8) | (1 << 6 as u8));
    			}

    			ADCprescaler::Prescaler16 => {
    				let prcl16 = core::ptr::read_volatile(Self::ADCSRA);

    				core::ptr::write_volatile(Self::ADCSRA, prcl16 & !(1 << 0 as u8) & !(1 << 1 as u8) | (1 << 2 as u8) | (1 << 6 as u8));
    			}

    			ADCprescaler::Prescaler32 => {
    				let prcl32 = core::ptr::read_volatile(Self::ADCSRA);

    				core::ptr::write_volatile(Self::ADCSRA, prcl32 & !(1 << 1 as u8) | (1 << 0 as u8) | (1 << 2 as u8) | (1 << 6 as u8));
    			}

    			ADCprescaler::Prescaler64 => {
    				let prcl64 = core::ptr::read_volatile(Self::ADCSRA);

    				core::ptr::write_volatile(Self::ADCSRA, prcl64 & !(1 << 0 as u8) | (1 << 1 as u8) | (1 << 2 as u8) | (1 << 6 as u8));
    			}

    			ADCprescaler::Prescaler128 => {
    				let prcl128 = core::ptr::read_volatile(Self::ADCSRA);

    				core::ptr::write_volatile(Self::ADCSRA, prcl128 | (1 << 0 as u8) | (1 << 1 as u8) | (1 << 2 as u8) | (1 << 6 as u8));
    			}
    		}

    		while core::ptr::read_volatile(Self::ADCSRA) & (1 << 6 as u8) !=0 {
    			// Wait...
    		}

    		// Check the result
    		core::ptr::read_volatile(Self::ADCL);
    		core::ptr::read_volatile(Self::ADCH);

    		// Trigger another conversion then save the result
    		core::ptr::write_volatile(Self::ADCSRA, (1 << 6 as u8) | (1 << 7 as u8) | prescaler as u8);

    		// Check for the accurate value then put it in a variable
    		while core::ptr::read_volatile(Self::ADCSRA) & (1 << 6 as u8) !=0 {
    			// Wait...
    		}

    		let low = core::ptr::read_volatile(Self::ADCL);
    		let high = core::ptr::read_volatile(Self::ADCH);

    		// Store the result in a variable
    		let result = (high as u16) << 8 | (low as u16);

    		result
    	}
    }
}

// pg 205

// pg 217 registers