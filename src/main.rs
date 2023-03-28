// Don't include standard rust library, since it's incompatible with embedded systems
#![no_std]

// We use our own entrypoint marked with #[riscv_rt::entry], not rust's default.
#![no_main]

// import Hardware Abstraction Layer for bl602
use bl602_hal as hal;

use core::fmt::Write;
use embedded_hal::delay::blocking::DelayMs;
use embedded_hal::digital::blocking::OutputPin;

use hal::{
	clock::{Strict, SysclkFreq, UART_PLL_FREQ},
	pac,
	prelude::*,
	serial::*,
};

// We want our embedded system to halt indefinitely when we panic.
// We discard the import since its functionality is a side-effect
use panic_halt as _;

// define a baudrate for serial communication
const BAUD_RATE:u32 = 1000000;

// RiscV entrypoint. The ! notation indicates we have no panic handler
#[riscv_rt::entry]
fn main() -> ! {

	// Load all peripherals
	let dp = pac::Peripherals::take().unwrap();
	// split the register block into individual pins and modules
	let mut parts = dp.GLB.split();

	// Set up all the clocks we need
	let clocks = Strict::new()
		.use_pll(40_000_000u32.Hz())
		.sys_clk(SysclkFreq::Pll160Mhz)
		.uart_clk(UART_PLL_FREQ.Hz())
		.freeze(&mut parts.clk_cfg);

	// Set up uart output. Bl602 uses a pin matrix so we set muxes too.
	let pin16 = parts.pin16.into_uart_sig0();
	let pin7 = parts.pin7.into_uart_sig7();
	let mux0 = parts.uart_mux0.into_uart0_tx();
	let mux7 = parts.uart_mux7.into_uart0_rx();

	// Configure our UART to our baudrate, and use the pins we configured above
	let mut serial = Serial::new(dp.UART0, Config::default().baudrate(BAUD_RATE.Bd()), ((pin16, mux0), (pin7, mux7)), clocks);

	// Pins for LED for Pinecone dev board
	let mut blue_led = parts.pin11.into_pull_down_output();
	let mut green_led = parts.pin14.into_pull_down_output();
	let mut red_led = parts.pin17.into_pull_down_output();

	// Create a delay function using the riscv chip cycle counter
	let mut delay = bl602_hal::delay::McycleDelay::new(clocks.sysclk().0);

	loop {
		// Toggle the LED on and off once a second. Report LED status over UART

		// Set blue led to high (turn on)
		blue_led.set_high().unwrap();
		// write to serial connection
		serial.write_str("RED HIGH\r\n").ok();
		// wait for 1000ms (not entire accurate, since it uses the internal cycle counter)
		delay.delay_ms(1000).unwrap();

		// set blue led to low (turn off)
		blue_led.set_low().unwrap();
		serial.write_str("BLUE  LOW\r\n").ok();
		delay.delay_ms(1000).unwrap();

		green_led.set_high().unwrap();
		serial.write_str("GREEN HIGH\r\n").ok();
		delay.delay_ms(1000).unwrap();

		green_led.set_low().unwrap();
		serial.write_str("GREEN  LOW\r\n").ok();
		delay.delay_ms(1000).unwrap();

		red_led.set_high().unwrap();
		serial.write_str("RED HIGH\r\n").ok();
		delay.delay_ms(1000).unwrap();

		red_led.set_low().unwrap();
		serial.write_str("RED  LOW\r\n").ok();
		delay.delay_ms(1000).unwrap();
	}
}
