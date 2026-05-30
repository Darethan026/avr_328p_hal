# Changelog

Changes made to the **avr_328p_hal** HAL

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
