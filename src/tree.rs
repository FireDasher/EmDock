use egui::{Align, Color32, CornerRadius, CursorIcon, FontId, Layout, Rect, Sense, Ui, UiBuilder, pos2, vec2};

use crate::{builder::TileBuilder, node::Node};

// Binary tree representing relationships of Nodes
// The root is always 0
// For a given node with index "n":
// - left /top    child of n is n * 2 + 1
// - right/bottom child of n is n * 2 + 2
// - parent       node  of n is (n - 1) / 2 (the result is floored because of integer division)
pub struct Tree(pub Vec<Node>);

impl Default for Tree {
	fn default() -> Self {
		Self::new()
	}
}

impl Tree {
	pub fn new() -> Self {
		Self(Vec::new())
	}

	pub fn show(&mut self, ui: &mut Ui, add_tiles: impl FnOnce(&mut TileBuilder)) {
		let mut builder = TileBuilder::new();
		add_tiles(&mut builder); // collect tiles

		for i in 0..self.0.len() {
			match &mut self.0[i] {
				Node::None => (),
				Node::Leaf { tabs, active, rect } => {
					*active = (*active).min(tabs.len().saturating_sub(1)); // make sure tab number is within bounds

					if *rect == Rect::EVERYTHING { *rect = ui.content_rect() } // fixes an error

					ui.set_clip_rect(*rect);

					const TAB_BAR_HEIGHT: f32 = 24.0;
					const TAB_ROUNDNESS: u8 = 4;
					const ROUNDING: CornerRadius = CornerRadius{nw: TAB_ROUNDNESS, ne: TAB_ROUNDNESS, sw: 0, se: 0};

					let tab_y = rect.min.y + TAB_BAR_HEIGHT;

					// Tab bar
					{
						let rect = rect.intersect(Rect::everything_above(tab_y));
						let mut ui = ui.new_child(UiBuilder::new().max_rect(rect).layout(Layout::left_to_right(Align::Min)));

						ui.painter().rect_filled(rect, ROUNDING, ui.visuals().faint_bg_color);
						ui.painter().line_segment([pos2(rect.min.x, rect.max.y - 1.0), pos2(rect.max.x, rect.max.y - 1.0)], ui.visuals().widgets.active.bg_stroke); // bottom seperator

						ui.spacing_mut().item_spacing = egui::vec2(0.0, 0.0);

						for (i, tab) in tabs.iter().enumerate() {
							// text
							const FONT: FontId = FontId::proportional(14.0);
							let title = builder.0.get(tab).map(|(s,_)| s as &str).unwrap_or("?");
							let galley = ui.painter().layout_no_wrap(title.to_string(), FONT, ui.visuals().widgets.active.fg_stroke.color);
							// the thing
							let (tab, response) = ui.allocate_at_least(vec2(galley.size().x + 16.0, TAB_BAR_HEIGHT), Sense::click_and_drag());
							let response = response.on_hover_cursor(CursorIcon::PointingHand);
							// selection
							if response.clicked() {
								*active = i;
							}
							// the tab-looking part
							if *active == i {
								// let rounding = CornerRadius{ nw: if tab.min.x - 1.0 < rect.min.x + TAB_ROUNDNESS_F {0} else {TAB_ROUNDNESS}, ne: if tab.max.x + 1.0 > rect.max.x - TAB_ROUNDNESS_F {0} else {TAB_ROUNDNESS}, sw: 0, se: 0};
								ui.painter().rect_filled(Rect::from_min_max(pos2(tab.min.x - 1.0, tab.min.y), pos2(tab.max.x + 1.0, tab.max.y - 1.0)), ROUNDING, ui.visuals().widgets.active.bg_stroke.color);
								ui.painter().rect_filled(Rect::from_min_max(pos2(tab.min.x, tab.min.y + 1.0), pos2(tab.max.x, tab.max.y)), ROUNDING, ui.visuals().window_fill);
							}
							// label
							ui.painter().galley(tab.shrink2(egui::vec2(8.0, 5.0)).min, galley, Color32::TRANSPARENT); // fallback colour is useless
						}
					}

					if let Some(id) = tabs.get(*active) {
						if let Some(f) = builder.0.remove(id) {
							let rect = rect.intersect(Rect::everything_below(tab_y));
							ui.painter().rect_filled(rect, 0, ui.visuals().window_fill);
							let mut ui = ui.new_child(UiBuilder::new().max_rect(rect.shrink(10.0))); // add a bit of padding
							f.1(&mut ui);
						}
					}
				}
				Node::HSplit { ratio, rect } => {
					if *rect == Rect::EVERYTHING { *rect = ui.content_rect() } // fixes an error

					const SEPERATOR_WIDTH: f32 = 1.0;
					let split = rect.min.x + rect.width() ** ratio;
					let seperator = rect.with_min_x(split - SEPERATOR_WIDTH).with_max_x(split + SEPERATOR_WIDTH);
					let response = ui.allocate_rect(seperator, Sense::drag()).on_hover_cursor(CursorIcon::ResizeHorizontal);

					*ratio = (*ratio + response.drag_delta().x/rect.width()).clamp(0.1, 0.9);

					// highlight the seperator
					if response.dragged() {
						ui.painter().rect_filled(seperator, 0, ui.style().visuals.widgets.active.fg_stroke.color);
					} else if response.hovered() {
						ui.painter().rect_filled(seperator, 0, ui.style().visuals.widgets.hovered.bg_stroke.color);
					}

					let left_rect = rect.intersect(Rect::everything_left_of(split - SEPERATOR_WIDTH));
					let right_rect = rect.intersect(Rect::everything_right_of(split + SEPERATOR_WIDTH));

					self.0[i*2 + 1].set_rect(left_rect);
					self.0[i*2 + 2].set_rect(right_rect);
				}
				Node::VSplit { ratio, rect } => {
					if *rect == Rect::EVERYTHING { *rect = ui.content_rect() } // fixes an error

					const SEPERATOR_WIDTH: f32 = 1.0;
					let split = rect.min.y + rect.height() ** ratio;
					let seperator = rect.with_min_y(split - SEPERATOR_WIDTH).with_max_y(split + SEPERATOR_WIDTH);
					let response = ui.allocate_rect(seperator, Sense::drag()).on_hover_cursor(CursorIcon::ResizeVertical);

					*ratio = (*ratio + response.drag_delta().y/rect.height()).clamp(0.1, 0.9);

					// highlight the seperator
					if response.dragged() {
						ui.painter().rect_filled(seperator, 0, ui.style().visuals.widgets.active.fg_stroke.color);
					} else if response.hovered() {
						ui.painter().rect_filled(seperator, 0, ui.style().visuals.widgets.hovered.bg_stroke.color);
					}

					let top_rect = rect.intersect(Rect::everything_above(split - SEPERATOR_WIDTH));
					let bottom_rect = rect.intersect(Rect::everything_below(split + SEPERATOR_WIDTH));

					self.0[i*2 + 1].set_rect(top_rect);
					self.0[i*2 + 2].set_rect(bottom_rect);
				}
			}
		}
	}
}