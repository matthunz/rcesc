#![no_main]
#![no_std]

use cortex_m_rt::entry;
use embedded_hal::blocking::delay::DelayMs;
use embedded_hal::PwmPin;
use panic_halt as _;
use stm32f1xx_hal::device::Peripherals;
use stm32f1xx_hal::prelude::*;
use stm32f1xx_hal::timer::Tim2NoRemap;

pub struct RcEsc<T: PwmPin> {
    pub pwm: T,
    pub arm_duty: T::Duty,
    pub stop_duty: T::Duty,
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
            stop_duty: Default::default(),
            min_duty: Default::default(),
            max_duty: Default::default(),
            calibration_ms: Default::default(),
        }
    }

    pub fn arm(&mut self) {
        self.pwm.set_duty(self.arm_duty.clone());
    }

    pub fn stop(&mut self) {
        self.pwm.set_duty(self.stop_duty.clone());
    }

    pub fn set_max_duty(&mut self) {
        self.pwm.set_duty(self.max_duty.clone());
    }

    pub fn set_min_duty(&mut self) {
        self.pwm.set_duty(self.min_duty.clone());
    }

    pub fn calibrate(&mut self, mut delay: impl DelayMs<T::Duty>) {
        self.set_max_duty();
        delay.delay_ms(self.calibration_ms.clone());

        self.set_min_duty();
        delay.delay_ms(self.calibration_ms.clone());
    }
}

#[entry]
fn main() -> ! {
    let peripherals = Peripherals::take().unwrap();

    let mut flash = peripherals.FLASH.constrain();
    let rcc = peripherals.RCC.constrain();
    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    let mut afio = peripherals.AFIO.constrain();
    let mut gpioa = peripherals.GPIOA.split();

    let c1 = gpioa.pa0.into_alternate_push_pull(&mut gpioa.crl);
    let pwm = peripherals
        .TIM2
        .pwm_hz::<Tim2NoRemap, _, _>(c1, &mut afio.mapr, 1.kHz(), &clocks)
        .split();

    let mut esc = RcEsc::new(pwm);
    esc.min_duty = 1000;
    esc.max_duty = 2000;
    esc.arm_duty = 500;
    esc.calibration_ms = 8000;
    esc.stop_duty = 500;

    let delay = peripherals.TIM1.delay_ms(&clocks);
    esc.calibrate(delay);
    esc.arm();

    loop {}
}
