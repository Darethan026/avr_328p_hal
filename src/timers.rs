//! A safe, bare-metal driver for the ATmega328P's timers written in bare metal rust, using the Atmega328p datasheet registers

// Copyright (c) 2026 [Darell Ethan Kiganga]
// SPDX-License-Identifier: MIT

#![allow(dead_code)]

/// Prescaler and their values set
#[repr(u8)]
pub enum Prescaler {
    NoClockSource = 0,
    NoPrescaling = 1,
    Prescaler8 = 2,
    Prescaler64 = 3,
    Prescaler256 = 4,
    Prescaler1024 = 5,
}

/// AVR-Atmega328p Timer0
pub struct Timer0 {
    _priv: (),
}
/// AVR-Atmega328p Timer1
pub struct Timer1 {
    _priv: (),
}

static mut TIMER0_TAKING: bool = false;
static mut TIMER1_TAKING: bool = false;

pub mod timer0 {
    use super::{Prescaler, TIMER0_TAKING, Timer0};

    impl Timer0 {
        const PRR: *mut u8 = 0x64 as *mut u8; // Power reduction register
        const TCCR0A: *mut u8 = 0x44 as *mut u8;
        const TCCR0B: *mut u8 = 0x45 as *mut u8;
        const OCR0A: *mut u8 = 0x47 as *mut u8;
        const OCR0B: *mut u8 = 0x48 as *mut u8;
        const TIFR0: *mut u8 = 0x35 as *mut u8; // Timer0 interrupt register
        const TCNT0: *mut u8 = 0x46 as *mut u8; // Timer register
        const TIMSK0: *mut u8 = 0x6E as *mut u8;

        /// Take timer0
        pub fn take() -> Option<Self> {
            unsafe {
                if TIMER0_TAKING {
                    None
                } else {
                    TIMER0_TAKING = true;
                    Some(Timer0 { _priv: () })
                }
            }
        }

        /// Starting the counter module
        pub fn start(&self) {
            unsafe {
                // Reading the PRR (Power reduction register state)
                let val = core::ptr::read_volatile(Self::PRR);

                // Writing 0 to bit 5 (PRTIM0) to start the timer0 module
                core::ptr::write_volatile(Self::PRR, val & !(1 << 5 as u8));
            }
        }

        /* NORMAL MODE METHODS */

        /// Set timer0 to normal mode after starting
        pub fn set_normal_mode(&self) {
            unsafe {
                // Read TCCR0A
                let val = core::ptr::read_volatile(Self::TCCR0A);
                let other_val = core::ptr::read_volatile(Self::TCCR0B);

                // Clear WGM0 bits for TCCR0A
                core::ptr::write_volatile(Self::TCCR0A, val & !(1 | 2));
                core::ptr::write_volatile(Self::TCCR0B, other_val & !(1 << 3 as u8));
            }
        }

        /// Select your desired prescaler based on the already defined ones
        pub fn set_prescaler(&self, prescaler: Prescaler) {
            unsafe {
                // Read TCCR0B
                let val = core::ptr::read_volatile(Self::TCCR0B);

                // Clear all CS0 bits
                let clear = val & !(1 << 0 as u8 | 1 << 1 as u8 | 1 << 2 as u8);

                // Clear flags
                core::ptr::write_volatile(Self::TIFR0, 1 << 1 as u8);

                // Set clock source
                core::ptr::write_volatile(Self::TCCR0B, clear | prescaler as u8);
            }
        }

        /// Wait for Normal mode delay
        pub fn wait(&self) {
            unsafe {
                // Check the timer and wait for it then set the flag once it's done counting
                while (core::ptr::read_volatile(Self::TIFR0) & 1) == 0 {
                    // Wait... 
                }

                core::ptr::write_volatile(Self::TIFR0, 1);
            }
        }

        /* CTC MODE METHODS */

        /// Set timer0 CTC mode
        pub fn set_ctc_mode(&self) {
            unsafe {
                // Read TCCR0A and TCCR0B
                let val = core::ptr::read_volatile(Self::TCCR0A);
                let other_val = core::ptr::read_volatile(Self::TCCR0B);


                // Set the mode
                core::ptr::write_volatile(Self::TCCR0A, val & !(1 << 0 as u8) | (1 << 1 as u8));
                core::ptr::write_volatile(Self::TCCR0B, other_val & !(1 << 3 as u8));
            }
        }

