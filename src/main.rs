#![no_main]
#![no_std]

extern crate panic_halt;

use crate::hal::{prelude::*, stm32};
use stm32f4xx_hal as hal;

// Imports to reduce length of type signatures in the code
use hal::gpio::gpioa::PA5;
use hal::gpio::{Output, PushPull};

use cortex_m::iprint;

use rtfm::cyccnt::U32Ext;

const PERIOD: u32 = 48_000_000;

#[rtfm::app(device = stm32f4xx_hal::stm32, peripherals = true, monotonic = rtfm::cyccnt::CYCCNT)]
const APP: () = {
    struct Resources {
        led: PA5<Output<PushPull>>,
        is_on: bool,
        itm: cortex_m::peripheral::ITM,
    }

    #[init(schedule = [blinky])]
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

        cx.schedule.blinky(cx.start + PERIOD.cycles()).unwrap();

        init::LateResources {
            led,
            is_on: false,
            itm,
        }
    }

    #[task(schedule = [blinky], resources = [led, itm, is_on])]
    fn blinky(cx: blinky::Context) {

        // Local alias to the reosurces, which are &mut
        let is_on = cx.resources.is_on;
        let led = cx.resources.led;

        // The ITM port for logging:
        let port = &mut cx.resources.itm.stim[0];

        if *is_on {
            led.set_high().unwrap();
        } else {
            led.set_low().unwrap();
        }
        *is_on = !(*is_on);

        let next = cx.scheduled + PERIOD.cycles();
        iprint!(port, "{:?}", next);
        cx.schedule.blinky(next).unwrap();
    }

    extern "C" {
        fn USART1();
    }
};
