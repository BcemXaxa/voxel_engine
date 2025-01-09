use std::collections::HashMap;

use winit::{
    event::{ElementState, KeyEvent},
    keyboard::{KeyCode, PhysicalKey},
};

#[derive(Default)]
pub struct KeyInputHelper {
    map: HashMap<KeyCode, ElementState>,
}

impl KeyInputHelper {
    pub fn input(&mut self, event: KeyEvent) {
        if event.repeat {
            return;
        }

        match event.state {
            ElementState::Pressed => {
                if let PhysicalKey::Code(key_code) = event.physical_key {
                    self.map.insert(key_code, event.state);
                }
            }
            ElementState::Released => {
                if let PhysicalKey::Code(key_code) = event.physical_key {
                    self.map.remove(&key_code);
                }
            }
        }
    }

    pub fn is_pressed(&self, key_code: KeyCode) -> bool {
        self.map
            .get(&key_code)
            .unwrap_or(&ElementState::Released)
            .is_pressed()
    }
}
