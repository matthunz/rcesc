#![no_main]
#![no_std]

use cortex_m_rt::entry;
use panic_halt as _;
use rcesc::Esc;
use stm32f1xx_hal::device::Peripherals;
use stm32f1xx_hal::prelude::*;
use stm32f1xx_hal::timer::{Channel, Tim2NoRemap};

#[entry]
fn main() -> ! {
    let peripherals = Peripherals::take().unwrap();

    let mut flash = peripherals.FLASH.constrain();
    let rcc = peripherals.RCC.constrain();
    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    let mut afio = peripherals.AFIO.constrain();
    let mut gpioa = peripherals.GPIOA.split();

    let c1 = gpioa.pa0.into_alternate_push_pull(&mut gpioa.crl);
    let c2 = gpioa.pa1.into_alternate_push_pull(&mut gpioa.crl);

    let mut pwm =
        peripherals
            .TIM2
            .pwm_hz::<Tim2NoRemap, _, _>((c1, c2), &mut afio.mapr, 1.kHz(), &clocks);
    pwm.enable(Channel::C1);
    pwm.enable(Channel::C2);

    let mut esc = Esc::new(pwm);
    esc.min_duty = 1000;
    esc.max_duty = 2000;
    esc.arm_duty = 500;
    esc.disarm_duty = 500;

    let delay = peripherals.TIM1.delay_ms(&clocks);
    esc.calibrate_channels(8000u32, delay, &[Channel::C1, Channel::C2]);

    loop {}
}