        /// Set value for OCR0A
        /// Top value = ((clock speed) / (Chosen prescaler * frequency target in Hz) - 1
        /// Top value must be between 0 and 255 as this is an 8 bit timer
        pub fn set_top_value(&self, top: u8) {
            unsafe {
                core::ptr::write_volatile(Self::OCR0A, top);
            }
        }

        /// Wait for OCR0A to match
        pub fn wait_for_match(&self) {
            unsafe {
                while (core::ptr::read_volatile(Self::TIFR0) & (1 << 1 as u8)) == 0 {
                    // Wait... 
                }

                core::ptr::write_volatile(Self::TIFR0, 1 << 1 as u8);
            }
        }

        /// Assumes a 16Mhz clock speed 
        /// Delays for the specified number of milliseconds used as an argument for 'ms'
        /// Uses Timer0 in CTC mode with Prescaler 64 and a TOP value of 249.
        pub fn delay_ms(&self, ms: u16) {
            self.set_ctc_mode();
            self.set_top_value(249);
            self.set_prescaler(Prescaler::Prescaler64);

            for _i in 0..ms {
                self.wait_for_match();
            }
        }
    }

    impl Drop for Timer0 {
        fn drop(&mut self) {
            unsafe { TIMER0_TAKING = false; }
        }
    }
}

pub mod timer1 {
    use super::{Prescaler, TIMER1_TAKING, Timer1};

    impl Timer1 {
        const PRR: *mut u8 = 0x64 as *mut u8; // Power reduction register
        const TCCR1A: *mut u8 = 0x80 as *mut u8;
        const TCCR1B: *mut u8 = 0x81 as *mut u8;
        const OCR1A: *mut u16 = 0x88 as *mut u16; // Low register
        const OCR1B: *mut u16 = 0x8A as *mut u16; // Low register
        const TIFR1: *mut u8 = 0x36 as *mut u8; // Timer1 interrupt register
        const TCNT1: *mut u16 = 0x84 as *mut u16; // Timer register (Low)
        const TIMSK1: *mut u8 = 0x6F as *mut u8;
        const ICR1: *mut u16 = 0x86 as *mut u16; // ICR1

        pub fn take() -> Option<Self> {
            unsafe {
                if TIMER1_TAKING {
                    None
                } else {
                    TIMER1_TAKING = true;
                    Some(Timer1 { _priv: () })
                }
            }
        }

        /// Starting the counter module
        pub fn start(&self) {
            unsafe {
                // Reading the PRR (Power reduction register state)
                let val = core::ptr::read_volatile(Self::PRR);

                // Writing 0 to bit 3 (PRTIM1) to start the timer1 module
                core::ptr::write_volatile(Self::PRR, val & !(1 << 3 as u8));
            }
        }

        pub fn set_normal_mode(&self) {
            unsafe {
                // Read TCCR1A and TCCR1B
                let val = core::ptr::read_volatile(Self::TCCR1A);
                let other_val = core::ptr::read_volatile(Self::TCCR1B);

                // Clear WGMx bits for TCCR1A and TCCR1B
                core::ptr::write_volatile(Self::TCCR1A, val & !(1 << 0 as u8 | 1 << 1 as u8));
                core::ptr::write_volatile(Self::TCCR1B, other_val & !(1 << 3 as u8 | 1 << 4 as u8));
            }
        }

        /// Select your prescaler, based on the ones implemented, from no clocksource, till prescaler 1024
        pub fn set_prescaler(&self, prescaler: Prescaler) {
            unsafe {
                // Read TCCR1B
                let val = core::ptr::read_volatile(Self::TCCR1B);

                // Clear all CS1 bits
                let clear = val & !(1 << 0 as u8 | 1 << 1 as u8 | 1 << 2 as u8);

                // Clear flags
                core::ptr::write_volatile(Self::TIFR1, 1 << 1 as u8);

                // Set clock source
                core::ptr::write_volatile(Self::TCCR1B, clear | prescaler as u8);
            }
        }

        /// Wait for Normal mode delay on timer1
        pub fn wait(&self) {
            unsafe {
                while (core::ptr::read_volatile(Self::TIFR1) & 1) == 0 {
                    // Wait... 
                }

                core::ptr::write_volatile(Self::TIFR1, 1);
            }
        }

