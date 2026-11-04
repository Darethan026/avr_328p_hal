# avr_328p_hal: An Avr HAL for the Arduino Uno R3

After reaching chapter 8 of the rust book a couple of weeks ago (as of `19/04/2026`), I decided to take a short break, because I thought of working on a project that had been on my mind for a while - this bare-metal Hardware Abstraction Layer(HAL) in the rust programming language, just to understand how the hardware works underneath all these libraries.

## Current Status

| Feature                                | Status         | Description                                                                  |
|:---------------------------------------|:---------------|:-----------------------------------------------------------------------------|
| **GPIO**                               | ✅ Completed   | DDRx for PORTB, PORTC, and PORTD                                             |
| **Timer0**                             | ✅ Completed   | Normal and CTC modes with `delay_ms()`.                                      |
| **Timer1**                             | ✅ Completed   | Normal, CTC, and Mode 14 PWM.                                                |
| **USART0**                             | ✅ Completed   | Async/Sync modes with auto-baud calculation.                                 |
| **ADC**                                | ✅ Completed   | Analog-to-Digital driver.                                                    |
| **I2C(TWI)**                           | ✅ Completed   | Implemented TWI(I2C) with status code handling at a speed of 400kHz on 16MHz |
| **LCD**                                | ✅ Completed   | JHD659 and compatible Lcd support in 4-bit mode                              |
| **EEPROM**                             | ✅ Completed   | AVR-ATmega328p EEPROM module to write or read from EEPROM                    |
| **SSD1306 128 x 64 I2C OLED Display**  | ✅ Completed   | SSD1306 128 x 64 OLED I2C display driver                                     |
| **SPI**                                |  Undecided     | Future hardware support.                                                     |

### Completed Drivers

- **GPIO** - The GPIO(General-Purpose Input/Output) driver covers all the Data direction registers(`DDRx`) for `PORTB`, `PORTC`, and `PORTD`. It implements setting the pins as input and output, and also setting them high, setting them low, and checking whether they're high. Below are the other implemented drivers and the ones currently in progress, including the ones I've planned.

- **Timers** - Implemented `Timer0` and `Timer1`. I implemented Normal mode and CTC mode for `Timer0`, including the `delay_ms()` method for setting delays. For `Timer1`, I implemented Normal mode, CTC mode, and Mode 14 PWM for PWM. For mode 14 PWM, the ICR1 value is used as top. I'm not planning to implement timer2 or any other modes for timer0 and timer1.

- **USART0** - I've implemented the `USART0` driver and it can be used in Normal Asynchronous mode, Double speed Asynchronous mode, and Master Synchronous mode. Instead of the user calculating the values manually, the `set_baud_rate()` method in the `USART0` driver does the calculation based on the u32 baud rate value used by the user and it finds the UBRRn value to be used to calculate the baud rate. The calculated u32 value is split and cast as a u8, and the high byte is pushed to the USART Baud rate register high (`UBRR0H`), and from the 'discarded' bits, from the least significant bit, 8 bits of the  UBRRn value are pushed to the USART Baud rate register low (`UBRR0L`). The string bug is fixed and the driver can be used efficiently.

