use std::fs::OpenOptions;
use std::io::prelude::*;
use std::thread::sleep;
use std::time::Duration;

// This mod is rather weak, and it is only useful for illustrative purposes.
// For instance, it is vulnerable to panicking when a double initialization
// or a double release occur. This way, it is easy to learn how sysfs is
// behaving looking in the console, and fixing errors by hand to see how
// it is done.

#[derive(Debug)]
pub struct GPIO {
    pin: u32,
}

impl GPIO {
    pub fn new(pin: u32) -> GPIO { GPIO { pin: pin } }
    pub fn initialize(&self) {
        self.export();
        sleep(Duration::from_millis(50));
    }

    pub fn release(&self) {
        self.unexport();
        sleep(Duration::from_millis(50));
    }

    pub fn set_mode(&self, mode: Mode) {
        let sysfs_path = format!("/sys/class/gpio/gpio{}/direction", self.pin);

        self.write_to_sysfs(
            sysfs_path,
            format!("{}", mode.get_path()).as_bytes());
        println!("PIN {} Mode {:?}", self.pin, mode);
    }

    pub fn set_value(&self, value: PinValue) {
        let sysfs_path = format!("/sys/class/gpio/gpio{}/value", self.pin);

        self.write_to_sysfs(
            sysfs_path,
            format!("{}", value.get_value()).as_bytes());
        println!("PIN {} Value {:?}", self.pin, value);
    }

    fn export(&self) {
        self.write_to_sysfs(
            format!("/sys/class/gpio/export"),
            format!("{}", self.pin).as_bytes());
        println!("PIN {} Exported", self.pin);
    }

    fn unexport(&self) {
        self.write_to_sysfs(
            format!("/sys/class/gpio/unexport"),
            format!("{}", self.pin).as_bytes());
        println!("PIN {} Unexported", self.pin);
    }

    fn write_to_sysfs(&self, file_path: String, buf: &[u8]) {
        let mut file = OpenOptions::new()
            .write(true)
            .append(true)
            .open(file_path)
            .unwrap();

        // Very weak. Good for our learning purposes regarding sysfs
        file.write_all(buf).expect("Write error");

        // A bit more failsafe. At least does not fail when resource is busy
        // match file.write_all(buf) {
        //     Ok(_) => {},
        //     Err(_) => {}
        // }
    }
}

#[derive(Debug)]
#[allow(dead_code)]
pub enum PinValue {
    High,
    Low,
}

impl PinValue {
    pub fn get_value(&self) -> String {
        match *self {
            PinValue::High => "1".to_string(),
            PinValue::Low => "0".to_string(),
        }
    }
}

#[derive(Debug)]
#[allow(dead_code)]
pub enum Mode {
    In,
    Out,
}

impl Mode {
    pub fn get_path(&self) -> String {
        match *self {
            Mode::In => "in".to_string(),
            Mode::Out => "out".to_string(),
        }
    }
}
