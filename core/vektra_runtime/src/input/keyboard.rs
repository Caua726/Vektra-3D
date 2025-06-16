//! Guarda o estado das teclas pressionadas (API winit 0.30).

use std::collections::HashSet;
use winit::{
    event::{ElementState, RawKeyEvent},
    keyboard::{KeyCode, PhysicalKey},
};

#[derive(Default)]
pub struct Keyboard {
    held: HashSet<KeyCode>,
}

impl Keyboard {
    /// Atualiza estado a partir de `RawKeyEvent` (de DeviceEvent).
    pub fn process_input(&mut self, ev: &RawKeyEvent) {
        if let PhysicalKey::Code(code) = ev.physical_key {
            match ev.state {
                ElementState::Pressed => {
                    self.held.insert(code);
                }
                ElementState::Released => {
                    self.held.remove(&code);
                }
            }
        }
    }

    /// Está pressionada?
    pub fn pressed(&self, key: KeyCode) -> bool {
        self.held.contains(&key)
    }
}
