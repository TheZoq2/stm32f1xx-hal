//! Blinks an LED

#![deny(unsafe_code)]
#![deny(warnings)]
#![no_std]
#![no_main]

extern crate cortex_m;
extern crate cortex_m_rt as rt;
extern crate panic_semihosting;
extern crate stm32f1xx_hal as hal;
extern crate embedded_hal_time;
#[macro_use(block)]
extern crate nb;

use hal::prelude::*;
use hal::stm32;
use hal::timer::Timer;
use rt::{entry, exception, ExceptionFrame};

use embedded_hal_time::{RealCountDown, Second};

#[entry]
fn main() -> ! {
    // let cp = cortex_m::Peripherals::take().unwrap();
    let dp = stm32::Peripherals::take().unwrap();

    let mut flash = dp.FLASH.constrain();
    let mut rcc = dp.RCC.constrain();

    // Try a different clock configuration
    let clocks = rcc.cfgr.freeze(&mut flash.acr);
    // let clocks = rcc.cfgr
    //     .sysclk(64.mhz())
    //     .pclk1(32.mhz())
    //     .freeze(&mut flash.acr);

    let mut gpioc = dp.GPIOC.split(&mut rcc.apb2);

    let mut led = gpioc.pc13.into_push_pull_output(&mut gpioc.crh);
    // Try a different timer (even SYST)
    let mut timer = Timer::tim2(dp.TIM2, 1.hz(), clocks, &mut rcc.apb1);
    let mut timer2 = Timer::tim3(dp.TIM3, 1.hz(), clocks, &mut rcc.apb1);
    loop {
        timer.start_real(Second(1));
        block!(timer.wait()).unwrap();
        led.set_high();
        timer2.start_real(Second(1));
        block!(timer2.wait()).unwrap();
        led.set_low();
    }
}

#[exception]
fn HardFault(ef: &ExceptionFrame) -> ! {
    panic!("{:#?}", ef);
}

#[exception]
fn DefaultHandler(irqn: i16) {
    panic!("Unhandled exception (IRQn = {})", irqn);
}
