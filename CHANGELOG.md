# Changelog

Changes made to the **avr_328p_hal** HAL

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
