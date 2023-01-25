use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::sync::{Arc, Mutex, RwLock, RwLockWriteGuard};

use notify::{Event, RecommendedWatcher, Watcher};
use notify::event::DataChange::Content;
use notify::event::ModifyKind::Data;
use notify::EventKind::Modify;
use notify::RecursiveMode::NonRecursive;
use rand::{Rng, thread_rng};
use serde_derive::Deserialize;

#[derive(Debug)]
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
            let description: String = value.description.clone().unwrap_or("NO_DESCRIPTION".to_string());
            println!("setting {} ({}) to value: {}", key, description, value.value);
            (String::from(key), value.value)
        }).collect();
    }
}

#[derive(Deserialize)]
pub struct YamlToggleItem {
    value: f32,
    description: Option<String>,
}

#[derive(Debug)]
pub struct Toggle {
    config_file_path: String,
    watcher: Arc<Mutex<RecommendedWatcher>>,
    data: Arc<RwLock<ToggleData>>,
}

impl Toggle {
    pub fn new(config_file_path: &str) -> Toggle {
        let data: Arc<RwLock<ToggleData>> = Arc::new(RwLock::new(ToggleData::default()));
        let path: Arc<Mutex<String>> = Arc::new(Mutex::new(config_file_path.to_string()));
        Toggle {
            config_file_path: config_file_path.to_string(),
            data: data.clone(),
            watcher: Arc::new(Mutex::new(notify::recommended_watcher(move |res| {
                match res {
                    Ok(Event { kind: Modify(Data(Content)), .. }) => {
                        data.write().unwrap().update_values(&path.lock().unwrap().to_string());
                    }
                    Err(e) => println!("watch error: {:?}", e),
                    _ => (),
                }
            }).unwrap())),
        }
    }

    pub fn start(toggle: &Toggle) {
        let mut clone: RwLockWriteGuard<ToggleData> = toggle.data.write().unwrap();
        clone.update_values(&toggle.config_file_path);
        toggle.watcher.lock().unwrap().watch(Path::new(&toggle.config_file_path), NonRecursive).unwrap();
    }

    pub fn is_available(&self, toggle_name: &str) -> bool {
        let r: f32 = thread_rng().gen();
        let read = self.data.read().unwrap();
        let binding: Option<&f32> = read.toggles.get(&toggle_name.to_string());
        match binding {
            Some(v) => &r <= v,
            _ => false
        }
    }
}