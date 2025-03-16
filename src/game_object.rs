#[derive(Debug, Clone)]
pub struct GameObject {
    pub x: usize,
    pub y: usize,
    pub character: char,
    pub tag: String,
    pub frames: Vec<char>,
    pub current_frame: usize,
    pub frame_duration: f32,
    pub animation_timer: f32,
    pub fg_color: Option<String>,
    pub bg_color: Option<String>,
}

impl GameObject {
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