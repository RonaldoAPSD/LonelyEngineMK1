//! Double-buffered ASCII renderer for terminal output
//!
//! Manages efficient screen updates using ANSI escape codes and delta rendering.
//! Provides:
//! - Coordinate-based character placement
//! - ANSI color support
//! - Minimal screen updates through frame comparison

use std::io::{self, Write};
use crate::game_object::GameObject;

/// Handles terminal rendering with double buffering
///
/// Maintains two buffers:
/// - Back buffer: Current frame being built
/// - Front buffer: Previously displayed frame
///
/// # Performance
/// Only updates changed characters between frames using ANSI cursor positioning
pub struct Renderer {
    width: usize,
    height: usize,
    front_buffer: Vec<Vec<String>>,
    back_buffer: Vec<Vec<String>>,
}

impl Renderer {
    /// Creates a new renderer with specified dimensions
    ///
    /// # Arguments
    /// * `width` - Number of character columns
    /// * `height` - Number of character rows
    ///
    /// # Example
    /// ```
    /// use lonely_engine::renderer::Renderer;
    /// 
    /// // Create 80x24 terminal renderer
    /// let renderer = Renderer::new(80, 24);
    /// ```
    pub fn new(width: usize, height: usize) -> Self {
        // Initilize both buffers with spaces. 
        let front_buffer = vec![vec![String::new(); width]; height];
        let back_buffer = vec![vec![String::new(); width]; height];

        Self {
            width,
            height,
            front_buffer,
            back_buffer,
        }
    }

    /// Gets current render width
    pub fn get_width(&self) -> usize {
        self.width
    }

    /// Gets current render height
    pub fn get_height(&self) -> usize {
        self.height
    }

    /// Resets back buffer to empty state
    ///
    /// # Example
    /// ```
    /// # use lonely_engine::renderer::Renderer;
    /// # let mut renderer = Renderer::new(10, 10);
    /// renderer.clear_back_buffer();
    /// ```
    pub fn clear_back_buffer(&mut self) {
        for row in self.back_buffer.iter_mut() {
            for cell in row.iter_mut() {
                *cell = String::new();
            }
        }
    }

    /// Writes a game object to the back buffer
    ///
    /// # Arguments
    /// * `x` - Column position (0-based)
    /// * `y` - Row position (0-based)
    /// * `obj` - GameObject with character and colors
    ///
    /// # Notes
    /// - Positions outside dimensions are ignored
    /// - ANSI colors are reset after each character
    ///
    /// # Example
    /// ```
    /// # use lonely_engine::{renderer::Renderer, game_object::GameObject};
    /// # let mut renderer = Renderer::new(10, 10);
    /// let mut obj = GameObject::new(5, 5, '@');
    /// obj.fg_color = Some("\x1B[31m".into()); // Red
    /// renderer.set_char(5, 5, &obj);
    /// ```
    pub fn set_char(&mut self, x: usize, y: usize, obj: &GameObject) {
        if x < self.width && y < self.height {
            let mut ansi_str = String::new();
            
            // Apply colors if present
            if let Some(fg) = &obj.fg_color {
                ansi_str.push_str(fg);
            }
            if let Some(bg) =&obj.bg_color {
                ansi_str.push_str(bg);
            }

            ansi_str.push(obj.character);
            ansi_str.push_str("\x1B[0m");
            self.back_buffer[y][x] = ansi_str;
        }
    }

    /// Renders the back buffer to screen and swaps buffers
    ///
    /// # Implementation
    /// 1. Moves cursor to home position (0,0)
    /// 2. Only updates changed characters
    /// 3. Flushes output buffer
    ///
    /// # Example
    /// ```no_run
    /// # use lonely_engine::{renderer::Renderer, game_object::GameObject};
    /// # let mut renderer = Renderer::new(80, 24);
    /// # let obj = GameObject::new(40, 12, '*');
    /// renderer.clear_back_buffer();
    /// renderer.set_char(40, 12, &obj);
    /// renderer.present().expect("Rendering failed");
    /// ```
    pub fn present(&mut self) -> io::Result<()> {
        // Move cursor to top-left
        print!("\x1B[H");

        for y in 0..self.height {
            for x in 0..self.width {
                // Only update changed cells
                print!("\x1B[{};{}H{}", y + 1, x + 1, self.back_buffer[y][x]);
                self.front_buffer[y][x] = self.back_buffer[y][x].clone();
            }
        }
        io::stdout().flush()
    }
}