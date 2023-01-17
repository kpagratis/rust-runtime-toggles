use std::sync::{Arc, RwLock};
use std::{thread, time};
use std::time::Duration;
use rust_runtime_toggles::Toggles;

fn main() {
    // Statements here are executed when the compiled binary is called

    // Print text to the console
    println!("Hello World!");
    let toggles: Arc<RwLock<Toggles>> = Arc::new(
        RwLock::new(
            Toggles::new_with_duration("toggle.yaml", Duration::from_secs(1))
        )
    );

    let _thread = Toggles::start(&toggles);

    loop {
        thread::sleep(time::Duration::from_secs(1));
        println!("{}", toggles.read().unwrap().is_available("halfOn"));
    }
}