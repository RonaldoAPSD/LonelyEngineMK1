use crate::{engine::Engine, game_object::GameObject};

/// Checks for collision between two GameObjects.
/// Optionally ignores objects with specific tags.
pub fn check_collision(a: &GameObject, b: &GameObject, ignore_tags: &[&str]) -> bool {
    // Skip collision if either object has an ignored tag
    if ignore_tags.contains(&a.tag.as_str()) || ignore_tags.contains(&b.tag.as_str()) {
        return false;
    }

    // Simple AABB collision
    a.x == b.x && a.y == b.y
}

pub fn draw_text (engine: &mut Engine, x: usize, y: usize, text: &str) {
    for (i, c) in text.chars().enumerate() {
        engine.add_object(GameObject::new(x + i, y, c));
    }
}

pub fn draw_progress_bar(engine: &mut Engine, x: usize, y: usize, width: usize, percent: f32) {
    let filled = (width as f32 * percent).round() as usize;
    for i in 0..width {
        let c = if i < filled { '#' } else { '-' };
        engine.add_object(GameObject::new(x + i, y, c));
    }
}