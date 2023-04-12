#![no_std]

use embedded_hal::{blocking::delay::DelayMs, Pwm, PwmPin};

pub struct Esc<P, D> {
    pub pwm: P,
    pub arm_duty: D,
    pub disarm_duty: D,
    pub min_duty: D,
    pub max_duty: D,
}

impl<P, D> Esc<P, D> {
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

    pub fn calibrate<M>(&mut self, calibration_ms: M, mut delay: impl DelayMs<M>)
    where
        P: PwmPin<Duty = D>,
        D: Clone,
        M: Clone,
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

impl<P, D> Pwm for Esc<P, D>
where
    P: Pwm<Duty = D>,
    D: Ord + Clone,
{
    type Channel = P::Channel;

    type Time = P::Time;

    type Duty = P::Duty;

    fn disable(&mut self, channel: Self::Channel) {
        self.pwm.disable(channel);
    }

    fn enable(&mut self, channel: Self::Channel) {
        self.pwm.enable(channel);
    }

    fn get_period(&self) -> Self::Time {
        self.pwm.get_period()
    }

    fn get_duty(&self, channel: Self::Channel) -> Self::Duty {
        self.pwm.get_duty(channel)
    }

    fn get_max_duty(&self) -> Self::Duty {
        self.pwm.get_max_duty()
    }

    fn set_duty(&mut self, channel: Self::Channel, duty: Self::Duty) {
        self.pwm.set_duty(
            channel,
            duty.min(self.max_duty.clone()).max(self.min_duty.clone()),
        )
    }

    fn set_period<P2>(&mut self, period: P2)
    where
        P2: Into<Self::Time>,
    {
        self.pwm.set_period(period)
    }
}

impl<P, D> PwmPin for Esc<P, D>
where
    P: PwmPin<Duty = D>,
    D: Ord + Clone,
{
    type Duty = D;

    fn disable(&mut self) {
        self.pwm.disable()
    }

    fn enable(&mut self) {
        self.pwm.enable()
    }

    fn get_duty(&self) -> Self::Duty {
        self.pwm.get_duty()
    }

    fn get_max_duty(&self) -> Self::Duty {
        self.pwm.get_max_duty()
    }

    fn set_duty(&mut self, duty: Self::Duty) {
        self.pwm
            .set_duty(duty.min(self.max_duty.clone()).max(self.min_duty.clone()))
    }
}
