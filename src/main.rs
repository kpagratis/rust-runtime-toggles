use std::{thread, time};

use rust_runtime_toggles::Toggles;

fn main() {
    // Statements here are executed when the compiled binary is called

    // Print text to the console
    println!("Hello World!");
    let toggles = Toggles::new("toggle.yaml");

    let _thread = Toggles::start(&toggles);

    loop {
        thread::sleep(time::Duration::from_secs(1));
        println!("{}", toggles.is_available("halfOn"));
    }
}