use serde::{Deserialize, Serialize};

// Structure to hold display content configuration
#[derive(Clone, Serialize, Deserialize)]
pub struct DisplayContent {
    pub text: String,
    pub scroll: bool,
    pub color: (u8, u8, u8),       // Default text color
    pub speed: f32,                // Pixels per second
    pub duration: u64,             // Display duration in seconds (0 = indefinite)
    pub repeat_count: u32,         // Number of times to repeat (0 = indefinite)
    pub border_effect: Option<BorderEffect>, // Optional border effect
    pub colored_segments: Option<Vec<ColoredSegment>>, // New field for multi-colored text
}

// New enum for available border effects
#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub enum BorderEffect {
    None,
    Rainbow,
    Pulse,
    Sparkle,
    Gradient { colors: Vec<(u8, u8, u8)> },
}

// Provide defaults
impl Default for BorderEffect {
    fn default() -> Self {
        BorderEffect::None
    }
}

// New struct to represent a colored segment within the text
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ColoredSegment {
    pub start: usize,  // Start index in the text
    pub end: usize,    // End index in the text (exclusive)
    pub color: (u8, u8, u8), // RGB color for this segment
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Playlist {
    pub items: Vec<DisplayContent>,
    pub active_index: usize,
    pub repeat: bool,
    pub brightness: u8,  // Global brightness setting
}

impl Default for Playlist {
    fn default() -> Self {
        Self {
            items: vec![],  // Start with an empty playlist
            active_index: 0,
            repeat: true,
            brightness: 100,  // Default brightness
        }
    }
} 