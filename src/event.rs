//! Event system implementation for engine communication
//!
//! Provides a publish-subscribe mechanism for game events using an event bus pattern.
//! Contains:
//! - [`EngineEvent`] enum defining all engine event types
//! - [`EventBus`] struct for managing event subscribers and dispatching

use crate::input::Key;

/// Enum representing all possible engine events
#[derive(Debug, Clone)]
pub enum EngineEvent {
    /// Emitted when a new game object is spawned.  
    /// Contains the object's index in the engine's objects list.  
    /// # Example
    /// ```rust
    /// # use lonely_engine::event::EngineEvent;
    /// let event = EngineEvent::ObjectSpawned(2);
    /// ```
    ObjectSpawned(usize),

    /// Emitted when an object changes position.  
    /// Contains (object index, new x, new y).  
    /// # Example
    /// ```rust
    /// # use lonely_engine::event::EngineEvent;
    /// let event = EngineEvent::ObjectMoved(0, 5, 10);
    /// ```
    ObjectMoved(usize, usize, usize),

    /// Emitted when any input is received (catch-all variant)
    /// # Example
    /// ```rust
    /// # use lonely_engine::{event::EngineEvent, input::Key};
    /// let event = EngineEvent::InputRecieved(Key::Right);
    /// ```
    InputRecieved(Key),

    /// Emitted on initial key press.  
    /// # Example
    /// ```rust
    /// # use lonely_engine::{event::EngineEvent, input::Key};
    /// let event = EngineEvent::KeyPressed(Key::Space);
    /// ```
    KeyPressed(Key),

    /// Emitted every frame while key is held.  
    /// # Example
    /// ```rust
    /// # use lonely_engine::{event::EngineEvent, input::Key};
    /// let event = EngineEvent::KeyHeld(Key::Ctrl);
    /// ```
    KeyHeld(Key),

    /// Emitted when key is released.  
    /// # Example
    /// ```rust
    /// # use lonely_engine::{event::EngineEvent, input::Key};
    /// let event = EngineEvent::KeyReleased(Key::Shift);
    /// ```
    KeyReleased(Key),

    /// Custom user-defined event payload.  
    /// # Example
    /// ```rust
    /// # use lonely_engine::event::EngineEvent;
    /// let event = EngineEvent::Custom("PowerupCollected:Fireball".into());
    /// ```
    Custom(String),
}

/// Central event bus for publish-subscribe communication.  
/// # Examples
/// 
/// **Basic Usage:**
/// ```rust
/// # use lonely_engine::{event::{EventBus, EngineEvent}, input::Key};
/// let mut bus = EventBus::new();
/// 
/// // Subscribe to events
/// bus.subscribe(|e| match e {
///     EngineEvent::KeyPressed(key) => {
///         println!("Key pressed: {:?}", key);
///     },
///     _ => {}
/// });
/// 
/// // Emit an event
/// bus.emit(EngineEvent::KeyPressed(Key::Enter));
/// ```
/// 
/// **Multiple Subscribers:**
/// ```rust
/// # use lonely_engine::{event::{EventBus, EngineEvent}, input::Key};
/// let mut bus = EventBus::new();
/// 
/// bus.subscribe(|e| if let EngineEvent::ObjectMoved(id, x, y) = e {
///     println!("Object {id} moved to ({x}, {y})");
/// });
/// 
/// bus.subscribe(|e| if let EngineEvent::Custom(text) = e {
///     println!("Custom event: {}", text);
/// });
/// 
/// bus.emit(EngineEvent::ObjectMoved(1, 10, 5));
/// bus.emit(EngineEvent::Custom("GameSaved".into()));
/// ```
pub struct EventBus {
    /// Creates a new empty EventBus.  
    /// # Example
    /// ```rust
    /// # use lonely_engine::event::EventBus;
    /// let bus = EventBus::new();
    /// ```
    subscribers: Vec<Box<dyn Fn(&EngineEvent) -> ()>>,
}

impl EventBus {
    /// Creates a new empty EventBus
    pub fn new() -> Self {
        Self { subscribers: Vec::new() }
    }

    /// Registers an event handler.  
    /// # Example
    /// ```rust
    /// # use lonely_engine::{event::{EventBus, EngineEvent}, input::Key};
    /// let mut bus = EventBus::new();
    /// 
    /// bus.subscribe(|event| {
    ///     if let EngineEvent::KeyReleased(Key::Esc) = event {
    ///         println!("Escape key released!");
    ///     }
    /// });
    /// ```
    pub fn subscribe(&mut self, callback: impl Fn(&EngineEvent) -> () + 'static) {
        self.subscribers.push(Box::new(callback));
    }

    /// Broadcasts an event to all subscribers.  
    /// # Example
    /// ```rust
    /// # use lonely_engine::{event::{EventBus, EngineEvent}, input::Key};
    /// # let mut bus = EventBus::new();
    /// // Notify all systems about game quit
    /// bus.emit(EngineEvent::Custom("GameQuit".into()));
    /// ```
    pub fn emit(&self, event: EngineEvent) {
        for callback in &self.subscribers {
            callback(&event);
        }
    }
}