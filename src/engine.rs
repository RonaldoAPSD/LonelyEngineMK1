//! Central engine class managing game loop and ECS
use std::{collections::HashSet, io::Write, time::{Duration, Instant}};
use crate::{event::{EngineEvent, EventBus}, game_object::GameObject, input, renderer::Renderer};
use windows::Win32::{Foundation::INVALID_HANDLE_VALUE, System::Console:: {
    GetConsoleMode, GetStdHandle, SetConsoleMode, CONSOLE_MODE, ENABLE_VIRTUAL_TERMINAL_PROCESSING, STD_OUTPUT_HANDLE
}};

#[derive(Debug)]
pub enum EngineCommand {
    SpawnObject(GameObject),
    DespawnObject(usize),
    MoveObject(usize, i32, i32),
    Quit,
}

pub trait Updatable {
    fn update(&mut self, delta_time: f32, active_keys: &HashSet<input::Key>) ->Vec<EngineCommand>;
}

/// Main engine class
pub struct Engine {
    running: bool,
    pub renderer: Renderer,
    pub objects: Vec<GameObject>,
    updatables: Vec<Box<dyn Updatable>>,
    commands: Vec<EngineCommand>,
    pub event_bus: EventBus,
    previous_keys: HashSet<input::Key>, 
    active_keys: HashSet<input::Key>,
}

impl Engine {
    /// Initializes engine with specified terminal dimensions
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

    pub fn add_updatable(&mut self, updatable: impl Updatable + 'static) {
        self.updatables.push(Box::new(updatable));
    }

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
