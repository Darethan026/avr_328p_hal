//! A safe, bare-metal driver for the AVR-Atmega328p's EEPROM, using the Atmega328p datasheet registers

// Copyright (c) 2026 [Darell Ethan Kiganga]
// SPDX-License-Identifier: MIT

/// AVR-Atmega328p EEPROM
pub struct EEPROM {
    _priv: (),
}

static mut EEPROM_TAKEN: bool = false;

impl EEPROM {
    const EEARH: *mut u8 = 0x42 as *mut u8; // EEPROM Address register high byte
    const EEARL: *mut u8 = 0x41 as *mut u8; // EEPROM Address register low byte
    const EEDR: *mut u8 = 0x40 as *mut u8; // EEPROM Data register
    const EECR: *mut u8 = 0x3F as *mut u8; // EEPROM Control register
    const SPMCSR: *mut u8 = 0x57 as *mut u8; // Store program memory control and status register
    
    /// Take ownership of the EEPROM
    pub fn take() -> Option<Self> {
        unsafe {
            if EEPROM_TAKEN {
                None
            } else {
                EEPROM_TAKEN = true;

                Some(EEPROM { _priv: () })
            }
        }
    }

    /// Method to write an address (value from 0 to 1023) and data (value from 0 to 255) to EEPROM
    pub fn eeprom_write(&self, address: u16, data: u8) {
        unsafe {
            // Check to see if previous write completed
            while core::ptr::read_volatile(Self::EECR) & (1 << 1_u8) != 0 {
                // Wait till it becomes 0 to signal a write is finished
            }

            while core::ptr::read_volatile(Self::SPMCSR) & (1 << 0_u8) != 0 {
                // Wait till it becomes 0
            }

            // SET UP ADDRESSES

            if address <= 1023 {
                // EEAR8 is an unused bit in ATmega328P and must always be written to zero.
                // Read and store the value of EEARH

                // Get the high byte for EEARH
                let high_byte = (address >> 8) as u8;

                // Get the low byte for EEARL
                let low_byte = address as u8;

                let read_eecr = core::ptr::read_volatile(Self::EECR); // Read EECR value

                // Write to EEAR High byte
                core::ptr::write_volatile(Self::EEARH, high_byte);

                // Write to EEAR Low byte
                core::ptr::write_volatile(Self::EEARL, low_byte);

                // Write EEPROM data to EEDR
                core::ptr::write_volatile(Self::EEDR, data);

                // Write a 1 to EEMPE and a 0 to EEPE in EECR
                core::ptr::write_volatile(Self::EECR, read_eecr & !(1 << 1_u8) | (1 << 2));

                // Write a 1 to EEPE to write to EEPROM
                core::ptr::write_volatile(Self::EECR, read_eecr | (1 << 1_u8));
            } else {
                panic!();
            }
        }
    }

    /// Method to read and return data from EEPROM
    pub fn eeprom_read(&self, address: u16) -> u8 {
        unsafe {
            // Check to see if previous write completed
            while core::ptr::read_volatile(Self::EECR) & (1 << 1_u8) != 0 {
                // Wait till it becomes 0 to signal a write is finished
            }

            // SET UP THE ADDRESS

            if address <= 1023 {
                // Get the high byte for EEARH
                let high_byte = (address >> 8) as u8;

                // Get the low byte for EEARL
                let low_byte = address as u8;

                let read_eecr = core::ptr::read_volatile(Self::EECR); // Read EECR value

                // Write to EEAR High byte
                core::ptr::write_volatile(Self::EEARH, high_byte);

                // Write to EEAR Low byte
                core::ptr::write_volatile(Self::EEARL, low_byte);

                // Read EEPROM

                // Write a 1 to EERE to start EEPROM read
                core::ptr::write_volatile(Self::EECR, read_eecr | (1 << 0_u8));

                // Return data from EEDR
                core::ptr::read_volatile(Self::EEDR)
            } else {
                panic!();
            }
        }
    }
}
