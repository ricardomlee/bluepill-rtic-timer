#![deny(unsafe_code)]
#![deny(warnings)]
#![no_main]
#![no_std]

use panic_rtt_target as _;
use rtic::app;

#[app(device = stm32f1xx_hal::pac, peripherals = true, dispatchers = [SPI1])]
mod app {
    use rtt_target::{rprintln, rtt_init_print};
    use systick_monotonic::{fugit::Duration, Systick};
    use timer::board::Board;
    use timer::types::*;

    #[shared]
    struct Shared {}

    #[local]
    struct Local {
        led: LedPin,
    }

    #[monotonic(binds = SysTick, default = true)]
    type MonoTimer = Systick<1000>;

    #[init]
    fn init(cx: init::Context) -> (Shared, Local, init::Monotonics) {
        rtt_init_print!();
        rprintln!("init");

        let board = Board::init(cx.device);
        let led = board.led;

        // Schedule the blinking task
        blink::spawn_after(Duration::<u64, 1, 1000>::from_ticks(1000)).unwrap();
        let mono = Systick::new(cx.core.SYST, 36_000_000);
        (Shared {}, Local { led }, init::Monotonics(mono))
    }

    #[task(local = [led])]
    fn blink(cx: blink::Context) {
        rprintln!("hello, blink");
        let led = cx.local.led;
        if led.is_set_low() {
            led.set_high();
        } else {
            led.set_low();
        }
        blink::spawn_after(Duration::<u64, 1, 1000>::from_ticks(1000)).unwrap();
    }
}
