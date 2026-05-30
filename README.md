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
| **SPI & I2C** |  Undecided     | Future hardware support.                        |
| **LCD**       | ✅ Completed   | JHD659 and compatible Lcd support in 4-bit mode |

### Completed Drivers

I've written the GPIO(General-Purpose Input/Output) drivers that covers all the Data direction registers(`DDRx`) for `PORTB`, `PORTC`, and `PORTD`. It implements setting the pins as input and output, and also setting them high, setting them low, and checking whether they're high. Below are the other implemented drivers and the ones currently in progress, including the ones I've planned.

- **Timers** - Implemented `Timer0` and `Timer1`. I implemented Normal mode and CTC mode for `Timer0`, including the `delay_ms()` method for setting delays. For `Timer1`, I implemented Normal mode, CTC mode, and Mode 14 PWM for PWM. For mode 14 PWM, the ICR1 value is used as top. I'm not planning to implement timer2 or any other modes for timer0 and timer1.

- **USART0** - I've implemented the `USART0` driver and it can be used in Normal Asynchronous mode, Double speed Asynchronous mode, and Master Synchronous mode. Instead of the user calculating the values manually, the `set_baud_rate()` method in the `USART0` driver does the calculation based on the u32 baud rate value used by the user and it finds the UBRRn value to be used to calculate the baud rate. The calculated u32 value is split and cast as a u8, and the high byte is pushed to the USART Baud rate register high (`UBRR0H`), and from the 'discarded' bits, from the least significant bit, 8 bits of the  UBRRn value are pushed to the USART Baud rate register low (`UBRR0L`). The string bug is fixed and the driver can be used efficiently.

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

## Goal of this project...

I'm currently 17, and when I first stumbled upon rust around 3 years ago, I fell in love with the language. I got everything I needed to start learning rust, but the learning curve was steep. At the time I wasn't particularly interested in high-level languages like Python or Java or Javascript, so when I started rust it felt like it was **the** language. This was the first language I actually stuck with and was interested in learning. I started learning it, and got through chapter 1 and reached chapter 3, then I got tired and left it. A couple of months later, the same thing happened. I went back to the book, covered chapter 1 to chapter 3 and even 4, then left it. This happened till early this year (around February), then I decided to go back to the book. I started from chapter 3 since I knew the basics from chapter 1 to chapter 2, and everything clicked. I covered each chapter and made sure I understood before moving on. I finished chapter 3, then started chapter 4 and I started understanding the borrow checker, the ownership rules, references and it became more fun. I then did a couple of personal project mainly focusing on references and borrowing. Once that was over, everything else was a breeze. I covered chapter 5 (structs), then chapter 5 (enums and pattern-matching), till I finished chapter 8. I decided to build this HAL to just bridge Rust's high-level concepts with low-level systems programming, and I've been learning more and more as the days have passed. This project has also been interesting since it's been like a 'knowledge-solidifying' process since I noticed it's all about register manipulation, using the datasheet for all the driver implementations. There's a certain way I break binary numbers in my head to convert them back to base 10, and vice versa, and since I understood that, using the bitwise operators was that, but made easier. I noticed what I was doing in my head to multiply numbers by 2^x, that the extra zeroes added was just numbers shifting left, so the `<<` operator was fun to use. The `>>` operator was like removing the numbers on the left side and 'shrinking' them towards the right. The `|` operator as I understood it was a way to manipulate registers but without affecting other bits on the register and only targeting specific bits. I could have a variable like `val` that reads a register and I want to set the fourth bit. I'd do something similar to `let x = val | (1 << 3);` to write a 1 to the 3rd bit on the register. The `^` operator is like a toggle, that clears the bit if it was 1 and if it was zero it sets it to 1. I also used the `&` operator to check certain bits on a specific register without clearing others.

## References

* [AVR-ATmega328P Official Datasheet](https://ww1.microchip.com/downloads/en/DeviceDoc/Atmel-7810-Automotive-Microcontrollers-ATmega328P_Datasheet.pdf)

* [The Rust Book](https://doc.rust-lang.org/stable/book/)

* The JHD659 datasheet

* The TMP36 Datasheet
