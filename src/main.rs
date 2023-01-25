use std::{thread, time};

use rust_runtime_toggles::{Toggle};

fn main() {
    let notify_toggles = Toggle::new("toggle.yaml");
    Toggle::start(&notify_toggles);
    loop {
        thread::sleep(time::Duration::from_secs(1));
        println!("{}", notify_toggles.is_available("halfOn"));
    }
}