use egui::{Align, Color32, CornerRadius, CursorIcon, FontId, Layout, Rect, Sense, Ui, UiBuilder, pos2, vec2};

use crate::node::{Node, Pane};

// Binary tree representing relationships of Nodes
// The root is always 0
// For a given node with index "n":
// - left /top    child of n is n * 2 + 1
// - right/bottom child of n is n * 2 + 2
// - parent       node  of n is (n - 1) / 2 (the result is floored because of integer division)
pub struct Tree<State>(Vec<Node<State>>);

impl<State> Default for Tree<State> {
	fn default() -> Self {
		Self::new()
	}
}

impl<State> Tree<State> {
	pub fn new() -> Self {
		Self(Vec::new())
	}

	// functions for making the starting layout

	/// Creates a new horizontal split, returning a tuple of the indexes of the left and right panes which can be used in hsplit or vsplit or tab.
	/// Use 0 as the index for the root
	pub fn hsplit(&mut self, index: usize, ratio: f32) -> (usize, usize) {
		if index >= self.0.len() {
			self.0.resize_with(index + 1, Default::default);
		}
		self.0[index] = Node::hsplit(ratio);
		(index*2 + 1, index*2 + 2)
	}

	/// Creates a new vertical split, returning a tuple of the indexes of the top and bottom panes which can be used in hsplit or vsplit or tab.
	/// Use 0 as the index for the root
	pub fn vsplit(&mut self, index: usize, ratio: f32) -> (usize, usize) {
		if index >= self.0.len() {
			self.0.resize_with(index + 1, Default::default);
		}
		self.0[index] = Node::vsplit(ratio);
		(index*2 + 1, index*2 + 2)
	}

	/// Creates a leaf node if there is no leaf node here yet, or adds a tab to the existing leaf node if there is.
	/// Use 0 as tghe index for theroot
	pub fn tab(&mut self, index: usize, title: String, ui: fn(&mut State, &mut egui::Ui)) {
		if let Some(Node::Leaf { tabs, .. }) = self.0.get_mut(index) {
			tabs.push(Pane{title, ui});
		} else {
			if index >= self.0.len() {
				self.0.resize_with(index + 1, Default::default);
			}
			self.0[index] = Node::leaf(Pane{title, ui});
		}
	}

	// the massive funcion that makes it all work

	/// Renders this tree, you give it the state that should be passed to the panes and the UI and it renders the panels for you.
	pub fn show(&mut self, state: &mut State, ui: &mut Ui) {
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
							let galley = ui.painter().layout_no_wrap(tab.title.clone() /* why don't they just make this take a reference */, FONT, ui.visuals().widgets.active.fg_stroke.color);
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

					if let Some(pane) = tabs.get(*active) {
						let rect = rect.intersect(Rect::everything_below(tab_y));
						ui.painter().rect_filled(rect, 0, ui.visuals().window_fill);
						let mut ui = ui.new_child(UiBuilder::new().max_rect(rect.shrink(10.0))); // add a bit of padding
						(pane.ui)(state, &mut ui);
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