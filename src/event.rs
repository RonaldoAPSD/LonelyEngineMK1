use crate::input::Key;

#[derive(Debug, Clone)]
pub enum EngineEvent {
    ObjectSpawned(usize),
    ObjectMoved(usize, usize, usize),
    InputRecieved(Key),
    KeyPressed(Key),
    KeyHeld(Key),
    KeyReleased(Key),
    Custom(String), // User-defined events
}

pub struct EventBus {
    subscribers: Vec<Box<dyn Fn(&EngineEvent) -> ()>>,
}

impl EventBus {
    pub fn new() -> Self {
        Self { subscribers: Vec::new() }
    }

    pub fn subscribe(&mut self, callback: impl Fn(&EngineEvent) -> () + 'static) {
        self.subscribers.push(Box::new(callback));
    }

    pub fn emit(&self, event: EngineEvent) {
        for callback in &self.subscribers {
            callback(&event);
        }
    }
}