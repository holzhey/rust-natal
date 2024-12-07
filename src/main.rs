#![no_std]
#![no_main]

use arduino_hal::port::{mode, Pin};
use arduino_hal::simple_pwm::{IntoPwmPin, PwmPinOps};
use panic_halt as _;

trait Slider {
    fn slide(&mut self, duty: u8);
}

impl<TC, PIN: PwmPinOps<TC, Duty = u8>> Slider for Pin<mode::PwmOutput<TC>, PIN> {
    fn slide(&mut self, duty: u8) {
        let current_duty = self.get_duty();
        if current_duty < duty {
            for d in current_duty..duty {
                self.set_duty(d);
                arduino_hal::delay_ms(10);
            }
        } else {
            for d in (duty..current_duty).rev() {
                self.set_duty(d);
                arduino_hal::delay_ms(10);
            }
        }
    }
}

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    let mut pwm_timer0 = arduino_hal::simple_pwm::Timer0Pwm::new(
        dp.TC0,
        arduino_hal::simple_pwm::Prescaler::Prescale64,
    );
    let mut pwm_timer2 = arduino_hal::simple_pwm::Timer2Pwm::new(
        dp.TC2,
        arduino_hal::simple_pwm::Prescaler::Prescale64,
    );

    let mut red = pins.d6.into_output().into_pwm(&mut pwm_timer0);
    let mut green = pins.d5.into_output().into_pwm(&pwm_timer0);
    let mut blue = pins.d3.into_output().into_pwm(&mut pwm_timer2);

    red.enable();
    green.enable();
    blue.enable();

    loop {
        green.slide(128);
        blue.slide(128);
        red.slide(128);
        green.slide(255);
        blue.slide(255);
        green.slide(0);
        red.slide(255);
        blue.slide(128);
        green.slide(255);
        blue.slide(0);
    }
}
