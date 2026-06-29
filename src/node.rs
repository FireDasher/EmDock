use egui::Rect;

pub struct Pane<State>{
	pub title: String,
	pub ui: fn(&mut State, &mut egui::Ui),
}
impl<State> std::fmt::Debug for Pane<State> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "<{}>", self.title)
	}
}

// This is the hierarcheal format that stores the tree state
pub enum Node<State> {
	None,
	Leaf {
		tabs: Vec<Pane<State>>, // the tabs
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
impl<State> Default for Node<State> {fn default() -> Self {Self::None}}

impl<State> std::fmt::Debug for Node<State> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Node::None => write!(f, "None"),
			Self::Leaf { tabs, active, .. } => write!(f, "Leaf({}){:?}", active, tabs),
			Self::HSplit { ratio, .. } => write!(f, "HSplit({})", ratio),
			Self::VSplit { ratio, .. } => write!(f, "VSplit({})", ratio),
		}
	}
}

impl<State> Node<State> {
	// Convenience functions
	pub fn leaf(first_tab: Pane<State>) -> Self {
		Self::Leaf { tabs: vec![first_tab], active: 0, rect: Rect::EVERYTHING }
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
	pub(crate) fn remove_tab(&mut self, itab: usize) -> Pane<State> {
		match self {
			Self::Leaf { tabs, active, .. } => {
				if itab <= *active { *active = (*active).saturating_sub(1) }
				tabs.remove(itab)
			},
			_ => unreachable!()
		}
	}
	pub(crate) fn push_tab(&mut self, tab: Pane<State>) {
		match self {
			Self::Leaf { tabs, active, .. } => {
				*active = tabs.len();
				tabs.push(tab);
			},
			_ => unreachable!()
		}
	}
	pub(crate) fn is_empty(&self) -> bool {
		match self {
			Self::None => true,
			Self::Leaf { tabs, .. } => tabs.is_empty(),
			_ => false,
		}
	}
}