        /// Set CTC mode for timer1
        pub fn set_ctc_mode(&self) {
            unsafe {
                // Read TCCR0A and TCCR0B
                let val = core::ptr::read_volatile(Self::TCCR1A);
                let other_val = core::ptr::read_volatile(Self::TCCR1B);

                // Set the mode
                core::ptr::write_volatile(Self::TCCR1A, val & !(1 << 0 as u8) & !(1 << 1 as u8));
                core::ptr::write_volatile(Self::TCCR1B, other_val & !(1 << 4 as u8) | (1 << 3 as u8));
            }
        }

        // Wait for OCR1A to match
        pub fn wait_for_match(&self) {
            unsafe {
                while (core::ptr::read_volatile(Self::TIFR1) & (1 << 1 as u8)) == 0 {
                    // Wait... 
                }

                core::ptr::write_volatile(Self::TIFR1, 1 << 1 as u8);
            }
        }

        /// Set value for OCR1A
        /// Top value (top) = ((clock speed) / (Chosen prescaler * frequency target in Hz) - 1.
        /// For time, the formula is OCR1 = ((Clock speed * target in seconds) / Prescaler) - 1.
        /// Must be less than or equal to ICR1
        pub fn set_top_value(&self, top: u16) {
            unsafe {
                core::ptr::write_volatile(Self::OCR1A, top);
            }
        }

        /// Set Fast PWM Mode 14
        /// ICR1 (top) = ((cpu frequency in Hz) / (prescaler * desired frequency in Hz)) - 1
        /// ICR1 must be between 0 and 65535
        pub fn set_fast_pwm(&self, top: u16) {
            unsafe {
                // Clear flags before starting
                core::ptr::write_volatile(Self::TIFR1, 1 << 1 as u8);

                // Read TCCR1A and TCCR1B
                let val = core::ptr::read_volatile(Self::TCCR1A);
                let other_val = core::ptr::read_volatile(Self::TCCR1B);
                let clear = val & !(1 << 0 as u8) & !(1 << 1 as u8) & !(1 << 6 as u8) & !(1 << 7 as u8);
                let other_clear = other_val & !(1 << 3 as u8) & !(1 << 4 as u8);

                // Configure WGM for Mode 14 Fast PWM while clearing some bits
                core::ptr::write_volatile(Self::TCCR1A, clear | (1 << 1 as u8) | 1 << 7 as u8);

                core::ptr::write_volatile(Self::TCCR1B, other_clear | (1 << 3 as u8) | (1 << 4 as u8));

                core::ptr::write_volatile(Self::ICR1, top);
            }
        }

        /// Since the top value for OCR1A is automatically calculated as it reads the current ICR1 value, you don't need to put the value here
        /// Set your duty cycle in %. Eg, using '50' as an argument
        pub fn set_duty_cycle(&self, duty_in_percentage: u16) {
            unsafe {
                // Read the ICR1 register to find the top value
                let val = core::ptr::read_volatile(Self::ICR1);

                let safe_duty = if duty_in_percentage > 100 {
                    100

                } else {
                    duty_in_percentage
                };

                // Calculate the OCR1A value based on the ICR1 value got by the duty in percentage
                let top_val_for_ocr1a = (safe_duty as u32 * (val as u32 + 1)) / 100;

                core::ptr::write_volatile(Self::OCR1A, top_val_for_ocr1a as u16);
            }
        }

        /// Assumes a 16Mhz clock speed 
        /// Sets the servo angle (0-180).
        /// Timer1 is in PWM Mode 14 with a 16MHz clock and assumes Prescaler 8 for the best accuracy.
        pub fn set_servo_angle(&self, angle: u16) {
            unsafe {
                let val = core::ptr::read_volatile(Self::ICR1);

                let safe_angle = if angle > 180 {
                    180

                } else {
                    angle
                };
                
                let min_ticks_ms = (val as u32 + 1) / 20; // For 0 degrees
                let max_ticks_2ms = (val as u32 + 1) / 10; // For 180 degrees

                let ticks_per_degree_ocr1a = min_ticks_ms + (safe_angle as u32 * (max_ticks_2ms - min_ticks_ms)) / 180; // value for ocr1a

                core::ptr::write_volatile(Self::OCR1A, ticks_per_degree_ocr1a as u16);
            }
        }
    }

    impl Drop for Timer1 {
        fn drop(&mut self) {
            unsafe { TIMER1_TAKING = false; }
        }
    }
}