#![no_main]
#![no_std]

use compass_lsm303agr::config::initialization::{
    entry, init, iprintln, switch_hal::OutputSwitch, Direction,
};
use stm32f3_discovery::stm32f3xx_hal::prelude::*;

#[entry]
fn main() -> ! {
    let (leds, mut lsm303agr, mut delay, mut itm) = init();
    let mut stm_leds = leds.into_array();

    loop {

        iprintln!(&mut itm.stim[0], "{:?}", lsm303agr.mag_data().unwrap());

        delay.delay_ms(1_000_u16);
    }
}