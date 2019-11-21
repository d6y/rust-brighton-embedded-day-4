#![no_main]
#![no_std]

extern crate panic_halt;
// use cortex_m;

use crate::hal::{prelude::*, stm32};
use stm32f4xx_hal as hal;

use cortex_m_semihosting::hprintln;
// use panic_semihosting as _;

#[rtfm::app(device = stm32f4xx_hal::stm32, peripherals = true)]
const APP: () = {
    #[init]
    fn init(cx: init::Context) {
        hprintln!("init").unwrap();

        // Cortex-M peripherals
        let cp: cortex_m::Peripherals = cx.core;

        // Device specific peripherals
        let dp: stm32::Peripherals = cx.device;

        // Set up the LED: it's connected to pin PA5 on the microcontroler
        let gpioa = dp.GPIOA.split();
        let mut led = gpioa.pa5.into_push_pull_output();

        // The external LED, on the next pin down:
        let mut xled = gpioa.pa6.into_push_pull_output();

        // Set up the system clock. We want to run at 48MHz for this one.
        let rcc = dp.RCC.constrain();
        let clocks = rcc.cfgr.sysclk(48.mhz()).freeze();

        // Create a delay abstraction based on SysTick
        let mut delay = hal::delay::Delay::new(cp.SYST, clocks);

        loop {
            led.set_high().unwrap();
            xled.set_low().unwrap();
            delay.delay_ms(1000_u32);
            led.set_low().unwrap();
            xled.set_high().unwrap();
            delay.delay_ms(1000_u32);
        }
    }

    #[idle]
    fn idle(cx: idle::Context) -> ! {
        hprintln!("idle").unwrap();
        loop {}
    }
};
