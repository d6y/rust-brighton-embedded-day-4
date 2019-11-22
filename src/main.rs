#![no_main]
#![no_std]

extern crate panic_halt;

use crate::hal::{prelude::*, stm32};
use stm32f4xx_hal as hal;

// Imports to reduce length of type signatures in the code
use hal::gpio::gpioa::PA5;
use hal::gpio::{Output, PushPull};

use cortex_m::iprintln;

use rtfm::cyccnt::U32Ext;

const PERIOD: u32 = 48_000_000; // 48mhz

#[rtfm::app(device = stm32f4xx_hal::stm32, peripherals = true, monotonic = rtfm::cyccnt::CYCCNT)]
const APP: () = {
    struct Resources {
        led: PA5<Output<PushPull>>,
        itm: cortex_m::peripheral::ITM,
    }

    #[init(schedule = [lights_on])]
    fn init(mut cx: init::Context) -> init::LateResources {
        // Device specific peripherals
        let dp: stm32::Peripherals = cx.device;

        // Set up the LED: it's connected to pin PA5 on the microcontroler
        let gpioa = dp.GPIOA.split();
        let led = gpioa.pa5.into_push_pull_output();

        // Set up the system clock. We want to run at 48MHz
        let rcc = dp.RCC.constrain();
        let _clocks = rcc.cfgr.sysclk(48.mhz()).freeze();

        // Initialize (enable) the monotonic timer (CYCCNT) on the Cortex-M peripherals
        cx.core.DCB.enable_trace();
        cx.core.DWT.enable_cycle_counter();
        let itm = cx.core.ITM;

        cx.schedule.lights_on(cx.start + PERIOD.cycles()).unwrap();

        init::LateResources { led, itm }
    }

    #[task(schedule = [lights_off], resources = [led, itm])]
    fn lights_on(cx: lights_on::Context) {
        cx.resources.led.set_high().expect("failed to set high");

        let next = cx.scheduled + PERIOD.cycles();
        cx.schedule
            .lights_off(next)
            .expect("failed to schedule lights off");

        let port = &mut cx.resources.itm.stim[0];
        iprintln!(port, "Off at: {:?}", next);
    }

    #[task(schedule = [lights_on], resources = [led, itm])]
    fn lights_off(cx: lights_off::Context) {
        cx.resources.led.set_low().expect("failed to set low");

        let next = cx.scheduled + PERIOD.cycles();
        cx.schedule
            .lights_on(next)
            .expect("failed to schedule lights on");

        let port = &mut cx.resources.itm.stim[0];
        iprintln!(port, "On at: {:?}", next);
    }

    extern "C" {
        fn USART1();
    }
};