- **ADC** - The ADC driver is complete! Thanks to the new custom written linker script, printing values is possible using `USART0`! (**NOTE:** I've tested the driver and it works flawlessly and can be used for ADC.)

- **LCD** - Implemented a driver for the JHD659 and compatible clones and LCDs like the Hitachi HD44780 LCD. The initialisation sequence is very accurate from the powering up and the 15ms delay, till the transition from 8-bit to 4-bit mode with delays mapped accurately. Everything is wrapped, but under the hood 2 nibbles are sent as the driver operates in 4-bit mode, and is wrapped in a `write_char()` method for characters and a `print_number()` method for printing numbers. Character printing is now possible with printing issues resolved with the implementation of a new custom linker script. There's a `clear_display()` method for clearing the Lcd display, using the correct timing of 2ms with delays in the driver at various places to handle the speed and make sure data is captured accurately and nothing is skipped.

- **EEPROM** - AVR-ATmega328p module to write or read to/from the EEPROM.

**NOTE:** - The LCD driver operates in 4-bit mode and maps pins: `VSS` ,`VDD`, `V0`, `RS`, `R/W`, `E`, `DB4`, `DB5`, `DB6`, `DB7`, `LED+`, `LED-` to the following pins respectively: `Common GND`, `5v`, `Arduino Uno R3 PD4`, `Arduino Uno R3 PD5`, `Arduino Uno R3 PD6`, `Arduino Uno R3 PD7`, `Arduino Uno R3 PB0`, `Arduino Uno R3 PB1`, `Arduino Uno R3 PB2`, `Arduino Uno R3 PB3`, `5v in series with a 220 Ohm resistor`, `Common GND`. ***Any other digital pin configurations won't work as these are hardcoded.*** The LCD optionally uses a 100uf capacitor with it's positive side in parallel with a `10k potentiometer` 5v pin, and the negative side connected to the potentiometer's `GND` pin leading to the `Common GND`.

- **TWI** - Completed the TWI master transmitter driver which initialises the driver at `400kHz` on a 16MHz cpu clock with status code handling. In the event of a `NACK` in the case of sending the `SLA+W` or data byte, then it enters a loop of 3 retries which sends a `STOP` followed by a `START` condition. It then sends the `I2C` address again using `write_packet(u8 i2c_address)` and then handles the possible request of which when it receives an `ACK`, it then proceeds to send the `SLA+W`. The function, `write_data(byte: u8)` then sends a byte to the `I2C` slave pressuming that you've already used `write_packet(u8 i2c_address)`, and once you write the data byte, it doesn't stop automatically, so to stop the I2C transmission, you'll then have to use `stop()`. To use it you'll have to use `take()` in order to gain ownership of the driver, then initialise it with `init()`, then use `start()`. After this you'll have to use `write_packet()` to send the I2C address and the `SLA+W` is automatically sent, and then now use `write_data()` to send the data byte and then use `stop()` to end the transmission.

- **SSD1306 128 x 64 OLED I2C display driver** - This driver uses the underlying `TWI` driver and builds on top of it. It can draw a pixel at a certain location on the display using `draw_point(x: u8, y: u8)`, or you could draw a rectangle with `draw_rectangle(x1: u8, y1: u8, width_px: u8, height_px: u8)`, where `x1` is the offset from the boundary of the display from the left where x = 0, and where `y1` is the vertical offset from the boundary at the `top`, where y = 0. You could also clear the display with `clear_display()` which clears the memory and the display remains blank. To use the driver you'll have to use `take()` to gain ownership of the driver, then use `init()` to intialise on a blank display, and draw a point or a rectangle then call `display()`. 

### Drivers in progress

- **SPI** - Undecided

## Features

- **Zero dependencies:** This project does not use the standard library or HAL crates.

- **Encapsulation:** There are struct implementations for the drivers and inside them methods including public functions encapsulating the actual work happening in the unsafe functions.

- **Register-level control:** This project purely works by accessing the memory mapped registers on the board using the avr-atmega328p datasheet as a reference, and doesn't have overhead since it doesn't use any external libraries.

- **Reusability:** Since I wrote the drivers and implemented the methods to use them, instead of mapping the registers in the different drivers again - and even for the projects, I used the methods in the implementations instead of having redundant code.

## Prerequisites

- **rustup**
- **rustup nightly compiler**
- **avrdude**
- **avr-gcc**
- **ravedude** (`cargo install ravedude`)
- **rust-src** (`Install using rustup`)
- **A linker file**

## How to use

### Blink the on-board PB5 Led.

```rust

use avr_328p_hal::gpio;
use avr_328p_hal::timers;

use gpio::*;
use timers::{Timer0, Timer1, Prescaler};

#[unsafe(no_mangle)]
pub extern "C" fn main() -> ! {

    let portb = PortB::take().unwrap();
    let timer = Timer0::take().unwrap();

    portb.set_output(PinB::PB5);
    timer.start();

    loop {
        portb.set_high(PinB::PB5);
        timer.delay_ms(500);
        portb.set_low(PinB::PB5);
    }
}

```
If you want to see more examples, check the **examples/** directory.

**NOTE:** Make sure you run the code in `release` mode with the rust nightly compiler as it compiles `core` and even if you're using nightly, due to problems with compiler builtins, it only works when you compile it in release mode, otherwise you'll get this error: `error: value evaluated as 122104 is out of range.`, along with `error: could not compile compiler_builtins (lib) due to 1 previous error`, even though everything in your code may be correct. It only works if you compile it in release mode. You can check my `Cargo.toml` and `.cargo/config.toml` to see my build configurations, as I did a lot of troubleshooting to find out what works. Instead of using a `target.json`, you can use the `avr-none` rust target for avr, although you still need a linker script.

## References

* [AVR-ATmega328P Official Datasheet](https://ww1.microchip.com/downloads/en/DeviceDoc/Atmel-7810-Automotive-Microcontrollers-ATmega328P_Datasheet.pdf)

* [The Rust Book](https://doc.rust-lang.org/stable/book/)

* The JHD659 datasheet

* The TMP36 datasheet

* [Solomons systech datasheet]()