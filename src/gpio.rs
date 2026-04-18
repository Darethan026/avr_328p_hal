//! A safe, bare-metal driver for the AVR-Atmega328p's GPIO pins, using the Atmega328p datasheet registers

// Copyright (c) 2026 [Darell Ethan Kiganga]
// SPDX-License-Identifier: MIT

#![allow(dead_code)]
/// Define the Pins on PortB starting from digital pin 8 (PB0) and ending with digital pin 13 (PB5)
#[repr(u8)]
#[derive(Copy, Clone)]
pub enum PinB {
    PB0 = 0, // Digital pin 8
    PB1 = 1, // Digital pin 9
    PB2 = 2, // Digital pin 10
    PB3 = 3, // Digital pin 11
    PB4 = 4, // Digital pin 12
    PB5 = 5, // Digital pin 13
}

/// Define the Pins on PortC starting from analog pin 0 (PC0) and ending with analog pin 5 (PC5)
#[repr(u8)]
#[derive(Copy, Clone)]
pub enum PinC {
    PC0 = 0, // Analog pin 0(A0)--|-- (Analog-to-Digital converter)
    PC1 = 1, // Analog pin 1(A1)  |-- (Analog-to-Digital converter)
    PC2 = 2, // Analog pin 2(A2)  |-- (Analog-to-Digital converter)
    PC3 = 3, // Analog pin 3(A3)  |-- (Analog-to-Digital converter)
    PC4 = 4, // Analog pin 4(A4)  |-- (Analog-to-Digital converter)
    PC5 = 5, // Analog pin 5(A5)--|-- (Analog-to-Digital converter)
}

/// Define the Pins on PortD starting from digital pin 0 (PD0) and ending with digital pin 7 (PD7)
#[repr(u8)]
#[derive(Copy, Clone)]
pub enum PinD {
    PD0 = 0, // Digital pin 0 and Serial (RX)
    PD1 = 1, // Digital pin 1 and Serial (TX)
    PD2 = 2, // Digital pin 2
    PD3 = 3, // Digital pin 3
    PD4 = 4, // Digital pin 4
    PD5 = 5, // Digital pin 5
    PD6 = 6, // Digital pin 6
    PD7 = 7, // Digital pin 7
}

/// PortB implementation, holding Pins on PortB from PB0 till PB5
pub struct PortB {
    _priv: (),
}
/// PortC implementation, holding Pins on PortC from PC0 till PC5
pub struct PortC {
    _priv: (),
}
/// PortD implementation, holding Pins on PortD from PD0 till PD7
pub struct PortD {
    _priv: (),
}

static mut B_TAKING: bool = false;
static mut C_TAKING: bool = false;
static mut D_TAKING: bool = false;

impl PortB {
    const DDRB: *mut u8 = 0x24 as *mut u8;
    const PINB: *mut u8 = 0x23 as *mut u8;
    const PORTB: *mut u8 = 0x25 as *mut u8;

    /// Take ownership of the port
    pub fn take() -> Option<Self> {
        unsafe {
            if B_TAKING {
                None
            } else {
                B_TAKING = true;
                Some(PortB { _priv: () })
            }
        }
    }

    /// Toggle a pin specified used as an argument for 'pin'
    pub fn toggle(&self, pin: PinB) {
        unsafe {
            core::ptr::write_volatile(Self::PINB, 1 << pin as u8);
        }
    }

    /// Set the Pin as output
    pub fn set_output(&self, pin: PinB) {
        unsafe {
            // Read the pin state
            let val = core::ptr::read_volatile(Self::DDRB);

            // Write to the Pin
            core::ptr::write_volatile(Self::DDRB, val | 1 << pin as u8);
        }
    }

    /// Set the pin as an input and for pullup use a bool value, ie, true or false
    pub fn set_input(&self, pin: PinB, use_pullup: bool) {
        unsafe {
            // Read the Port state
            let port_val = core::ptr::read_volatile(Self::PORTB);

            if use_pullup {
                core::ptr::write_volatile(Self::PORTB, port_val | (1 << pin as u8));
            } else {
                core::ptr::write_volatile(Self::PORTB, port_val & !(1 << pin as u8));
            }

            // Read pin state (Output or Input)
            let val = core::ptr::read_volatile(Self::DDRB);

            // Write a 0 to the pin to change it to input
            core::ptr::write_volatile(Self::DDRB, val & !(1 << pin as u8));
        }
    }

    /// Set the pin high
    pub fn set_high(&self, pin: PinB) {
        unsafe {
            // Read the state of the pin
            let val = core::ptr::read_volatile(Self::PORTB);
            // Write to the pin
            core::ptr::write_volatile(Self::PORTB, val | 1 << pin as u8);
        }
    }

    /// Set the pin low
    pub fn set_low(&self, pin: PinB) {
        unsafe {
            // Read the pin state
            let val = core::ptr::read_volatile(Self::PORTB);
            // Write to the pin
            core::ptr::write_volatile(Self::PORTB, val & !(1 << pin as u8));
        }
    }

    /// Check whether the pin is high
    pub fn is_high(&self, pin: PinB) -> bool {
        unsafe {
            (core::ptr::read_volatile(Self::PINB) & (1 << pin as u8)) != 0
        }
    }
}

