use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::sync::{Arc, RwLock, RwLockReadGuard};
use std::time::Duration;

use clokwerk::{ScheduleHandle, Scheduler, TimeUnits};
use rand::Rng;
use serde_derive::Deserialize;

pub struct Toggles {
    config_file_path: String,
    update_duration: Duration,
    toggles: HashMap<String, f32>,
    loaded: bool
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
            toggles: HashMap::new(),
            loaded: false
        }
    }

    pub fn new_with_duration(config_file_path: &str, update_duration: Duration) -> Toggles {
        Toggles {
            config_file_path: config_file_path.to_string(),
            update_duration,
            toggles: HashMap::new(),
            loaded: false
        }
    }

    pub fn start(toggles: &Arc<RwLock<Toggles>>) -> Option<ScheduleHandle> {
        let read: RwLockReadGuard<Toggles> = toggles.read().unwrap();
        if read.loaded {
            return None;
        }
        let seconds: u32 = read.update_duration.as_secs() as u32;

        let mut _scheduler = Scheduler::new();
        let clone: Arc<RwLock<Toggles>> = toggles.clone();
        _scheduler
            .every(seconds.seconds())
            .run(move || {
                clone.write().unwrap().update_values();
            });
        Some(_scheduler.watch_thread(Duration::from_millis(0)))
    }

    pub fn is_available(&self, toggle_name: &str) -> bool {
        let r: f32 = rand::thread_rng().gen();
        let binding: Option<&f32> = self.toggles.get(&toggle_name.to_string());
        match binding {
            Some(v) => &r <= v,
            _ => false
        }
    }

    fn update_values(&mut self) {
        self.loaded = true;
        let mut contents: String = String::new();
        File::open(&self.config_file_path)
            .expect(format!("{} could not be opened", self.config_file_path).as_str())
            .read_to_string(&mut contents)
            .expect(format!("Unable to read {}", self.config_file_path).as_str());

        let d: HashMap<String, YamlToggleItem> = serde_yaml::from_str(&contents).unwrap();

        self.toggles = d.iter().map(|(key, value)| {
            println!("setting {}, description: {} to value: {}", key, value.value, value.description);
            (String::from(key), value.value)
        }).collect();
    }
}
