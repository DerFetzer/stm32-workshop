#![no_main]
#![no_std]

// These lines are part of our setup for debug printing.
use defmt_rtt as _;
use panic_probe as _;

use core::cell::Cell;

use cortex_m::{
    interrupt::{free, Mutex},
    peripheral::NVIC,
};
use cortex_m_rt::entry;

use stm32_hal2::{
    gpio::{Edge, Pin, PinMode, PinState, Port},
    pac::{self, interrupt, EXTI},
};

static PUSH_COUNT: Mutex<Cell<u8>> = Mutex::new(Cell::new(0));

/// An example function to set up the pins that don't need to be interacted with directly later.
/// For example, ones used with buses (eg I2C, SPI, UART), USB, ADC, and DAC pins.
/// This may also include input pins that trigger interrupts, and aren't polled.
pub fn setup_pins() {
    // Set up button that triggers on the falling edge.
    let mut up_btn = Pin::new(Port::C, 13, PinMode::Input);
    up_btn.enable_interrupt(Edge::Falling);
}

#[entry]
fn main() -> ! {
    // Call a function we've made to help organize our pin setup code.
    setup_pins();

    // Example pins PB5 and PB6.
    let mut example_output = Pin::new(Port::B, 5, PinMode::Output);

    // Set high.
    example_output.set_state(PinState::High);

    // Unmask interrupt lines associated with the input pins we've configured interrupts
    // for in `setup_pins`.
    unsafe {
        // EXTI4_15 is associated with pins numbered 4-15
        NVIC::unmask(pac::Interrupt::EXTI4_15);
    }

    loop {}
}

#[interrupt]
/// Interrupt handler for PC13. This ISR is called when this push button goes low.
fn EXTI4_15() {
    free(|cs| {
        // Clear the interrupt flag, to prevent continous firing.
        unsafe { (*EXTI::ptr()).fpr1.modify(|_, w| w.fpif13().bit(true)) }

        let pc = PUSH_COUNT.borrow(cs);
        pc.replace(pc.get() + 1);
        defmt::println!("Push count: {}", pc.get());
    });
}
