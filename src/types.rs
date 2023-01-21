use max3010x::{
    marker::{ic::Max30102, mode::HeartRate},
    Max3010x,
};
use shared_bus::{AtomicCheckMutex, BusManager, I2cProxy};
use ssd1306::{mode::BufferedGraphicsMode, prelude::*, Ssd1306};
use stm32f1::stm32f103::I2C1;
use stm32f1xx_hal::{gpio::*, i2c::BlockingI2c};

pub type LedPin = gpioc::PC13<Output<PushPull>>;

pub type I2c = BlockingI2c<
    I2C1,
    (
        Pin<'B', 8, Alternate<OpenDrain>>,
        Pin<'B', 9, Alternate<OpenDrain>>,
    ),
>;

pub type SharedBus = &'static BusManager<AtomicCheckMutex<I2c>>;

pub type SharedBusProxy = I2cProxy<'static, AtomicCheckMutex<I2c>>;

pub type Max30102Sensor = Max3010x<SharedBusProxy, Max30102, HeartRate>;

pub type OledScreen = Ssd1306<
    I2CInterface<SharedBusProxy>,
    DisplaySize128x64,
    BufferedGraphicsMode<DisplaySize128x64>,
>;
