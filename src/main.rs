#![no_std]
#![no_main]

// pick a panicking behavior
use panic_halt as _; // you can put a breakpoint on `rust_begin_unwind` to catch panics
// use panic_abort as _; // requires nightly
// use panic_itm as _; // logs messages over ITM; requires ITM support
// use panic_semihosting as _; // logs messages to the host stderr; requires a debugger

use cortex_m::{asm, peripheral};
use cortex_m_rt::entry;
use core::cell::{Cell, RefCell};
use stm32f4xx_hal::{
    gpio::{Pin, gpiod},
    pac::{self},
    prelude::*,
};

#[entry]
fn main() -> ! {
    // asm::nop(); // To not have main optimize to abort in release mode, remove when you add code

    // obtains a handle for the device peripherlas
    let dp = pac::Peripherals::take().unwrap();

    // configures the LED pin as a push-pull output and obtain a handler for the pin
    let gpiod = dp.GPIOD.split();

    // Obtain a handle for the LED 
    let mut led = gpiod.pd12.into_push_pull_output();

    // Set up sys clock
    let rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr.sysclk(48.MHz()).freeze();

    // Creates a delay abstraction based on SysTick
    let mut delay = dp.TIM1.delay_ms(&clocks);

    loop {
        // your code goes here
        led.toggle();
        delay.delay_ms(1000_u32);
    }
}
