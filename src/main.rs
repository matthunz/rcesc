#![no_main]
#![no_std]

use cortex_m_rt::entry;
use embedded_hal::blocking::delay::DelayMs;
use embedded_hal::{Pwm, PwmPin};
use panic_halt as _;
use stm32f1xx_hal::device::Peripherals;
use stm32f1xx_hal::prelude::*;
use stm32f1xx_hal::timer::{Channel, Tim2NoRemap};

pub struct RcEsc<P, D> {
    pub pwm: P,
    pub arm_duty: D,
    pub disarm_duty: D,
    pub min_duty: D,
    pub max_duty: D,
}

impl<P, D> RcEsc<P, D> {
    /// Create a new RC ESC with the given pwm output.
    pub fn new(pwm: P) -> Self
    where
        D: Default,
    {
        Self {
            pwm,
            arm_duty: Default::default(),
            disarm_duty: Default::default(),
            min_duty: Default::default(),
            max_duty: Default::default(),
        }
    }

    /// Arm the ESC.
    pub fn arm(&mut self)
    where
        P: PwmPin<Duty = D>,
        D: Clone,
    {
        self.pwm.set_duty(self.arm_duty.clone());
    }

    /// Arm the ESC at the given channel.
    pub fn arm_channel(&mut self, channel: P::Channel)
    where
        P: Pwm<Duty = D>,
        D: Clone,
    {
        self.pwm.set_duty(channel, self.arm_duty.clone());
    }

    /// Disarm the ESC.
    pub fn disarm(&mut self)
    where
        P: PwmPin<Duty = D>,
        D: Clone,
    {
        self.pwm.set_duty(self.disarm_duty.clone());
    }

    /// Disarm the ESC at the given channel.
    pub fn disarm_channel(&mut self, channel: P::Channel)
    where
        P: Pwm<Duty = D>,
        D: Clone,
    {
        self.pwm.set_duty(channel, self.disarm_duty.clone());
    }

    pub fn set_max_duty(&mut self)
    where
        P: PwmPin<Duty = D>,
        D: Clone,
    {
        self.pwm.set_duty(self.max_duty.clone());
    }

    pub fn set_channel_max_duty(&mut self, channel: P::Channel)
    where
        P: Pwm<Duty = D>,
        D: Clone,
    {
        self.pwm.set_duty(channel, self.max_duty.clone());
    }

    pub fn set_min_duty(&mut self)
    where
        P: PwmPin<Duty = D>,
        D: Clone,
    {
        self.pwm.set_duty(self.min_duty.clone());
    }

    pub fn set_channel_min_duty(&mut self, channel: P::Channel)
    where
        P: Pwm<Duty = D>,
        D: Clone,
    {
        self.pwm.set_duty(channel, self.min_duty.clone());
    }

    pub fn calibrate<M: Clone>(&mut self, calibration_ms: M, mut delay: impl DelayMs<M>)
    where
        P: PwmPin<Duty = D>,
        D: Clone,
    {
        self.set_max_duty();
        delay.delay_ms(calibration_ms.clone());

        self.set_min_duty();
        delay.delay_ms(calibration_ms);
    }

    pub fn calibrate_channels<M: Clone>(
        &mut self,
        calibration_ms: M,
        mut delay: impl DelayMs<M>,
        channels: &[P::Channel],
    ) where
        P: Pwm<Duty = D>,
        P::Channel: Clone,
        D: Clone,
    {
        for channel in channels {
            self.set_channel_max_duty(channel.clone());
        }
        delay.delay_ms(calibration_ms.clone());

        for channel in channels {
            self.set_channel_min_duty(channel.clone());
        }
        delay.delay_ms(calibration_ms);
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
    let c2 = gpioa.pa1.into_alternate_push_pull(&mut gpioa.crl);

    let mut pwm =
        peripherals
            .TIM2
            .pwm_hz::<Tim2NoRemap, _, _>((c1, c2), &mut afio.mapr, 1.kHz(), &clocks);
    pwm.enable(Channel::C1);
    pwm.enable(Channel::C2);

    let mut esc = RcEsc::new(pwm);
    esc.min_duty = 1000;
    esc.max_duty = 2000;
    esc.arm_duty = 500;
    esc.disarm_duty = 500;

    let delay = peripherals.TIM1.delay_ms(&clocks);
    esc.calibrate_channels(8000u32, delay, &[Channel::C1, Channel::C2]);

    loop {}
}
