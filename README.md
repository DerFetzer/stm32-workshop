# STM32 workshop

This is based on the [STM32-HAL quickstart](https://github.com/David-OConnor/stm32-hal-quickstart) with modified 
examples from [STM32-HAL](https://github.com/David-OConnor/stm32-hal).

# Hardware

| MCU           | Target                | Features       | Flash | RAM  | Datasheet | Reference Manual | Board Manual |
|---------------|-----------------------|----------------|-------|------|-----------|------------------|--------------|
| STM32F302R8Tx | thumbv7em-none-eabihf | "f302", "f3rt" | 64K   | 16K  | https://www.st.com/resource/en/datasheet/stm32f302r8.pdf | https://www.st.com/resource/en/reference_manual/rm0365-stm32f302xbcde-and-stm32f302x68-advanced-armbased-32bit-mcus-stmicroelectronics.pdf | https://www.st.com/resource/en/user_manual/um1724-stm32-nucleo64-boards-mb1136-stmicroelectronics.pdf |
| STM32F303RETx | thumbv7em-none-eabihf | "f303", "f3rt" | 512K  | 80K  | https://www.st.com/resource/en/datasheet/stm32f303re.pdf | https://www.st.com/resource/en/reference_manual/rm0316-stm32f303xbcde-stm32f303x68-stm32f328x8-stm32f358xc-stm32f398xe-advanced-armbased-mcus-stmicroelectronics.pdf | https://www.st.com/resource/en/user_manual/um1724-stm32-nucleo64-boards-mb1136-stmicroelectronics.pdf |
| STM32F410RBTx | thumbv7em-none-eabihf | "f410", "f4rt" | 128K  | 32K  | https://www.st.com/resource/en/datasheet/stm32f410rb.pdf | https://www.st.com/resource/en/reference_manual/rm0401-stm32f410-advanced-armbased-32bit-mcus-stmicroelectronics.pdf | https://www.st.com/resource/en/user_manual/um1724-stm32-nucleo64-boards-mb1136-stmicroelectronics.pdf |
| STM32F411RETx | thumbv7em-none-eabihf | "f411", "f4rt" | 512K  | 128K | https://www.st.com/resource/en/datasheet/stm32f411re.pdf | https://www.st.com/resource/en/reference_manual/rm0383-stm32f411xce-advanced-armbased-32bit-mcus-stmicroelectronics.pdf | https://www.st.com/resource/en/user_manual/um1724-stm32-nucleo64-boards-mb1136-stmicroelectronics.pdf |
| STM32F446RETx | thumbv7em-none-eabihf | "f446", "f4rt" | 512K  | 128K | https://www.st.com/resource/en/datasheet/stm32f446re.pdf | https://www.st.com/resource/en/reference_manual/rm0390-stm32f446xx-advanced-armbased-32bit-mcus-stmicroelectronics.pdf | https://www.st.com/resource/en/user_manual/um1724-stm32-nucleo64-boards-mb1136-stmicroelectronics.pdf |
| STM32G0B1RETx | thumbv6m-none-eabi    | "g0b1", "g0rt" | 512K  | 128K | https://www.st.com/resource/en/datasheet/stm32g0b1re.pdf | https://www.st.com/resource/en/reference_manual/rm0444-stm32g0x1-advanced-armbased-32bit-mcus-stmicroelectronics.pdf | https://www.st.com/resource/en/user_manual/um2324-stm32-nucleo64-boards-mb1360-stmicroelectronics.pdf |

# Setup
- Windows-only: Install ST-Link driver from [here](https://www.st.com/en/development-tools/stsw-link009.html) (registration required) or `Z:\Tools\STM`
- [Install Rust](https://www.rust-lang.org/tools/install).
- Install the compilation target for your [MCU](#hardware). Eg run `rustup target add thumbv7em-none-eabihf`.
- Install flash and debug tools: `cargo install flip-link`, `cargo install probe-run`, `cargo install cargo-binutils`.
- Change the following lines to match your [MCU](#hardware):
  - `Cargo.toml`: `features = ["embedded-hal", "g0b1", "g0rt"]`
  - `memory.x`: `FLASH` and `RAM` lines
  - `.cargo/config.toml`: `runner` and `target` lines.
- Connect your device. Run `cargo run --release` to compile and flash.

# Rust
> Low-level code is prone to a variety of subtle bugs, which in most other languages can be caught only through extensive testing and careful code review by experienced developers.
> In Rust, the compiler plays a gatekeeper role by refusing to compile code with these elusive bugs, including concurrency bugs.
> ...
> By striving for zero-cost abstractions, higher-level features that compile to lower-level code as fast as code written manually, Rust endeavors to make safe code be fast code as well.
> 
> -- <cite>Rust Book</cite>

## General
- [Rust by Example](https://doc.rust-lang.org/stable/rust-by-example/)
  1. [Hello World](https://doc.rust-lang.org/stable/rust-by-example/hello.html)
  2. [Primitives](https://doc.rust-lang.org/stable/rust-by-example/primitives.html)
  3. [Custom Types](https://doc.rust-lang.org/stable/rust-by-example/custom_types.html)
  4. [Flow of Control](https://doc.rust-lang.org/stable/rust-by-example/flow_control.html)
  5. [Methods](https://doc.rust-lang.org/stable/rust-by-example/fn/methods.html)
  6. [Scoping rules](https://doc.rust-lang.org/stable/rust-by-example/scope.html)
  7. [Traits](https://doc.rust-lang.org/stable/rust-by-example/trait.html)
  8. [Error handling](https://doc.rust-lang.org/stable/rust-by-example/error.html)
- [Rust Book](https://doc.rust-lang.org/stable/book/)
- [Playground](https://play.rust-lang.org/)
- [Crate registry](https://crates.io/)

## Embedded
- [Embedded Rust Book](https://docs.rust-embedded.org/book/intro/index.html)
- [Awesome Embedded Rust](https://github.com/rust-embedded/awesome-embedded-rust)