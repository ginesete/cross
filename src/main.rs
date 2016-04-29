use std::thread::sleep;
use std::time::Duration;

mod gpio;
use gpio::*;

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
    use std::thread::sleep;
    use std::time::Duration;

    #[test]
    #[ignore]
    fn tries_to_open_and_close_gpio_port() {
        let pin = GPIO::new(26);
        pin.initialize();
        sleep(Duration::from_millis(50));
        pin.release();
        sleep(Duration::from_millis(50));
    }

    #[test]
    #[ignore]
    #[should_panic]
    fn double_opening_a_port_should_fail() {
        let pin = GPIO::new(26);
        pin.initialize();
        sleep(Duration::from_millis(50));
        pin.initialize();
        sleep(Duration::from_millis(50));
    }

    #[test]
    #[ignore]
    #[should_panic]
    fn double_releasing_a_port_should_fail() {
        let pin = GPIO::new(26);
        pin.release();
        sleep(Duration::from_millis(50));
        pin.release();
        sleep(Duration::from_millis(50));
    }
}
