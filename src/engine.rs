//! Core game engine implementation containing the main loop, Object management,
//! and systems for input processing, rendering, and event handling.

use std::{collections::HashSet, io::Write, time::{Duration, Instant}};
use crate::{event::{EngineEvent, EventBus}, game_object::GameObject, input, renderer::Renderer};
use windows::Win32::{Foundation::INVALID_HANDLE_VALUE, System::Console:: {
    GetConsoleMode, GetStdHandle, SetConsoleMode, CONSOLE_MODE, ENABLE_VIRTUAL_TERMINAL_PROCESSING, STD_OUTPUT_HANDLE
}};

/// Commands that can be issued to advise the engine what to do.
#[derive(Debug)]
pub enum EngineCommand {
    /// Spawn a new game object into the scene
    SpawnObject(GameObject),
    /// Remove a game object by its index in the objects list
    DespawnObject(usize),
    /// Move an existing game object by specified delta coordinates
    MoveObject(usize, i32, i32),
    /// Signal the engine to begin shutdown process
    Quit,
}

/// Trait for systems that can update game state each frame
pub trait Updatable {
    /// Main update method called every frame
    /// 
    /// # Arguments
    /// * `delta_time` - Time since last update in seconds
    /// * `active_keys` - Set of currently pressed keyboard keys
    ///
    /// # Returns
    /// Vector of engine commands to be processed this frame
    fn update(&mut self, delta_time: f32, active_keys: &HashSet<input::Key>) ->Vec<EngineCommand>;
}

/// Main game engine managing all game state and systems
pub struct Engine {
    /// Engine running state flag
    running: bool,
    /// Rendering system handle
    pub renderer: Renderer,
    /// Collection of active game objects
    pub objects: Vec<GameObject>,
    /// Registered update systems
    updatables: Vec<Box<dyn Updatable>>,
    /// Command queue for frame processing
    commands: Vec<EngineCommand>,
    /// Event distribution system
    pub event_bus: EventBus,
    /// Keyboard state from previous frame
    previous_keys: HashSet<input::Key>,
     /// Current keyboard state
    active_keys: HashSet<input::Key>,
}

impl Engine {

    /// Creates a new engine instance with specified render dimensions
    ///
    /// # Arguments
    /// * `width` - Width of the render surface in characters
    /// * `height` - Height of the render surface in characters
    ///
    /// # Example
    /// ```
    /// let mut engine = Engine::new(80, 24);
    /// ```
    pub fn new(width: usize, height: usize) -> Self {
        Self { 
            running: true,
            renderer: Renderer::new(width, height),
            objects: Vec::new(),
            updatables: Vec::new(),
            commands: Vec::new(),
            event_bus: EventBus::new(),
            previous_keys: HashSet::new(),
            active_keys: HashSet::new(),
        }
    }

    /// Registers a new updatable system
    ///
    /// # Arguments
    /// * `updatable` - System implementing the Updatable trait
    pub fn add_updatable(&mut self, updatable: impl Updatable + 'static) {
        self.updatables.push(Box::new(updatable));
    }

    /// Main game loop entry point
    ///
    /// Handles initialization, runs the game loop at ~30 FPS,
    /// and performs cleanup when finished
    pub fn run(&mut self) {
        self.init_terminal();

        let mut last_update = Instant::now();
        while self.is_running() {
            self.process_input();

            // Calculate delta time
            let delta_time = last_update.elapsed().as_secs_f32();
            last_update = Instant::now();

            self.update(delta_time);
            self.render();

            // Limit to ~30FPS
            let frame_duration = Duration::from_millis(33);
            let elapsed = Instant::now().duration_since(last_update);
            if elapsed < frame_duration {
                std::thread::sleep(frame_duration - elapsed);
            }
        }

        self.cleanup_terminal();
    }

