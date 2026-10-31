//! A safe, bare-metal driver for the AVR-Atmega328p's TWI (Master Transmitter driver), using the Atmega328p datasheet registers

// Copyright (c) 2026 [Darell Ethan Kiganga]
// SPDX-License-Identifier: MIT

/// AVR-Atmega328p TWI (2-Wire interface) compatible with I2C
pub struct TWI {
    address: u8,
    _priv: (),
}

// STATUS CODES

// 0x08, // Start condition is transmitted
// 0x10, // Repeat start condition is transmitted
// 0x18, // Slave write has been transmitted, ACK received
// 0x20, // Slave write has been transmitted, NOT ACK has been received
// 0x28, // Data byte has been transmitted, ACK received
// 0x30, // Data byte has been transmitted, NOT ACK received
// 0x38, // Arbitration lost in Slave write or data bytes

static mut TWI_TAKEN: bool = false;

impl TWI {
    const PRR: *mut u8 = 0x64 as *mut u8; // Power reduction register
    const TWBR: *mut u8 = 0xB8 as *mut u8; // TWI Bit rate register
    const TWCR: *mut u8 = 0xBC as *mut u8; // TWI Control register
    const TWSR: *mut u8 = 0xB9 as *mut u8; // TWI Status register
    const TWDR: *mut u8 = 0xBB as *mut u8; // TWI Data register

    /// Take ownership of the TWI
    pub fn take() -> Option<Self> {
        unsafe {
            if TWI_TAKEN {
                None
            } else {
                TWI_TAKEN = true;

                let address_initial = 0x0_u8;

                Some(TWI {
                    address: address_initial,
                    _priv: ()
                })
            }
        }
    }

    /// Initialise the TWI
    pub fn init(&self) {
        unsafe {
            // Read the PRR (Power reduction register)
            let prr_state = core::ptr::read_volatile(Self::PRR);

            core::ptr::write_volatile(Self::PRR, prr_state & !(1 << 7_u8));

            // Set the prescaler bits to max out the speed
            // Prescaler 1, 4, 16 or 64
            // With Prescaler 1, TWBR = 12
            // TWBR = CPU_FREQ - (16 * SCL_FREQ) / (2 * SCL_FREQ * Prescaler)
            // With prescaler 1 and frequency of 400kHz, TWBR = 12

            let read_twsr = core::ptr::read_volatile(Self::TWSR);

            // Use prescaler 1 with TWBR of 12
            core::ptr::write_volatile(Self::TWSR, read_twsr & !(0x03));
            
            // Load the TWBR value
            core::ptr::write_volatile(Self::TWBR, 12_u8);

            // Turn on TWI
            core::ptr::write_volatile(Self::TWCR, 1 << 2_u8);
        }
    }

    /// Send a START condition
    pub fn start(&self) {
        unsafe {
            // Send a START condition
            core::ptr::write_volatile(Self::TWCR, (1 << 2_u8) | (1 << 5_u8) | (1 << 7_u8));

            // Check if the job is finished
            while core::ptr::read_volatile(Self::TWCR) & (1 << 7_u8) == 0 {
                // Wait since to clear it it has to be 1
            }

            // Check if the status code is the one expected, otherwise panic
            if (core::ptr::read_volatile(Self::TWSR) & 0xF8) != 0x08 {
                panic!();
            }
        }
    }

    /// # Transmit Slave write in order to enter Master transmitter mode.
    ///
    /// This function takes an address with its 7 address bits and the Read/Write bit.
    /// # Please Read:
    /// - There's no need to use the address when it's right shifted by 1. e.g, `0x78`.
    /// - (0111 1000) right shifted by one is `0x3C` (0011 1100)
    /// - Since this function takes the whole address and does it automatically. You can use `0x78` directly
    pub fn write_packet(&mut self, address: u8) {
        // Packet is 9 bits (Address = 7 bits, 1 Read/Write bit, 1 achnowledge bit)

        self.address = address;

        unsafe {
            // Write the slave address write to TWI Data register
            core::ptr::write_volatile(Self::TWDR, address);

            // Clear the TWINT bit
            core::ptr::write_volatile(Self::TWCR, (1 << 2_u8) | (1 << 7_u8));

            // Check if the job is finished
            while core::ptr::read_volatile(Self::TWCR) & (1 << 7_u8) == 0 {
                // Wait since to clear it it has to be 1
            }

            // Check if the status code is the one expected, otherwise panic
            if (core::ptr::read_volatile(Self::TWSR) & 0xF8) == 0x18 {

                return;
            }

            else if (core::ptr::read_volatile(Self::TWSR) & 0xF8) == 0x20 {
                for _retry in 1..=3 {
                    // Send a STOP condition followed by a start condition for 3 tries
                    core::ptr::write_volatile(Self::TWCR, (1 << 4_u8) | (1 << 5_u8) | (1 << 7_u8) | (1 << 2_u8));

                    // Check if STOP condition is finished
                    while core::ptr::read_volatile(Self::TWCR) & (1 << 4_u8) != 0 {
                        // Wait since to clear it it has to be 1
                    }

                    // Check if the job is finished
                    while core::ptr::read_volatile(Self::TWCR) & (1 << 7_u8) == 0 {
                        // Wait since to clear it it has to be 0
                    }

                    // Check if the status code is the one expected, otherwise panic
                    if (core::ptr::read_volatile(Self::TWSR) & 0xF8) == 0x08 {
                        // Write the slave address write to TWI Data register
                        core::ptr::write_volatile(Self::TWDR, address);

                        // Clear the TWINT bit
                        core::ptr::write_volatile(Self::TWCR, (1 << 2_u8) | (1 << 7_u8));

                        // Check if the job is finished
                        while core::ptr::read_volatile(Self::TWCR) & (1 << 7_u8) == 0 {
                            // Wait since to clear it it has to be 1
                        }

                        // If status code is expected, break to main loop to load address
                        if (core::ptr::read_volatile(Self::TWSR) & 0xF8) == 0x18 {

                            return;
                        }
                    }
                }

                panic!();

            } else if (core::ptr::read_volatile(Self::TWSR) & 0xF8) == 0x38 {
                for _retry in 1..=3 {
                    // Send a START condition
                    self.start();

                    // Check if the status code is the one expected, otherwise panic
                    if (core::ptr::read_volatile(Self::TWSR) & 0xF8) == 0x08 {
                        // Write the slave address write to TWI Data register
                        core::ptr::write_volatile(Self::TWDR, address);

                        // Clear the TWINT bit
                        core::ptr::write_volatile(Self::TWCR, (1 << 2_u8) | (1 << 7_u8));

                        // Check if the job is finished
                        while core::ptr::read_volatile(Self::TWCR) & (1 << 7_u8) == 0 {
                            // Wait since to clear it it has to be 1
                        }

                        // If status code is expected, break to main loop to load address
                        if (core::ptr::read_volatile(Self::TWSR) & 0xF8) == 0x18 {

                            return;
                        }
                    }
                }

                panic!();
            }
        }
    }

