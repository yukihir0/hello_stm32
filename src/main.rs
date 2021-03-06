#![no_std]
#![no_main]

extern crate panic_halt;

use cortex_m_rt::entry;
use cortex_m_semihosting::{debug, hprintln};
use stm32f4xx_hal::{delay::Delay, prelude::*, stm32};

#[entry]
fn main() -> ! {
  let _ = hprintln!("Hello, world!");

  if let (Some(dp), Some(cp)) = (stm32::Peripherals::take(), stm32::CorePeripherals::take()) {
    let rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr.sysclk(48.mhz()).freeze();
    
    let gpioa = dp.GPIOA.split();
    let user_sw = gpioa.pa0.into_pull_down_input();

    let gpiod = dp.GPIOD.split();
    let mut led_blue = gpiod.pd15.into_push_pull_output();
    let mut led_red = gpiod.pd14.into_push_pull_output();
    let mut led_orange = gpiod.pd13.into_push_pull_output();
    let mut led_green = gpiod.pd12.into_push_pull_output();

    let mut delay = Delay::new(cp.SYST, clocks);

    for _ in 0..5 {
      match user_sw.is_high() {
        Ok(status)  => {
          if status == true {
            led_green.set_high().unwrap()
          } else {
            led_green.set_low().unwrap()
          }
        },
        Err(_) => {},
      }

      led_blue.set_high().unwrap();
      led_red.set_high().unwrap();
      led_orange.set_high().unwrap();
      delay.delay_ms(1000_u32);

      led_blue.set_low().unwrap();
      led_red.set_low().unwrap();
      led_orange.set_low().unwrap();
      delay.delay_ms(1000_u32);
    }
  }

  debug::exit(debug::EXIT_SUCCESS);

  loop {}
}
