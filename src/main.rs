//! This example shows a complete project, including file structure, and config
//! needed to flash using an ST-Link. The project structure is based on
//! [Knurling's app-template](https://github.com/knurling-rs/app-template).

#![no_main]
#![no_std]

use cortex_m::{self, delay::Delay};
use cortex_m_rt::entry;

// These lines are part of our setup for debug printing.
use defmt_rtt as _;
use panic_probe as _;

// Import parts of this library we use. You could use this style, or perhaps import
// less here.
use stm32_hal2::{
    self,
    clocks::Clocks,
    gpio::{Pin, PinMode, Port},
    pac,
    spi::{BaudRate, Spi, SpiConfig},
};

use smart_leds::{SmartLedsWrite, RGB8};
use ws2812_spi::prerendered::Ws2812;

const LED_COUNT: usize = 8;

const RED: RGB8 = RGB8 { r: 10, g: 0, b: 0 };
const GREEN: RGB8 = RGB8 { r: 0, g: 10, b: 0 };
const BLUE: RGB8 = RGB8 { r: 0, g: 0, b: 10 };
const WHITE: RGB8 = RGB8 { r: 5, g: 5, b: 5 };

fn setup_pins() {
    // Configure pins for Spi
    let _sck = Pin::new(Port::A, 0, PinMode::Alt(0));
    let _miso = Pin::new(Port::A, 3, PinMode::Alt(0));
    let _mosi = Pin::new(Port::A, 10, PinMode::Alt(0));
}

#[entry]
fn main() -> ! {
    // Set up ARM Cortex-M peripherals. These are common to many MCUs, including all STM32 ones.
    let cp = cortex_m::Peripherals::take().unwrap();
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

    defmt::println!(
        "Clocks: SYSCLK={}Hz APB1={}Hz APB2={}Hz",
        clock_cfg.sysclk(),
        clock_cfg.apb1(),
        clock_cfg.apb2()
    );

    setup_pins();

    let mut delay = Delay::new(cp.SYST, clock_cfg.sysclk());

    let spi_cfg = SpiConfig::default();

    let spi = Spi::new(
        dp.SPI2,
        spi_cfg,
        BaudRate::Div16, // APB1 / BaudRate::DivX = SPI clock
    );

    let mut output_buffer = [0; 20 + (LED_COUNT * 12)];
    let mut ws = Ws2812::new(spi, &mut output_buffer);

    let mut data: [RGB8; LED_COUNT] = [RED, GREEN, BLUE, WHITE, RED, GREEN, BLUE, WHITE];

    loop {
        defmt::println!("Looping!"); // A print statement using DEFMT.
        ws.write(data.iter().cloned()).unwrap();
        data.rotate_left(1);
        delay.delay_ms(1000);
    }
}

// same panicking *behavior* as `panic-probe` but doesn't print a panic message
// this prevents the panic message being printed *twice* when `defmt::panic` is invoked
#[defmt::panic_handler]
fn panic() -> ! {
    cortex_m::asm::udf()
}

/// Terminates the application and makes `probe-run` exit with exit-code = 0
pub fn exit() -> ! {
    loop {
        cortex_m::asm::bkpt();
    }
}
