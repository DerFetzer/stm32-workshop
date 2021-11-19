#![no_main]
#![no_std]

// These lines are part of our setup for debug printing.
use defmt_rtt as _;
use panic_probe as _;

use core::cell::RefCell;

use cortex_m::{
    interrupt::{free, Mutex},
    peripheral::NVIC,
};
use cortex_m_rt::entry;

use stm32_hal2::{
    clocks::Clocks,
    pac::{self, interrupt, TIM3},
    timer::{Timer, TimerInterrupt},
};

// More complex values go in `RefCell`s. Use an option, since we need to set this up
// before we initialize the peripheral it stores.
static TIM: Mutex<RefCell<Option<Timer<TIM3>>>> = Mutex::new(RefCell::new(None));

#[entry]
fn main() -> ! {
    // Set up ARM Cortex-M peripherals. These are common to many MCUs, including all STM32 ones.
    let _cp = cortex_m::Peripherals::take().unwrap();
    // Set up peripherals specific to the microcontroller you're using.
    let dp = pac::Peripherals::take().unwrap();

    // Create an initial clock configuration that uses the MCU's internal oscillator (HSI),
    // sets the MCU to its maximum system clock speed.
    let clock_cfg = Clocks {
        ..Default::default()
    };

    // Write the clock configuration to the MCU. If you wish, you can modify `clocks` above
    // in accordance with [its docs](https://docs.rs/stm32-hal2/0.3.1/stm32_hal2/clocks/index.html),
    // and the `clock_cfg` example.
    clock_cfg.setup().unwrap();

    let mut countdown_timer = Timer::new_tim3(dp.TIM3, 0.5, &clock_cfg);
    countdown_timer.enable_interrupt(TimerInterrupt::Update); // Enable update event interrupts.
    countdown_timer.enable();

    // Set up our TIM as a global variable accessible in interrupts, now that it's initialized.
    free(|cs| {
        TIM.borrow(cs).replace(Some(countdown_timer));
    });

    // Unmask interrupt line associated with the timer we've configured interrupts for.
    unsafe {
        NVIC::unmask(pac::Interrupt::TIM3_TIM4);
    }

    loop {}
}

#[interrupt]
/// Interrupt handler for TIM3.
fn TIM3_TIM4() {
    free(|cs| {
        let mut tim_ref = TIM.borrow(cs).borrow_mut();
        let tim = tim_ref.as_mut().unwrap();
        tim.clear_interrupt(TimerInterrupt::Update);

        defmt::println!("TIM3 expired");
    });
}
