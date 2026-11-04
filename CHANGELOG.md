# Changelog

Changes made to the **avr_328p_hal** HAL


## [Version 3.0.0] - 2026-11-4

## Breaking changes

- Changed function name `send_u16_binary` in `USART0` to `send_u16_bytes` to improve clarity.

- Changed function name `start` to `init` in the modules: `usart0`, `adc`, and `timers`

- Made function `set_frame_format` and `enable_tx_rx` private and instead call them in the `set_baud_rate(u32_val, mode)` function with `self.set_frame_format();` and `self.enable_tx_rx();` repectively, in order to improve ease of use with the USART0 driver. Now to initialise `USART0`, you only need to take ownership of it with `USART0.take().unwrap` or handle the `Option`, and then set your baud rate with: `set_baud_rate(u32_val, mode)` and use `transmit_char(char)` or `print_string(&str)` or `send_16_bytes(u16_val)` or `print_num(u16)` or even `usart_receive()` (which returns a `u8` value).

- Improved on module documentation

- Reduced code redundancy in all modules

## Added

- `EEPROM` module tested thoroughly. Able to write and read to `EEPROM`. Take ownership with `EEPROM::take()` (Returns an `Option<Self>`), meaning you'll need to handle the `Option`. To write use the `eeprom_write(address: u16, data: u8)` of which the maximum value for `address` is `1023`. For `data` the maximum value is `255` due to it being an `8-bit` variable.

- **TWI** - TWI master transmitter driver which initialises the driver at `400kHz` on a 16MHz cpu clock with status code handling. In the event of a `NACK` in the case of sending the `SLA+W` or data byte, then it enters a loop of 3 retries which sends a `STOP` followed by a `START` condition. It then sends the `I2C` address again using `write_packet(u8 i2c_address)` and then handles the possible request of which when it receives an `ACK`, it then proceeds to send the `SLA+W`. The function, `write_data(byte: u8)` then sends a byte to the `I2C` slave pressuming that you've already used `write_packet(u8 i2c_address)`, and once you write the data byte, it doesn't stop automatically, so to stop the I2C transmission, you'll then have to use `stop()`.

- **SSD1306 128 x 64 OLED I2C display driver** - This driver uses the underlying `TWI` driver and builds on top of it. It can draw a pixel at a certain location on the display using `draw_point(x: u8, y: u8)`, or you could draw a rectangle with `draw_rectangle(x1: u8, y1: u8, width_px: u8, height_px: u8)`, where `x1` is the offset from the boundary of the display from the left where x = 0, and where `y1` is the vertical offset from the boundary at the `top`, where y = 0. You could also clear the display with `clear_display()` which clears the memory and the display remains blank.

## [Version 2.0.0] - 2026-05-30

### Breaking changes
- Changed function name `send_u16` in `USART0` to `send_u16_binary` to improve clarity.

### Added
- New custom, functional and robust linker script `avr-atmega328p_linker.ld` and after doing an `objdump` to dissassemble the final .elf, I discovered that there's no need to write a custom startup routine as the compiler does that automatically based on what I've defined in the linker script.

- Native `print_string()` function for printing text to the serial line on `USART0` for single chars or full &str slices.

- A `print_num()` function for printing `u16` numbers in the `u16` range (0..65535) for `USART0`.

- A `print()` function for the `LCD` driver to print text to the LCD screen.

### Fixed
- `USART0` can now be used to print `&str` directly on the serial line! The issue has been fixed as read only data is being copied from flash to ram by adding `*(.rodata)` and `*(.rodata*)` to the new custom linker script, and they're now initialised at runtime.

## [Version 1.1.0] - 2026-04-24

### Added
**LCD Driver (JHD659/HD44780):**
  - Implemented 4-bit parallel communication mode.
  - Added `init()` sequence with a working "8-bit to 4-bit" synchronization handshake.
  - Implemented `write_char()` with manual DDRAM address tracking to handle 16-character line wrapping after a character reaches the end of the first and last line, along with a `print_number()` method that operates on the stack to bypass the linker issues.
  - Added `clear_display()` with accurate execution delays (5ms).

### Fixed
- **Hardware Stability:** Corrected VSS/VDD power configurations to resolve incorrect LCD initialisation.

- **Timing:** Optimized `enable` delays to meet the JHD659 timing requirements on a 16MHz Uno R3.

## [Version 1.0.0] - 2026-04-21

### Added

- ADC driver for the AVR-Atmega328p

### Fixed

- USART0 `transmit_char()` method to print accurate text on the serial line.
