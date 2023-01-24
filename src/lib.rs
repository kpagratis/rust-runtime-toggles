use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::sync::{Arc, RwLock};
use std::time::Duration;

use clokwerk::{ScheduleHandle, Scheduler, TimeUnits};
use rand::Rng;
use serde_derive::Deserialize;

pub struct Toggles {
    config_file_path: String,
    data: Arc<RwLock<ToggleData>>,
    update_duration: Duration,
}

pub struct ToggleData {
    toggles: HashMap<String, f32>,
    loaded: bool,
}

impl Default for ToggleData {
    fn default() -> Self {
        ToggleData {
            toggles: HashMap::new(),
            loaded: false,
        }
    }
}

impl ToggleData {
    fn update_values(&mut self, config_file_path: &String) {
        self.loaded = true;
        let mut contents: String = String::new();
        File::open(&config_file_path)
            .expect(format!("{} could not be opened", config_file_path).as_str())
            .read_to_string(&mut contents)
            .expect(format!("Unable to read {}", config_file_path).as_str());

        let d: HashMap<String, YamlToggleItem> = serde_yaml::from_str(&contents).unwrap();

        self.toggles = d.iter().map(|(key, value)| {
            println!("setting {}, description: {} to value: {}", key, value.value, value.description);
            (String::from(key), value.value)
        }).collect();
    }
}

#[derive(Deserialize)]
pub struct YamlToggleItem {
    value: f32,
    description: String,
}

impl Toggles {
    pub fn new(config_file_path: &str) -> Toggles {
        Toggles {
            config_file_path: config_file_path.to_string(),
            update_duration: Duration::from_secs(10),
            data: Arc::new(RwLock::new(ToggleData::default())),
        }
    }

    pub fn new_with_duration(config_file_path: &str, update_duration: Duration) -> Toggles {
        Toggles {
            update_duration,
            config_file_path: config_file_path.to_string(),
            data: Arc::new(RwLock::new(ToggleData::default())),
        }
    }

    pub fn start(toggles: &Toggles) -> Option<ScheduleHandle> {
        let config_file = String::from(&toggles.config_file_path);
        let seconds: u32 = toggles.update_duration.as_secs() as u32;

        let clone = toggles.data.clone();
        let mut write = clone.write().unwrap();

        if write.loaded {
            return None;
        }

        let mut _scheduler = Scheduler::new();
        let clone: Arc<RwLock<ToggleData>> = toggles.data.clone();

        write.update_values(&config_file);

        _scheduler
            .every(seconds.seconds())
            .run(move || {
                clone.write().unwrap().update_values(&config_file);
            });
        Some(_scheduler.watch_thread(Duration::from_millis(0)))
    }

    pub fn is_available(&self, toggle_name: &str) -> bool {
        let r: f32 = rand::thread_rng().gen();
        let read = self.data.read().unwrap();
        let binding: Option<&f32> = read.toggles.get(&toggle_name.to_string());
        match binding {
            Some(v) => &r <= v,
            _ => false
        }
    }
}
