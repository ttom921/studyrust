#![no_std]
#![no_main]

// pick a panicking behavior
use panic_halt as _; // you can put a breakpoint on `rust_begin_unwind` to catch panics
                     // use panic_abort as _; // requires nightly
                     // use panic_itm as _; // logs messages over ITM; requires ITM support
                     // use panic_semihosting as _; // logs messages to the host stderr; requires a debugger
use nb::block;
//use cortex_m::asm;
use cortex_m_rt::entry;
use stm32f1xx_hal::{pac, prelude::*, timer::Timer};

#[entry]
fn main() -> ! {
    let _cp = cortex_m::Peripherals::take().unwrap();
    let _dp = pac::Peripherals::take().unwrap();
    let mut flash = _dp.FLASH.constrain();
    let rcc = _dp.RCC.constrain();

    // `clocks`
    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    // Acquire the GPIOC peripheral
    let mut gpioc = _dp.GPIOC.split();

    // Configure gpio C pin 13 as a push-pull output. The `crh` register is passed to the function
    // in order to configure the port. For pins 0-7, crl should be passed instead.
    let mut led = gpioc.pc13.into_push_pull_output(&mut gpioc.crh);

    // Configure the syst timer to trigger an update every second
    let mut timer = Timer::syst(_cp.SYST, &clocks).counter_hz();
    timer.start(5.Hz()).unwrap();

    loop {
        // your code goes here
        block!(timer.wait()).unwrap();
        led.set_high();

        block!(timer.wait()).unwrap();
        led.set_low();
    }
}