    fn init_terminal(&self) {
        unsafe {
            let h_stdout = GetStdHandle(STD_OUTPUT_HANDLE);
            if h_stdout == INVALID_HANDLE_VALUE {
                return;
            }

            let mut mode = CONSOLE_MODE(0);
            if GetConsoleMode(h_stdout, &mut mode).as_bool() {
                let new_mode = mode | ENABLE_VIRTUAL_TERMINAL_PROCESSING;
                SetConsoleMode(h_stdout, new_mode);
            }
        }

        // Clear screen and hide cursor
        print!("\x1B[2J\x1B[?25l");
        let _ = std::io::stdout().flush();
    }

    fn process_input(&mut self) {
        self.active_keys = input::read_active_keys().unwrap_or_default();
    }

    fn detect_key_transitions(&mut self) {
        // Detect pressed key
        for key in &self.active_keys {
            if !self.previous_keys.contains(key) {
                self.event_bus.emit(EngineEvent::KeyPressed(key.clone()));
            }
        }

        // Detect key being held
        for key in self.active_keys.intersection(&self.previous_keys) {
            self.event_bus.emit(EngineEvent::KeyHeld(key.clone()));
        }

        // Detect released keys
        for key in &self.previous_keys {
            if !self.active_keys.contains(key) {
                self.event_bus.emit(EngineEvent::KeyReleased(key.clone()));
            }
        }
    }

    fn update(&mut self, delta_time: f32) {
        self.detect_key_transitions();
        self.previous_keys = self.active_keys.clone();
        
        // Clear previous commands
        self.commands.clear();

        // Process animations.
        for obj in &mut self.objects {
            if obj.frames.len() > 1 {
                obj.animation_timer += delta_time;
                if obj.animation_timer >= obj.frame_duration {
                    obj.current_frame = (obj.current_frame +1) % obj.frames.len();
                    obj.character = obj.frames[obj.current_frame];
                    obj.animation_timer = 0.0;
                }
            }
        }

        // Run all registered updatable system.
        for updatable in &mut self.updatables {
            let new_commands = updatable.update(delta_time, &self.active_keys);
            self.commands.extend(new_commands);
        }

        // Process all queued commands
        let commands = std::mem::take(&mut self.commands);
        for command in commands {
            match command {
                EngineCommand::SpawnObject(obj) => self.add_object(obj),
                EngineCommand::DespawnObject(index) => {
                    if index < self.objects.len() {
                        self.objects.remove(index);
                    }
                },
                EngineCommand::MoveObject(index, dx, dy) => {
                    if let Some(obj) = self.objects.get_mut(index) {
                        let new_x= (obj.x as i32 + dx).clamp(0, self.renderer.get_width() as i32 - 1) as usize;
                        let new_y = (obj.y as i32 + dy).clamp(0, self.renderer.get_height() as i32 - 1) as usize;

                        obj.x = new_x;
                        obj.y = new_y;

                        self.event_bus.emit(EngineEvent::ObjectMoved(index, new_x, new_y));
                    }
                },
                EngineCommand::Quit => self.stop(),
            }
        }
    }

    fn render(&mut self) {
        self.renderer.clear_back_buffer();

        for obj in &self.objects {
            self.renderer.set_char(obj.x, obj.y, obj);
        }

        let _ = self.renderer.present();
    }

    /// Adds a game object to the engine's object collection
    /// 
    /// # Arguments
    /// * `obj` - The [`GameObject`] to add to the scene
    /// 
    /// # Notes
    /// - The object will be rendered starting on the next frame
    /// - Object will participate in animation system updates
    /// - Object index is determined by insertion order
    /// 
    /// # Example
    /// ```
    /// let mut engine = Engine::new(80, 24);
    /// let player = GameObject::new(10, 5, '@');
    /// engine.add_object(player);
    /// ```
    /// 
    /// [`GameObject`]: crate::game_object::GameObject
    pub fn add_object(&mut self, obj: GameObject) {
        self.objects.push(obj);
    }

    /// Returns whether the egnie is still running.
    pub fn is_running(&self) -> bool {
        self.running
    }

    //Stops the engine
    pub fn stop(&mut self) {
        self.running = false;
    }

    fn cleanup_terminal(&self) {
        // Reset terminal state
        print!("\x1B[2J\x1B[?25h");
        let _ = std::io::stdout().flush();
    }
}
