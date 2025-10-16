//This is singleton pattern example
use std::sync::OnceLock;
use std::sync::Mutex;
pub struct Config {
    pub value: i32,
}

pub struct SingletonMutableConfig {
    pub value: i32,
}

impl SingletonMutableConfig {
    pub fn new() -> Self {
        SingletonMutableConfig { value: 0 }
    }
    pub fn get_instance(&self) -> &'static Mutex<SingletonMutableConfig> {
        static MUTABLE_INSTANCE: OnceLock<Mutex<SingletonMutableConfig>> = OnceLock::new();
        MUTABLE_INSTANCE.get_or_init(|| {
            Mutex::new(SingletonMutableConfig::new())
        })
    }
}

pub static INSTANCE: OnceLock<Config> = OnceLock::new();