use std::io::{self, Write};

use crate::game_object::GameObject;

/// Renderer struct that manages double buffering for efficient ASCII rendering.
pub struct Renderer {
    width: usize,
    height: usize,
    front_buffer: Vec<Vec<String>>,
    back_buffer: Vec<Vec<String>>,
}

impl Renderer {
    /// Create a new Renderer with the given width and height.
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

    pub fn get_width(&self) -> usize {
        self.width
    }

    pub fn get_height(&self) -> usize {
        self.height
    }

    /// Clears the back buffer to a blank state.
    pub fn clear_back_buffer(&mut self) {
        for row in self.back_buffer.iter_mut() {
            for cell in row.iter_mut() {
                *cell = String::new();
            }
        }
    }

    /// Set a character in the back buffer at (x, y).
    pub fn set_char(&mut self, x: usize, y: usize, obj: &GameObject) {
        if x < self.width && y < self.height {
            let mut ansi_str = String::new();
            
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

    /// Present the back buffer to the screen.
    /// Compare the back buffer to front buffer,
    /// only draws characters that have changed,
    /// if there is a change clear the back buffer.
    pub fn present(&mut self) -> io::Result<()> {
        print!("\x1B[H"); // ANSI: Move cursor to top-left
        for y in 0..self.height {
            for x in 0..self.width {
                print!("\x1B[{};{}H{}", y + 1, x + 1, self.back_buffer[y][x]);
                self.front_buffer[y][x] = self.back_buffer[y][x].clone();
            }
        }
        io::stdout().flush()
    }
}