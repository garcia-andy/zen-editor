use std::{collections::HashMap, sync::{Mutex, MutexGuard, OnceLock}};
use registers::Event;

#[derive(Debug,Clone)]
pub struct KeyBinding {
    pub key: char,
    pub ctrl: bool,
    pub shift: bool,
    pub alt: bool,
    pub event: Event,
}

impl KeyBinding {
    pub fn new(key: char, ctrl: bool, shift: bool, alt: bool, event: Event) -> Self {
        Self {
            key,
            ctrl,
            shift,
            alt,
            event,
        }
    }
}

pub fn get_global_hashmap() -> MutexGuard<'static, HashMap<char, Vec<KeyBinding>>> {
    static MAP_KEYS: OnceLock<Mutex<HashMap<char, Vec<KeyBinding>>>> = OnceLock::new();
    MAP_KEYS.get_or_init(|| Mutex::new(HashMap::new()))
        .lock()
        .expect("Let's hope the lock isn't poisoned")
}