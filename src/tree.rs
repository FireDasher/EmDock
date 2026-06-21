use egui::{Align, Color32, CursorIcon, FontId, Id, LayerId, Layout, Order, Pos2, Rect, Sense, Ui, UiBuilder, emath::fast_midpoint, vec2};

use crate::node::{Node, Pane};

// Binary tree representing relationships of Nodes
// The root is always 0
// For a given node with index "n":
// - left /top    child of n is n * 2 + 1
// - right/bottom child of n is n * 2 + 2
// - parent       node  of n is (n - 1) / 2 (the result is floored because of integer division)
pub struct Tree<State>{
	tree: Vec<Node<State>>
}

impl<State> Default for Tree<State> {
	fn default() -> Self {
		Self::new()
	}
}

#[inline(always)]
fn centered_position(target_rect: Rect, the_rect: Rect) -> Pos2 {
	Pos2 { x: fast_midpoint(target_rect.min.x, target_rect.max.x) - fast_midpoint(the_rect.min.x, the_rect.max.x), y: fast_midpoint(target_rect.min.y, target_rect.max.y) - fast_midpoint(the_rect.min.y, the_rect.max.y) }
}

impl<State> Tree<State> {
	pub fn new() -> Self {
		Self{ tree: Vec::new() }
	}

	// functions for making the starting layout

	/// Creates a new horizontal split, returning a tuple of the indexes of the left and right panes which can be used in hsplit or vsplit or tab.
	/// Use 0 as the index for the root
	pub fn hsplit(&mut self, index: usize, ratio: f32) -> (usize, usize) {
		if index >= self.tree.len() {
			self.tree.resize_with(index + 1, Default::default);
		}
		self.tree[index] = Node::hsplit(ratio);
		(index*2 + 1, index*2 + 2)
	}

	/// Creates a new vertical split, returning a tuple of the indexes of the top and bottom panes which can be used in hsplit or vsplit or tab.
	/// Use 0 as the index for the root
	pub fn vsplit(&mut self, index: usize, ratio: f32) -> (usize, usize) {
		if index >= self.tree.len() {
			self.tree.resize_with(index + 1, Default::default);
		}
		self.tree[index] = Node::vsplit(ratio);
		(index*2 + 1, index*2 + 2)
	}

	/// Creates a leaf node if there is no leaf node here yet, or adds a tab to the existing leaf node if there is.
	/// Use 0 as tghe index for theroot
	pub fn tab(&mut self, index: usize, title: String, ui: fn(&mut State, &mut egui::Ui)) {
		if let Some(Node::Leaf { tabs, .. }) = self.tree.get_mut(index) {
			tabs.push(Pane{title, ui});
		} else {
			if index >= self.tree.len() {
				self.tree.resize_with(index + 1, Default::default);
			}
			self.tree[index] = Node::leaf(Pane{title, ui});
		}
	}

	/// Renders this tree, you give it the state that should be passed to the panes and the UI and it renders the panels for you.
	pub fn show(&mut self, state: &mut State, ui: &mut Ui) {
		for i in 0..self.tree.len() {
			match &mut self.tree[i] {
				Node::None => (),
				Node::Leaf { tabs, active, rect } => {
					*active = (*active).min(tabs.len().saturating_sub(1)); // make sure tab number is within bounds

					if i == 0 { *rect = ui.content_rect() } // fixes an error

					const TAB_BAR_HEIGHT: f32 = 32.0;
					const TAB_ROUNDNESS: u8 = 4;

					let tab_y = rect.min.y + TAB_BAR_HEIGHT;

					// Tab bar
					{
						let rect = rect.intersect(Rect::everything_above(tab_y));
						let mut ui = ui.new_child(UiBuilder::new().max_rect(rect).layout(Layout::left_to_right(Align::Min)));

						ui.painter().rect_filled(rect, 0, ui.visuals().faint_bg_color);

						ui.spacing_mut().item_spacing = egui::vec2(0.0, 0.0);

						for (i, tab) in tabs.iter().enumerate() {
							// text
							const FONT: FontId = FontId::proportional(14.0);
							let galley = ui.painter().layout_no_wrap(tab.title.clone(), FONT, ui.visuals().widgets.active.fg_stroke.color);
							// the thing
							let (tab, response) = ui.allocate_at_least(vec2(galley.size().x + 16.0, TAB_BAR_HEIGHT), Sense::drag());
							let visual_tab = tab.shrink2(vec2(2.0, 4.0));

							// selection
							if response.drag_started() {
								*active = i;
							}

							// manually differntiate clicks and drags so starting a drag also selects the tab because it feels more responsive that way
							if response.dragged() && let Some(total_drag_delta) = response.total_drag_delta() && total_drag_delta.x*total_drag_delta.x + total_drag_delta.y*total_drag_delta.y > 36.0 && let Some(pos) = ui.pointer_interact_pos() {
								// Tooltip
								ui.set_cursor_icon(CursorIcon::Grabbing);
								ui.set_clip_rect(Rect::EVERYTHING);

								let hover_tab = visual_tab.translate(pos - visual_tab.center());

								let paitner = ui.painter().clone().with_layer_id(LayerId::new(Order::Tooltip, Id::new("emdock:dragging_tab")));
								paitner.rect_filled(hover_tab, TAB_ROUNDNESS, ui.visuals().widgets.active.bg_fill);
								paitner.galley(centered_position(hover_tab, galley.rect), galley, Color32::TRANSPARENT); // fallback colour is useless
							} else {
								if response.hovered() {
									ui.set_cursor_icon(CursorIcon::PointingHand); // the response.on_hover_cursor function returns itself for no reason which sucks and it is inlined anyways so this is way better and does the same thing
								}

								// draw the actual tab visual
								ui.set_clip_rect(rect);

								if *active == i {
									ui.painter().rect_filled(visual_tab, TAB_ROUNDNESS, ui.visuals().widgets.active.bg_fill);
								} else if response.hovered() {
									ui.painter().rect_filled(visual_tab, TAB_ROUNDNESS, ui.visuals().widgets.noninteractive.weak_bg_fill);
								}
								ui.painter().galley(centered_position(visual_tab, galley.rect), galley, Color32::TRANSPARENT); // fallback colour is useless
							}
						}
					}

					if let Some(pane) = tabs.get(*active) {
						let rect = rect.intersect(Rect::everything_below(tab_y));
						ui.painter().rect_filled(rect, 0, ui.visuals().window_fill);
						let mut ui = ui.new_child(UiBuilder::new().max_rect(rect.shrink(10.0))); // add a bit of padding
						ui.set_clip_rect(rect);
						(pane.ui)(state, &mut ui);
					}
				}
				Node::HSplit { ratio, rect } => {
					if i == 0 { *rect = ui.content_rect() } // fixes an error

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

					self.tree[i*2 + 1].set_rect(left_rect);
					self.tree[i*2 + 2].set_rect(right_rect);
				}
				Node::VSplit { ratio, rect } => {
					if i == 0 { *rect = ui.content_rect() } // fixes an error

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

					self.tree[i*2 + 1].set_rect(top_rect);
					self.tree[i*2 + 2].set_rect(bottom_rect);
				}
			}
		}
	}
}