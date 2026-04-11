use egui::{Id, Rect};

// This is the hierarcheal format that stores the tree state
pub enum Node {
	None,
	Leaf {
		tabs: Vec<Id>, // Id's of the tabs
		active: usize, // selected tab
		rect: Rect, // storing this is nessasary for the flat array format
	},
	HSplit {
		ratio: f32, // this determines the amount you scaled it
		rect: Rect,
	},
	VSplit {
		ratio: f32,
		rect: Rect,
	}
}
impl Default for Node {fn default() -> Self {Self::None}}

impl Node {
	// Convenience functions
	pub fn leaf(tabs: &[&str]) -> Self {
		Self::Leaf { tabs: tabs.into_iter().map(|e| Id::new(e)).collect(), active: 0, rect: Rect::EVERYTHING }
	}
	pub fn hsplit(ratio: f32) -> Self {
		Self::HSplit { ratio, rect: Rect::EVERYTHING }
	}
	pub fn vsplit(ratio: f32) -> Self {
		Self::VSplit { ratio, rect: Rect::EVERYTHING }
	}

	// Used in the rendering process because of Rust's staticness
	pub(crate) fn set_rect(&mut self, new_rect: Rect) {
		match self {
			Self::None => (),
			Self::Leaf { rect, .. } | Self::HSplit { rect, .. } | Self::VSplit { rect, .. } => *rect = new_rect,
		}
	}
}