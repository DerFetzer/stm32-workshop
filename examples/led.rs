#![no_std]
#![no_main]

// These lines are part of our setup for debug printing.
use defmt_rtt as _;
use panic_probe as _;

use cortex_m_rt::entry;
use stm32_hal2::gpio::{Pin, PinMode, Port};

#[entry]
fn main() -> ! {
    let mut led = Pin::new(Port::B, 13, PinMode::Output);
    led.set_high();

    defmt::info!("Looping!");
    loop {}
}
