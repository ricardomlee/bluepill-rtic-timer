#![deny(unsafe_code)]
#![deny(warnings)]
#![no_main]
#![no_std]

use embedded_graphics::{
    mono_font::{ascii::FONT_6X10, MonoTextStyleBuilder},
    pixelcolor::BinaryColor,
    prelude::*,
    text::{Baseline, Text},
};
use heapless::String;
use panic_rtt_target as _;
use rtic::app;
use rtt_target::{rprintln, rtt_init_print};
use systick_monotonic::{fugit::Duration, Systick};
use timer::board::Board;
use timer::types::*;

#[app(device = stm32f1xx_hal::pac, peripherals = true, dispatchers = [SPI1])]
mod app {

    use super::*;

    #[shared]
    struct Shared {}

    #[local]
    struct Local {
        led: LedPin,
        oled: OledScreen,
    }

    #[monotonic(binds = SysTick, default = true)]
    type MonoTimer = Systick<1000>;

    #[init]
    fn init(cx: init::Context) -> (Shared, Local, init::Monotonics) {
        rtt_init_print!();
        rprintln!("init");

        let board = Board::init(cx.device);
        let led = board.led;
        let oled = board.oled;

        // Schedule the blinking task
        show_timer::spawn(1111, 11, 11).unwrap();
        blink::spawn_after(Duration::<u64, 1, 1000>::from_ticks(1000)).unwrap();

        let mono = Systick::new(cx.core.SYST, 36_000_000);
        (Shared {}, Local { led, oled }, init::Monotonics(mono))
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

    #[task(local = [oled])]
    fn show_timer(cx: show_timer::Context, mut hour: u16, mut min: u8, mut sec: u8) {
        let oled = cx.local.oled;
        oled.clear();

        let text_style = MonoTextStyleBuilder::new()
            .font(&FONT_6X10)
            .text_color(BinaryColor::On)
            .build();

        Text::with_baseline(
            String::<16>::from(hour).as_str(),
            Point::zero(),
            text_style,
            Baseline::Top,
        )
        .draw(oled)
        .unwrap();

        Text::with_baseline(
            String::<8>::from(min).as_str(),
            Point::new(0, 16),
            text_style,
            Baseline::Top,
        )
        .draw(oled)
        .unwrap();

        Text::with_baseline(
            String::<8>::from(sec).as_str(),
            Point::new(0, 32),
            text_style,
            Baseline::Top,
        )
        .draw(oled)
        .unwrap();

        oled.flush().unwrap();

        if sec == 0 {
            sec = 59;
            if min == 0 {
                min = 59;
                if hour == 0 {
                    hour = 1111;
                } else {
                    hour -= 1;
                }
            } else {
                min -= 1;
            }
        } else {
            sec -= 1;
        }

        show_timer::spawn_after(Duration::<u64, 1, 1000>::from_ticks(1000), hour, min, sec)
            .unwrap();
    }
}
