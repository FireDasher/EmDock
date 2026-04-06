use egui::{Align, Color32, CornerRadius, CursorIcon, FontId, Id, IdMap, Layout, Rect, Sense, Ui, UiBuilder, pos2, vec2};

use crate::builder::TileBuilder;

// This is the hierarcheal format that stores the tree state
pub enum Node {
	Leaf {
		tabs: Vec<Id>, // Id's of the tabs
		active: usize, // selected tab
	},
	HSplit {
		ratio: f32, // this determines the amount you scaled it
		left: Box<Node>, // recursion
		right: Box<Node>,
	},
	VSplit {
		ratio: f32,
		top: Box<Node>,
		bottom: Box<Node>,
	}
}

// Stores the nodes
pub struct Tree(pub Vec<Node>); // First element is the root, others are floating

impl Default for Tree {
	fn default() -> Self {
		Self(Vec::new())
	}
}

impl Tree {
	pub fn new() -> Self {
		Self(Vec::new())
	}

	pub fn show(&mut self, ui: &mut Ui, add_tiles: impl FnOnce(&mut TileBuilder)) {
		let mut builder = TileBuilder::new();
		add_tiles(&mut builder); // collect tiles

		Self::render_node(&mut self.0[0], ui, ui.content_rect(), &mut builder.contents);
	}

	fn render_node(node: &mut Node, ui: &mut Ui, rect: Rect, contents: &mut IdMap<(String, Box<dyn FnOnce(&mut Ui)>)>) {
		match node {
			Node::Leaf { tabs, active } => {
				*active = (*active).min(tabs.len().saturating_sub(1));

				const TAB_BAR_HEIGHT: f32 = 24.0;
				const TAB_ROUNDING: CornerRadius = CornerRadius{ ne: 4, nw: 4, se: 0, sw: 0 };
				const TAB_ROUNDING_LEFT: CornerRadius = CornerRadius{ ne: 4, nw: 0, se: 0, sw: 0 };
				ui.set_clip_rect(rect);

				let tab_y = rect.min.y + TAB_BAR_HEIGHT;

				// Tab bar
				{
					let rect = rect.intersect(Rect::everything_above(tab_y));
					let mut ui = ui.new_child(UiBuilder::new().max_rect(rect).layout(Layout::left_to_right(Align::Min)));

					ui.painter().rect_filled(rect, TAB_ROUNDING, ui.visuals().faint_bg_color);
					ui.painter().line_segment([pos2(rect.min.x, rect.max.y - 1.0), pos2(rect.max.x, rect.max.y - 1.0)], ui.visuals().widgets.active.bg_stroke); // bottom seperator

					ui.spacing_mut().item_spacing = egui::vec2(0.0, 0.0);

					for (i, tab) in tabs.iter().enumerate() {
						// text
						const FONT: FontId = FontId::proportional(14.0);
						let title = contents.get(tab).map(|(s,_)| s as &str).unwrap_or("?");
						let galley = ui.painter().layout_no_wrap(title.to_string(), FONT, ui.visuals().widgets.active.fg_stroke.color);
						// the thing
						let (rect, response) = ui.allocate_at_least(vec2(galley.size().x + 16.0, TAB_BAR_HEIGHT), Sense::click_and_drag());
						let response = response.on_hover_cursor(CursorIcon::PointingHand);
						// selection
						if response.clicked() {
							*active = i;
						}
						// the tab-looking part
						if *active == i {
							ui.painter().rect_filled(Rect::from_min_max(pos2(rect.min.x - 1.0, rect.min.y), pos2(rect.max.x + 1.0, rect.max.y - 1.0)), if i == 0 {TAB_ROUNDING_LEFT} else {TAB_ROUNDING}, ui.visuals().widgets.active.bg_stroke.color);
							ui.painter().rect_filled(Rect::from_min_max(pos2(rect.min.x, rect.min.y + 1.0), pos2(rect.max.x, rect.max.y)), if i == 0 {TAB_ROUNDING_LEFT} else {TAB_ROUNDING}, ui.visuals().window_fill);
						}
						// label
						ui.painter().galley(rect.shrink2(egui::vec2(8.0, 5.0)).min, galley, Color32::TRANSPARENT); // fallback colour is useless
					}
				}

				if let Some(id) = tabs.get(*active) {
					if let Some(f) = contents.remove(id) {
						let rect = rect.intersect(Rect::everything_below(tab_y));
						ui.painter().rect_filled(rect, 0, ui.visuals().window_fill);
						let mut ui = ui.new_child(UiBuilder::new().max_rect(rect.shrink(10.0))); // add a bit of padding
						f.1(&mut ui);
					}
				}
			}
			Node::HSplit { ratio, left, right } => {
				let split = rect.min.x + rect.width() ** ratio;
				Self::render_node(left, ui, rect.intersect(Rect::everything_left_of(split - 2.0)), contents);
				Self::render_node(right, ui, rect.intersect(Rect::everything_right_of(split + 2.0)), contents);
			}
			Node::VSplit { ratio, top, bottom } => {
				let split = rect.min.y + rect.height() ** ratio;
				Self::render_node(top, ui, rect.intersect(Rect::everything_above(split - 2.0)), contents);
				Self::render_node(bottom, ui, rect.intersect(Rect::everything_below(split + 2.0)), contents);
			}
		}
	}
}