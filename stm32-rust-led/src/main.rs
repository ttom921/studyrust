#![no_std]
#![no_main]

// pick a panicking behavior
use panic_halt as _; 
use cortex_m::asm;
use cortex_m_rt::entry;
use stm32f1::stm32f103;

#[entry]
fn main() -> ! {
    let peripherals = stm32f103::Peripherals::take().unwrap();
    let gpioc = &peripherals.GPIOC;
    let rcc = &peripherals.RCC;

    // enable the GPIO clock for IO port C
    rcc.apb2enr.write(|w| w.iopcen().set_bit());


    gpioc.crh.write(|w| {
        w.mode13().bits(0b11);
        w.cnf13().bits(0b00)
    });

    loop {
        gpioc.bsrr.write(|w| w.bs13().set_bit());
        asm::delay(2000000);
        gpioc.bsrr.write(|w| w.br13().set_bit());
        asm::delay(2000000);
    }
}
