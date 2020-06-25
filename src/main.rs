use rppal::gpio::{Gpio, OutputPin};
use std::error::Error;
use std::time::{Duration, Instant};
use time::{OffsetDateTime, Time};

#[macro_use]
extern crate time;

const GPIO_LIGHTS: u8 = 27;
const LIGHTS_ON_TIME: Time = time!(7:00);
const LIGHTS_OFF_TIME: Time = time!(21:00);

const GPIO_MIST: u8 = 22;
const MIST_ON_TIME: Time = time!(19:00);
const MIST_OFF_TIME: Time = time!(19:01);

const GPIO_FOG: u8 = 17;
const FOG_ON_INTERVAL: Duration = Duration::from_secs(60 * 10);
const FOG_OFF_INTERVAL: Duration = Duration::from_secs(60 * 30);

struct Lights {
    pin: OutputPin,
}

impl Lights {
    fn new() -> Result<Self, Box<dyn Error>> {
        Ok(Lights {
            pin: Gpio::new()?.get(GPIO_LIGHTS)?.into_output(),
        })
    }

    fn update(&mut self) {
        let now = OffsetDateTime::now_local().time();
        if now > LIGHTS_ON_TIME && now < LIGHTS_OFF_TIME {
            self.pin.set_high();
        } else {
            self.pin.set_low();
        }
    }
}

struct Mist {
    pin: OutputPin,
}

impl Mist {
    fn new() -> Result<Self, Box<dyn Error>> {
        Ok(Mist {
            pin: Gpio::new()?.get(GPIO_MIST)?.into_output(),
        })
    }

    fn update(&mut self) {
        let now = OffsetDateTime::now_local().time();
        // set low for the relay means on
        if now > MIST_ON_TIME && now < MIST_OFF_TIME {
            self.pin.set_low();
        } else {
            self.pin.set_high();
        }
    }
}

struct Fog {
    pin: OutputPin,
    last_switch: Instant,
}

impl Fog {
    fn new() -> Result<Self, Box<dyn Error>> {
        let mut pin = Gpio::new()?.get(GPIO_FOG)?.into_output();
        let last_switch = Instant::now();
        pin.set_low();

        Ok(Fog { pin, last_switch })
    }

    fn update(&mut self) {
        let elapsed = Instant::now().duration_since(self.last_switch);

        // set low for the relay means on
        if self.pin.is_set_low() && elapsed > FOG_OFF_INTERVAL {
            self.pin.set_high();
            self.last_switch = Instant::now();
        } else if self.pin.is_set_high() && elapsed > FOG_ON_INTERVAL {
            self.pin.set_low();
            self.last_switch = Instant::now();
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut fog = Fog::new()?;
    let mut mist = Mist::new()?;
    let mut lights = Lights::new()?;
    let time = OffsetDateTime::now_local().time().format("%r");
    print!("{}", time);
    loop {
        fog.update();
        mist.update();
        lights.update();
        std::thread::sleep(Duration::from_secs(3));
    }
}
