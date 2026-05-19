# avr_328p_hal: An Avr HAL for the Arduino Uno R3

After reaching chapter 8 of the rust book a couple of weeks ago, I decided to take a short break, because I thought of working on a project that had been on my mind for a while - this bare-metal Hardware Abstraction Layer(HAL) in the rust programming language, just to understand how the hardware works underneath all these libraries.

## Current Status

| Feature       | Status         | Description                                     |
|:--------------|:---------------|:------------------------------------------------|
| **GPIO**      | ✅ Completed   | DDRx for PORTB, PORTC, and PORTD                |
| **Timer0**    | ✅ Completed   | Normal and CTC modes with `delay_ms()`.         |
| **Timer1**    | ✅ Completed   | Normal, CTC, and Mode 14 PWM.                   |
| **USART0**    | ✅ Completed   | Async/Sync modes with auto-baud calculation.    |
| **ADC**       | ✅ Completed   | Analog-to-Digital driver.                       |
| **SPI & I2C** |  Undecided     | Undecided.                                      |
| **LCD**       | ✅ Completed   | JHD659 and compatible Lcd support in 4-bit mode |

### Completed Drivers

I've written the GPIO(General-Purpose Input/Output) drivers that covers all the Data direction registers(`DDRx`) for `PORTB`, `PORTC`, and `PORTD`. It implements setting the pins as input and output, and also setting them high, setting them low, and checking whether they're high. Below are the other implemented drivers and the ones currently in progress, including the ones I've planned.

- **Timers** - Implemented `Timer0` and `Timer1`. I implemented Normal mode and CTC mode for `Timer0`, including the `delay_ms()` method for setting delays. For `Timer1`, I implemented Normal mode, CTC mode, and Mode 14 PWM for PWM. For mode 14 PWM, the ICR1 value is used as top. I'm not planning to implement timer2 or any other modes for timer0 and timer1.

- **USART0** - I've implemented the `USART0` driver and it can be used in Normal Asynchronous mode, Double speed Asynchronous mode, and Master Synchronous mode. Instead of the user calculating the values manually, the `set_baud_rate()` method in the `USART0` driver does the calculation based on the u32 baud rate value used by the user and it finds the UBRRn value to be used to calculate the baud rate. The calculated u32 value is split and cast as a u8, and the high byte is pushed to the USART Baud rate register high (`UBRR0H`), and from the 'discarded' bits, from the least significant bit, 8 bits of the  UBRRn value are pushed to the USART Baud rate register low (`UBRR0L`). After a lot of testing and troubleshooting, I found an issue, not with the driver, but with how the linker file maps the memory. Due to how the Atmega328p maps memory, flash and RAM have the same address but are stored at different location. The lack of me using a `startup.rs` file to manage the ram locations and how data is moved from flash to ram, any character more than one being handled by the `send_string()` method causes a bug where it tries to read its value from RAM, but instead of finding the `&str`, which contains more than one character, it looks at an area of ram where the string slice hasn't been moved from flash to RAM. Because of this, it's uninitialised, and therefore any character more than one being sent at once using the `send_string()` method shows garbage on the serial line, although it works with single characters, or the same character repeted whatever the number of times. This is due to using a basic linker file as everything else like initialising the memory with a `startup.rs` file isn't happening due to its complexity in bare-metal Rust, which is still new and a Tier 3 target, so this is the basic workaround to make the HAL work.

- **ADC** - The ADC driver is complete! Unfortunately I can't show code examples for how I use it since I can't send string values or integer values using my `USART0` driver since as I mentioned, I don't have a `startup.rs` file to move variables from flash to ram, thus I personally can't view the values on mine, unless I'm viewing `hexadecimal` values on a particular serial monitor. But fortunately for you, if you have a startup file that initialises the variables from flash to ram, then you can view the variables on your serial monitor. (**NOTE:** I've tested the driver and it works flawlessly and can be used for ADC.)

- **LCD** - Implemented a driver for the JHD659 and compatible clones and LCDs like the Hitachi HD44780 LCD. The initialisation sequence is very accurate from the powering up and the 15ms delay, till the transition from 8-bit to 4-bit mode with delays mapped accurately. Everything is wrapped, but under the hood 2 nibbles are sent as the driver operates in 4-bit mode, and is wrapped in a `write_char()` method for characters and a `print_number()` method for printing numbers. Because of the memory initialisation issues for data, the method uses a stack buffer that allocates a fixed number of maximum digits (5) before the program starts so that words are printed from flash, since they're stored on the stack and are not heap allocated. There's a `clear_display()` method for clearing the Lcd display, using the correct timing of 2ms with delays in the driver at various places to handle the speed and make sure data is captured accurately and nothing is skipped.

**NOTE:** - The LCD driver operates in 4-bit mode and maps pins: `VSS` ,`VDD`, `V0`, `RS`, `R/W`, `E`, `DB4`, `DB5`, `DB6`, `DB7`, `LED+`, `LED-` to the following pins respectively: `Common GND`, `5v`, `Arduino Uno R3 PD4`, `Arduino Uno R3 PD5`, `Arduino Uno R3 PD6`, `Arduino Uno R3 PD7`, `Arduino Uno R3 PB0`, `Arduino Uno R3 PB1`, `Arduino Uno R3 PB2`, `Arduino Uno R3 PB3`, `5v in series with a 220 Ohm resistor`, `Common GND`. ***Any other digital pin configurations won't work as these are hardcoded.*** The LCD optionally uses a 100uf capacitor with it's positive side in parallel with a `10k potentiometer` 5v pin, and the negative side connected to the potentiometer's `GND` pin leading to the `Common GND`.

### Drivers in progress

- **SPI** - Undecided

- **I2C(TWI)** - Undecided

## Features

- **Zero dependencies:** This project does not use the standard library or HAL crates.

- **Encapsulation:** There are struct implementations for the drivers and inside them methods including public functions encapsulating the actual work happening in the unsafe functions.

- **Register-level control:** This project purely works by accessing the memory mapped registers on the board using the avr-atmega328p datasheet as a reference, and doesn't have overhead since it doesn't use any external libraries.

- **Reusability:** Since I wrote the drivers and implemented the methods to use them, instead of mapping the registers in the different drivers again - and even for the projects, I used the methods in the implementations instead of having redundant code.

## Prerequisites

- **rustup**
- **rustup nightly**
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

* The TMP36 Datasheet