    /// Write data bytes after writing the packet
    pub fn write_data(&mut self, data: u8) {
        unsafe {
        // Write data byte to TWDR
        core::ptr::write_volatile(Self::TWDR, data);

        // Clear the TWINT bit
        core::ptr::write_volatile(Self::TWCR, (1 << 7_u8) | (1 << 2_u8));

        // Poll TWINT to ensure write is complete
        while core::ptr::read_volatile(Self::TWCR) & (1 << 7_u8) == 0 {
            // Wait since to clear it it has to be 1
        }

        // Check if the status code is the one expected, otherwise panic
        if (core::ptr::read_volatile(Self::TWSR) & 0xF8) == 0x28 {

            return;

        } else if (core::ptr::read_volatile(Self::TWSR) & 0xF8) == 0x30 {
            for _retry in 1..=3 {
                // Transmit again and check

                // Write data byte to TWDR
                core::ptr::write_volatile(Self::TWDR, data);

                // Clear the TWINT bit
                core::ptr::write_volatile(Self::TWCR, (1 << 7_u8) | (1 << 2_u8));

                // Poll TWINT to ensure write is complete
                while core::ptr::read_volatile(Self::TWCR) & (1 << 7_u8) == 0 {
                    // Wait since to clear it it has to be 1
                }

                if core::ptr::read_volatile(Self::TWSR) & (0xF8) == 0x28 {

                    return;
                }
            }

            panic!();
                
        } else if (core::ptr::read_volatile(Self::TWSR) & 0xF8) == 0x38 {
            for _retry in 1..=3 {
                // Send a START condition
                self.start();

                // Check if the status code is the one expected, otherwise panic
                if (core::ptr::read_volatile(Self::TWSR) & 0xF8) == 0x08 {
                    // Send the slave address to set it into master transmitter mode then send the data bytes
                    self.write_packet(self.address);

                    // Then send the data to slave

                    // Write the slave address write to TWI Data register
                    core::ptr::write_volatile(Self::TWDR, data);

                    // Clear the TWINT bit
                    core::ptr::write_volatile(Self::TWCR, (1 << 2_u8) | (1 << 7_u8));

                    // Check if the job is finished
                    while core::ptr::read_volatile(Self::TWCR) & (1 << 7_u8) == 0 {
                        // Wait since to clear it it has to be 1
                    }

                    // If status code is expected, break to main loop to load address
                    if (core::ptr::read_volatile(Self::TWSR) & 0xF8) == 0x28 {
                                
                        return;

                    } else if (core::ptr::read_volatile(Self::TWSR) & 0xF8) == 0x30 {
                        // Transmit data again

                        // Write data byte to TWDR
                        core::ptr::write_volatile(Self::TWDR, data);

                        // Clear the TWINT bit
                        core::ptr::write_volatile(Self::TWCR, (1 << 7_u8) | (1 << 2_u8));

                        // Poll TWINT to ensure write is complete
                        while core::ptr::read_volatile(Self::TWCR) & (1 << 7_u8) == 0 {
                            // Wait since to clear it it has to be 1
                        }

                        if (core::ptr::read_volatile(Self::TWSR) & 0xF8) == 0x28 {

                            return;
                        }
                    }
                }
            }

            panic!();
        }
    }
}

    /// Send a STOP condition
    pub fn stop(&self) {
        unsafe {
            // Send a STOP condition
            core::ptr::write_volatile(Self::TWCR, (1 << 2_u8) | (1 << 4_u8) | (1 << 7_u8));

            // Check if the job is finished
            while core::ptr::read_volatile(Self::TWCR) & (1 << 4_u8) != 0 {
                
                // Wait since to clear it it has to be 0
            }
        }
    }
}