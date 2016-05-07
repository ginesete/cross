use std::thread::sleep;
use std::time::Duration;

mod gpio;
use gpio::*;

// This main function illustrates how to access a GPIO resource through
// sysfs, using the provided mod gpio.
fn main() {
    let pin = GPIO::new(26);
    pin.initialize();
    pin.set_mode(Mode::Out);
    pin.set_value(PinValue::High);
    sleep(Duration::from_millis(3000));
    pin.set_value(PinValue::Low);
    pin.release();
}

#[cfg(test)]
mod tests {
    use gpio::*;

// Guess what happens when we try to run all these unit tests on a
// shared hardware resource...
    #[test]
    #[ignore]
    fn tries_to_open_and_close_gpio_port() {
        let pin = GPIO::new(26);
        pin.initialize();
        pin.release();
    }

    #[test]
    #[ignore]
    #[should_panic]
    fn double_opening_a_port_should_fail() {
        let pin = GPIO::new(26);
        pin.initialize();
        pin.initialize();
    }

    #[test]
    #[ignore]
    #[should_panic]
    fn double_releasing_a_port_should_fail() {
        let pin = GPIO::new(26);
        pin.release();
        pin.release();
    }
}
