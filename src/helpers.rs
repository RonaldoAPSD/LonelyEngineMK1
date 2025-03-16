//! Utility functions for common game operations
//!
//! Provides helper methods for:
//! - Collision detection
//! - Text rendering
//! - UI elements

use crate::{engine::Engine, game_object::GameObject};

/// Checks for simple grid-based collision between two GameObjects
///
/// # Arguments
/// * `a` - First game object
/// * `b` - Second game object
/// * `ignore_tags` - Tags to exclude from collision checks
///
/// # Returns
/// `true` if objects collide and neither has an ignored tag
///
/// # Notes
/// - Uses grid-based collision (same coordinates)
/// - Tags are case-sensitive
///
/// # Example
/// ```
/// # use lonely_engine::{helpers::check_collision, game_object::GameObject};
/// let mut obj1 = GameObject::new(5, 5, '@');
/// let mut obj2 = GameObject::new(5, 5, '#');
/// 
/// obj1.tag = "player".to_string();
/// obj2.tag = "wall".to_string();
///
/// assert!(check_collision(&obj1, &obj2, &[]));
/// assert!(!check_collision(&obj1, &obj2, &["player"]));
/// ```
pub fn check_collision(a: &GameObject, b: &GameObject, ignore_tags: &[&str]) -> bool {
    // Skip collision if either object has an ignored tag
    if ignore_tags.contains(&a.tag.as_str()) || ignore_tags.contains(&b.tag.as_str()) {
        return false;
    }

    // Simple AABB collision
    a.x == b.x && a.y == b.y
}

/// Renders text using GameObjects at specified coordinates
///
/// # Arguments
/// * `engine` - Engine instance to add objects to
/// * `x` - Starting X position (leftmost character)
/// * `y` - Y position (same for all characters)
/// * `text` - String to render
///
/// # Notes
/// - Characters are placed horizontally from left to right
/// - Creates temporary GameObjects without animation or colors
///
/// # Example
/// ```
/// # use lonely_engine::{helpers::draw_text, engine::Engine};
/// # let mut engine = Engine::new(80, 24);
/// // Draw "Score: 0" at position (10, 5)
/// draw_text(&mut engine, 10, 5, "Score: 0");
/// ```
pub fn draw_text (engine: &mut Engine, x: usize, y: usize, text: &str) {
    for (i, c) in text.chars().enumerate() {
        engine.add_object(GameObject::new(x + i, y, c));
    }
}

/// Draws a progress bar using GameObjects
///
/// # Arguments
/// * `engine` - Engine instance to add objects to
/// * `x` - Starting X position
/// * `y` - Y position
/// * `width` - Total bar width in characters
/// * `percent` - Fill percentage (0.0 to 1.0)
///
/// # Notes
/// - Uses '#' for filled portion
/// - Uses '-' for empty portion
/// - Automatically clamps percent between 0.0 and 1.0
///
/// # Example
/// ```
/// # use lonely_engine::{helpers::draw_progress_bar, engine::Engine};
/// # let mut engine = Engine::new(80, 24);
/// // Draw 60% filled health bar at (5, 2) with width 10
/// draw_progress_bar(&mut engine, 5, 2, 10, 0.6);
/// ```
pub fn draw_progress_bar(engine: &mut Engine, x: usize, y: usize, width: usize, percent: f32) {
    let filled = (width as f32 * percent).round() as usize;
    for i in 0..width {
        let c = if i < filled { '#' } else { '-' };
        engine.add_object(GameObject::new(x + i, y, c));
    }
}