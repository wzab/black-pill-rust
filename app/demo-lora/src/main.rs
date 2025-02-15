#![deny(unsafe_code)]
#![no_main]
#![no_std]

use panic_semihosting as _;

use cortex_m_rt::entry;
use stm32f1xx_hal::{
  gpio::gpioa::PA4,
  gpio::{Output, PushPull},
  pac::{Peripherals,SPI1}, 
  prelude::*,
  spi::{Pins, Spi, Spi1NoRemap},
  };
  
use sx127x_lora::MODE;
use cortex_m_semihosting::hprintln;

fn setup() -> (
    Spi<SPI1, Spi1NoRemap, impl Pins<Spi1NoRemap>, u8>,
    PA4<Output<PushPull>>,
) {
    let dp = Peripherals::take().unwrap();

    let mut flash = dp.FLASH.constrain();
    let rcc = dp.RCC.constrain();

    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    let mut afio = dp.AFIO.constrain();
    let mut gpioa = dp.GPIOA.split();

    // SPI1
    let sck = gpioa.pa5.into_alternate_push_pull(&mut gpioa.crl);
    let miso = gpioa.pa6;
    let mosi = gpioa.pa7.into_alternate_push_pull(&mut gpioa.crl);
    let cs = gpioa.pa4.into_push_pull_output(&mut gpioa.crl);

    let spi = Spi::spi1(
        dp.SPI1,
        (sck, miso, mosi),
        &mut afio.mapr,
        MODE,
        1.MHz(),
        clocks,
    );

    (spi, cs)
}


#[entry]
fn main() -> ! {
    // Acquire SPI
    let (_spi, _cs) = setup(); 
    loop{};
}
