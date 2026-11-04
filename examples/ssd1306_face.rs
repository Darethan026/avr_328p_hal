#![no_std]
#![no_main]

use core::panic::PanicInfo;

use avr_328p_hal::ssd1306::SSD1306;

#[unsafe(no_mangle)]
pub extern "C" fn main() -> ! {
	let mut oled_display = SSD1306::take().unwrap(); // Take ownership of the display

	oled_display.init(); // Initialise the display

	oled_display.draw_rectangle(0, 0, 122, 60); // Face outline

	oled_display.draw_rectangle(6, 6, 55, 30); // Left eye

	oled_display.draw_rectangle(67, 6, 50, 30); // Right eye

	for line in 6..=117 {
		oled_display.draw_point(line, 50); // Mouth
	}

	for line in 23..43 {
		oled_display.draw_point(line, 21); // Left pupil
	}

	for line in 84..100 {
		oled_display.draw_point(line, 21); // Right pupil
	}

	oled_display.display(); // Display the contents in GDDRAM
	
	loop {}
}


#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {

    }
}
