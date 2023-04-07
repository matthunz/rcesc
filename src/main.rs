#![no_main]
#![no_std]

use cortex_m_rt::entry;
use embedded_hal::blocking::delay::DelayMs;
use embedded_hal::{Pwm, PwmPin};
use panic_halt as _;
use stm32f1xx_hal::device::Peripherals;
use stm32f1xx_hal::prelude::*;
use stm32f1xx_hal::timer::Tim2NoRemap;

pub struct RcEsc<T: PwmPin> {
    pub pwm: T,
    pub arm_duty: T::Duty,
    pub min_duty: T::Duty,
    pub max_duty: T::Duty,
    pub calibration_ms: T::Duty,
}

impl<T> RcEsc<T>
where
    T: PwmPin,
    T::Duty: Clone,
{
    pub fn new(pwm: T) -> Self
    where
        T::Duty: Default,
    {
        Self {
            pwm,
            arm_duty: Default::default(),
            min_duty: Default::default(),
            max_duty: Default::default(),
            calibration_ms: Default::default(),
        }
    }

    pub fn arm(&mut self) {
        self.pwm.set_duty(self.arm_duty.clone());
    }

    pub fn calibrate(&mut self, mut delay: impl DelayMs<T::Duty>) {
        self.pwm.set_duty(self.max_duty.clone());
        delay.delay_ms(self.calibration_ms.clone());

        self.pwm.set_duty(self.min_duty.clone());
        delay.delay_ms(self.calibration_ms.clone());
    }
}

#[entry]
fn main() -> ! {
    let p = Peripherals::take().unwrap();

    let mut flash = p.FLASH.constrain();
    let rcc = p.RCC.constrain();

    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    let mut afio = p.AFIO.constrain();

    let mut gpioa = p.GPIOA.split();
    // let mut gpiob = p.GPIOB.split();

    // TIM2
    let c1 = gpioa.pa0.into_alternate_push_pull(&mut gpioa.crl);

    let pwm = p
        .TIM2
        .pwm_hz::<Tim2NoRemap, _, _>(c1, &mut afio.mapr, 1.kHz(), &clocks)
        .split();

    let mut esc = RcEsc::new(pwm);

    let delay = p.TIM1.delay_ms(&clocks);
    esc.calibrate(delay);
    esc.arm();

    loop {}
}
