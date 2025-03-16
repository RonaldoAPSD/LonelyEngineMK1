//! Game object representation and management
//!
//! Contains the [`GameObject`] struct that represents entities in the game world,
//! including their visual representation, animation, and positioning.

/// Represents an entity in the game world with visual and spatial properties
///
/// # Fields
/// - `x`, `y`: Grid position coordinates (zero-based)
/// - `character`: Default display character
/// - `tag`: Identifier for grouping/classification
/// - `frames`: Animation frame sequence
/// - `current_frame`: Index of current animation frame
/// - `frame_duration`: Time (seconds) between frame changes
/// - `animation_timer`: Accumulated time since last frame change
/// - `fg_color`: Optional ANSI foreground color code
/// - `bg_color`: Optional ANSI background color code
///
/// # Examples
/// ```
/// use lonely_engine::game_object::GameObject;
///
/// // Create a basic stationary object
/// let player = GameObject::new(5, 10, '@');
///
/// // Create an animated object with colors
/// let mut torch = GameObject::new(8, 3, '|');
/// torch.frames = vec!['|', '/', '─', '\\'];
/// torch.frame_duration = 0.2;
/// torch.fg_color = Some("\x1B[38;5;208m".to_string()); // Orange
/// ```
#[derive(Debug, Clone)]
pub struct GameObject {
    /// Horizontal position in grid cells
    pub x: usize,
    /// Vertical position in grid cells
    pub y: usize,
    /// Default display character
    pub character: char,
    /// Object identifier/category
    pub tag: String,
    /// Animation sequence (requires frame_duration > 0)
    pub frames: Vec<char>,
    /// Current animation frame index
    pub current_frame: usize,
    /// Time between automatic frame changes (seconds)
    pub frame_duration: f32,
    /// Accumulated time since last frame change
    pub animation_timer: f32,
    /// ANSI foreground color escape code
    pub fg_color: Option<String>,
    /// ANSI background color escape code
    pub bg_color: Option<String>,
}

impl GameObject {
    /// Creates a new GameObject with default configuration
    ///
    /// # Arguments
    /// * `x` - Initial grid X position
    /// * `y` - Initial grid Y position
    /// * `character` - Display character
    ///
    /// # Defaults
    /// - `tag`: Empty string
    /// - Single-frame animation using `character`
    /// - `frame_duration`: 0.1 seconds
    /// - No colors set
    ///
    /// # Example
    /// ```
    /// use lonely_engine::game_object::GameObject;
    ///
    /// // Create a player object at position (5, 10)
    /// let player = GameObject::new(5, 10, '@');
    ///
    /// // Create a colored projectile
    /// let mut fireball = GameObject::new(8, 3, '*');
    /// fireball.fg_color = Some("\x1B[31m".to_string()); // Red
    /// fireball.frames = vec!['*', '●', '○']; // 3-frame animation
    /// fireball.frame_duration = 0.05; // Fast animation
    /// ```
    pub fn new(x: usize, y: usize, character: char) -> Self {
        Self { 
            x, y, 
            character,
            tag: String::new(),
            frames: vec![character],
            current_frame: 0,
            frame_duration: 0.1,
            animation_timer: 0.0,
            fg_color: None,
            bg_color: None,
        }
    }
}