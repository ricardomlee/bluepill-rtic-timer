//! Board initialization

use max3010x::Max3010x;
use ssd1306::{prelude::*, I2CDisplayInterface, Ssd1306};

use stm32f1xx_hal::prelude::*;

use crate::types::*;

use max3010x::{Led, LedPulseWidth, SampleAveraging, SamplingRate};
use stm32f1xx_hal::{
    gpio::PinState,
    i2c::{BlockingI2c, DutyCycle, Mode},
};

pub struct Board {
    pub led: LedPin,
    pub max30102: Max30102Sensor,
    pub oled: OledScreen,
    pub bus_manager: SharedBus,
}

impl Board {
    pub fn init(device: stm32f1::stm32f103::Peripherals) -> Self {
        let mut flash = device.FLASH.constrain();
        let rcc = device.RCC.constrain();

        let clocks = rcc
            .cfgr
            .use_hse(8.MHz())
            .sysclk(36.MHz())
            .pclk1(36.MHz())
            .freeze(&mut flash.acr);

        let mut gpiob = device.GPIOB.split();
        let mut gpioc = device.GPIOC.split();
        let mut afio = device.AFIO.constrain();

        let led = gpioc
            .pc13
            .into_push_pull_output_with_state(&mut gpioc.crh, PinState::Low);

        let scl = gpiob.pb8.into_alternate_open_drain(&mut gpiob.crh);
        let sda = gpiob.pb9.into_alternate_open_drain(&mut gpiob.crh);

        let i2c = BlockingI2c::i2c1(
            device.I2C1,
            (scl, sda),
            &mut afio.mapr,
            Mode::Fast {
                frequency: 400_000.Hz(),
                duty_cycle: DutyCycle::Ratio2to1,
            },
            clocks,
            1000,
            10,
            1000,
            1000,
        );
        let bus_manager: &'static _ = shared_bus::new_atomic_check!(I2c = i2c).unwrap();

        let max30102 = Max3010x::new_max30102(bus_manager.acquire_i2c());
        let mut max30102 = max30102.into_heart_rate().unwrap();

        max30102.enable_fifo_rollover().unwrap();
        max30102.set_pulse_amplitude(Led::All, 15).unwrap();
        max30102.set_sample_averaging(SampleAveraging::Sa8).unwrap();
        max30102.set_sampling_rate(SamplingRate::Sps100).unwrap();
        max30102.set_pulse_width(LedPulseWidth::Pw411).unwrap();

        max30102.clear_fifo().unwrap();

        let interface = I2CDisplayInterface::new(bus_manager.acquire_i2c());
        let mut oled = Ssd1306::new(interface, DisplaySize128x64, DisplayRotation::Rotate0)
            .into_buffered_graphics_mode();
        oled.init().unwrap();

        Board {
            led,
            max30102,
            oled,
            bus_manager,
        }
    }
}
