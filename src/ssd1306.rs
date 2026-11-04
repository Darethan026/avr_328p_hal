//! A safe, bare-metal driver for the SSD1306 0.96" I2C OLED display, 128 by 64 pixels

// Copyright (c) 2026 [Darell Ethan Kiganga]
// SPDX-License-Identifier: MIT

use crate::twi::TWI;

/// SSD1306 Display
pub struct SSD1306 {
    twi: TWI,
    buffer: [u8; 1024],
    _priv: (),
}

static mut SSD1306: bool = false;

impl SSD1306 {
    /// Take ownership of the display (SSD1306)
    pub fn take() -> Option<Self> {
        unsafe {
            if SSD1306 {
                None
            } else {
                SSD1306 = true;

                let twi = TWI::take().unwrap();
                let buffer = [0u8; 1024];

                Some(SSD1306 {
                    buffer,
                    twi,
                    _priv: (),
                })
            }
        }
    }

    /// Initialise the display
    pub fn init(&mut self) {
        self.twi.init();

        self.set_write_mode();

        // Send control byte to signal a comand is incoming
        self.twi.write_data(0x0);

        self.twi.write_data(0xAE); // Turn off the display
        
        self.twi.write_data(0xD5); // Display clock
        self.twi.write_data(0x80); // Display clock

        self.twi.write_data(0xA8); // Multiplex ratio
        self.twi.write_data(0x3F); // Multiplex ratio

        self.twi.write_data(0xD3); // Display offset
        self.twi.write_data(0x00); // Display offset

        self.twi.write_data(0x40); // Start line

        self.twi.write_data(0x81); // Set contrast
        self.twi.write_data(0x80); // Contrast val at 50%

        self.twi.write_data(0x8D); // Charge pump
        self.twi.write_data(0x14); // Charge pump

        self.twi.write_data(0x20); // Horizontal addressing mode
        self.twi.write_data(0x00); // Horizontal addressing mode

        self.twi.write_data(0xA1); // Segment remap

        self.twi.write_data(0xC8); // Reverse COM scan

        self.twi.write_data(0xDA); // COM pin config
        self.twi.write_data(0x12); // COM pin config

        self.twi.write_data(0xD9); // Pre-charge period
        self.twi.write_data(0xF1); // Pre-charge period

        self.twi.write_data(0xDB); // VCOMH deselect level
        self.twi.write_data(0x40); // VCOMH deselect level

        // self.twi.write_data(0xA4); // Show GDDRAM contents

        self.twi.write_data(0xA4); // Normal display

        self.twi.write_data(0x2E); // Deactivate scrolling

        self.twi.stop(); // Stop the TWI transmission

        self.set_write_mode();

        self.twi.write_data(0x40); // Send the control byte for data

        // Write 0s to the display
        for _ in 0..1024 {
            self.twi.write_data(0x00);
        }

        self.twi.stop(); // Stop the transmission

        // Start a new command transmission
        self.set_write_mode();

        self.twi.write_data(0x00); // Enable command mode
        self.twi.write_data(0xAF); // Turn on display
        self.twi.stop(); // Stop the transmission
    }

    /// SSD1306 function to display contents in GDDRAM
    pub fn display(&mut self) {
        self.set_write_mode();

        // Send a command byte
        self.twi.write_data(0x0);

        self.twi.write_data(0xD3); // Set the cursor to 0x0
        self.twi.write_data(0x00); // Set the cursor to page 0 on x = 0

        self.twi.stop(); // Stop the transmission

        self.set_write_mode(); // Set write mode

        self.twi.write_data(0x40); // Send the data byte

        // Write data to the display
        for byte in 0..1024 {
            self.twi.write_data(self.buffer[byte]);
        }

        self.twi.stop(); // Stop the transmission
    }

    pub fn draw_rectangle(&mut self, x1: u8, y1: u8, width_px: u8, height_px: u8) {
        if x1 > 127 || y1 > 63 { 
            return; 
        }

        // let's say the width = 5
        let x2 = x1 + width_px;
        let y2 = y1 + height_px;

        for line_x1 in x1..=x2 {
            self.draw_point(line_x1, y1);
            self.draw_point(line_x1, y2);
        }

        for line_y1 in y1..=y2 {
            self.draw_point(x1, line_y1);
            self.draw_point(x2, line_y1);
        }
    }

    /// SSD1306 function to draw a point on the display
    pub fn draw_point(&mut self, x: u8, y: u8) {
        // Check if the x and y coordinates are valid
        if x > 127 || y > 63 { 
            return; 
        }

        let buffer_index = (128 * (y as usize / 8)) + x as usize; // Find the index of the buffer
        let bit_position = y % 8; // Position Mod 8 in order to find bit position

        self.buffer[buffer_index] |= 1 << bit_position; // Toggle the respective bit
    }

    /// SSD1306 function to set the driver to write mode
    fn set_write_mode(&mut self) {
        // Send a start condition
        self.twi.start();

        // Send the slave address to prepare for a write
        self.twi.write_packet(0x78);
    }

    /// SSD1306 function to clear the display
    pub fn clear_display(&mut self) {
	    self.buffer = [0u8; 1024]; // Clear local memory

        self.set_write_mode(); // Set write mode

        // Send the data byte
        self.twi.write_data(0x40);

        // Write 0s to the display
        for _ in 0..1024 {
            self.twi.write_data(0x0);
        }

        self.twi.stop(); // Stop the transmission
    }
}
// 256-step brightness control. 100% = 256
// 1 = read, 0 = write for R/W bit

// 0x40 is control byte to set to data mode