impl Drop for PortB {
    fn drop(&mut self) {
        unsafe {
            B_TAKING = false;
        }
    }
}

impl PortC {
    const DDRC: *mut u8 = 0x27 as *mut u8;
    const PINC: *mut u8 = 0x26 as *mut u8;
    const PORTC: *mut u8 = 0x28 as *mut u8;

    /// Take ownership of the port
    pub fn take() -> Option<Self> {
        unsafe {
            if C_TAKING {
                None
            } else {
                C_TAKING = true;
                Some(PortC { _priv: () })
            }
        }
    }

    /// Toggle the pin
    pub fn toggle(&self, pin: PinC) {
        unsafe {
            core::ptr::write_volatile(Self::PINC, 1 << pin as u8);
        }
    }

    /// Set the Pin as output pin
    pub fn set_output(&self, pin: PinC) {
        unsafe {
            // Read the pin state
            let val = core::ptr::read_volatile(Self::DDRC);

            // Write to the Pin
            core::ptr::write_volatile(Self::DDRC, val | 1 << pin as u8);
        }
    }

    /// Set the pin as an input pin
    pub fn set_input(&self, pin: PinC, use_pullup: bool) {
        unsafe {
            // Read the Port state
            let port_val = core::ptr::read_volatile(Self::PORTC);

            if use_pullup {
                core::ptr::write_volatile(Self::PORTC, port_val | (1 << pin as u8));
            } else {
                core::ptr::write_volatile(Self::PORTC, port_val & !(1 << pin as u8));
            }

            // Read pin state (Output or Input)
            let val = core::ptr::read_volatile(Self::DDRC);

            // Write a 0 to the pin to change it to input
            core::ptr::write_volatile(Self::DDRC, val & !(1 << pin as u8));
        }
    }


    /// Set the pin high
    pub fn set_high(&self, pin: PinC) {
        unsafe {
            // Read the state of the pin
            let val = core::ptr::read_volatile(Self::PORTC);
            // Write to the pin
            core::ptr::write_volatile(Self::PORTC, val | 1 << pin as u8);
        }
    }

    /// Set the pin low
    pub fn set_low(&self, pin: PinC) {
        unsafe {
            // Read the pin state
            let val = core::ptr::read_volatile(Self::PORTC);
            // Write to the pin
            core::ptr::write_volatile(Self::PORTC, val & !(1 << pin as u8));
        }
    }

    /// Check whether the pin is high
    pub fn is_high(&self, pin: PinC) -> bool {
        unsafe {
            (core::ptr::read_volatile(Self::PINC) & (1 << pin as u8)) != 0
        }
    }
}

impl Drop for PortC {
    fn drop(&mut self) {
        unsafe {
            C_TAKING = false;
        }
    }
}

impl PortD {
    const DDRD: *mut u8 = 0x2A as *mut u8;
    const PIND: *mut u8 = 0x29 as *mut u8;
    const PORTD: *mut u8 = 0x2B as *mut u8;

    /// Take ownership of the port
    pub fn take() -> Option<Self> {
        unsafe {
            if D_TAKING {
                None
            } else {
                D_TAKING = true;
                Some(PortD { _priv: () })
            }
        }
    }

    pub fn toggle(&self, pin: PinD) {
        unsafe {
            core::ptr::write_volatile(Self::PIND, 1 << pin as u8);
        }
    }

    /// Set the Pin as output pin
    pub fn set_output(&self, pin: PinD) {
        unsafe {
            // Read the pin state
            let val = core::ptr::read_volatile(Self::DDRD);

            // Write to the Pin
            core::ptr::write_volatile(Self::DDRD, val | 1 << pin as u8);
        }
    }

    /// Set the pin as an input pin and for pullup, use a bool value, ie, true or false
    pub fn set_input(&self, pin: PinD, use_pullup: bool) {
        unsafe {
            // Read the Port state
            let port_val = core::ptr::read_volatile(Self::PORTD);

            if use_pullup {
                core::ptr::write_volatile(Self::PORTD, port_val | (1 << pin as u8));
            } else {
                core::ptr::write_volatile(Self::PORTD, port_val & !(1 << pin as u8));
            }

            // Read pin state (Output or Input)
            let val = core::ptr::read_volatile(Self::DDRD);

            // Write a 0 to the pin to change it to input
            core::ptr::write_volatile(Self::DDRD, val & !(1 << pin as u8));
        }
    }

    /// Set the pin high
    pub fn set_high(&self, pin: PinD) {
        unsafe {
            // Read the state of the pin
            let val = core::ptr::read_volatile(Self::PORTD);
            // Write to the pin
            core::ptr::write_volatile(Self::PORTD, val | 1 << pin as u8);
        }
    }

    /// Set the pin low
    pub fn set_low(&self, pin: PinD) {
        unsafe {
            // Read the pin state
            let val = core::ptr::read_volatile(Self::PORTD);
            // Write to the pin
            core::ptr::write_volatile(Self::PORTD, val & !(1 << pin as u8));
        }
    }

    /// Check whether the pin is high
    pub fn is_high(&self, pin: PinD) -> bool {
        unsafe {
            (core::ptr::read_volatile(Self::PIND) & (1 << pin as u8)) != 0
        }
    }
}

impl Drop for PortD {
    fn drop(&mut self) {
        unsafe {
            D_TAKING = false;
        }
    }
}
