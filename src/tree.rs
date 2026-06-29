use std::mem;

use egui::{Align, Color32, CursorIcon, FontId, Id, LayerId, Layout, Order, Pos2, Rect, Sense, Stroke, Ui, UiBuilder, emath::fast_midpoint, vec2};

use crate::node::{Node, Pane};

// Binary tree representing relationships of Nodes
// The root is always 0
// For a given node with index "n":
// - left /top    child of n is n * 2 + 1
// - right/bottom child of n is n * 2 + 2
// - parent       node  of n is (n - 1) / 2
pub struct Tree<State>{
	tree: Vec<Node<State>>
}

impl<State> Default for Tree<State> {
	fn default() -> Self {
		Self::new()
	}
}

impl<State> std::fmt::Debug for Tree<State> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    	self.tree.fmt(f)
	}
}

#[derive(Debug)]
enum Split {
	None, Left, Right, Above, Below
}

struct DragData {
	itree: usize,
	itab: usize,
	pointer: Pos2,
	release: bool,
} struct HoverData {
	itree: usize,
	rect: Rect,
	tabbar_rect: Rect,
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

	// helpful helper
	fn resize_and_set(&mut self, index: usize, value: Node<State>) {
		if index >= self.tree.len() { self.tree.resize_with(index + 1, Default::default); }
		self.tree[index] = value;
	}

	/// Creates a new horizontal split, returning a tuple of the indexes of the left and right panes which can be used in hsplit or vsplit or tab.
	/// Use 0 as the index for the root
	pub fn hsplit(&mut self, index: usize, ratio: f32) -> (usize, usize) {
		self.resize_and_set(index, Node::hsplit(ratio));
		(index*2 + 1, index*2 + 2)
	}

	/// Creates a new vertical split, returning a tuple of the indexes of the top and bottom panes which can be used in hsplit or vsplit or tab.
	/// Use 0 as the index for the root
	pub fn vsplit(&mut self, index: usize, ratio: f32) -> (usize, usize) {
		self.resize_and_set(index, Node::vsplit(ratio));
		(index*2 + 1, index*2 + 2)
	}

	/// Creates a leaf node if there is no leaf node here yet, or adds a tab to the existing leaf node if there is.
	/// Use 0 as tghe index for theroot
	pub fn tab(&mut self, index: usize, title: String, ui: fn(&mut State, &mut egui::Ui)) {
		if let Some(Node::Leaf { tabs, .. }) = self.tree.get_mut(index) {
			tabs.push(Pane{title, ui});
		} else {
			self.resize_and_set(index, Node::leaf(Pane{title, ui}));
		}
	}


	// cleaning up (old and doesn't work)
	/*
	#[inline(always)]
	fn remove(&mut self, index: usize) {
		if index < self.tree.len() { self.tree[index] = Node::None }
	}

	fn move_subtree(&mut self, from: usize, to: usize) { // shifts An entired group of nodes
		let node = mem::take(&mut self.tree[from]);
		match node {
			Node::Leaf { .. } => self.resize_and_set(to, node),
			Node::HSplit { .. } | Node::VSplit { .. } => {
				self.resize_and_set(to, node);
				self.move_subtree(from*2 + 1, to*2 + 1);
				self.move_subtree(from*2 + 2, to*2 + 2);
			},
			Node::None => (),
		}
	}

	fn cleanup(&mut self) {
		for i in (0..self.tree.len()).rev() {
			if let Node::HSplit { .. } | Node::VSplit { .. } = &self.tree[i] {
				let left = i*2 + 1;
				let riht = i*2 + 2;

				match (self.tree.get(left).map_or(true, Node::is_empty), self.tree.get(riht).map_or(true, Node::is_empty)) {
					(true, true) => {
						self.tree[i] = Node::None;
						self.remove(left);
						self.remove(riht);
					},
					(true, false) => {
						self.tree[i] = Node::None;
						self.remove(left);
						self.move_subtree(riht, i);
					},
					(false, true) => {
						self.tree[i] = Node::None;
						self.remove(riht);
						self.move_subtree(left, i);
					},
					(false, false) => (),
				}
			}
		}
	}
	*/

	/// Renders this tree, you give it the state that should be passed to the panes and the UI and it renders the panels for you.
	pub fn show(&mut self, state: &mut State, ui: &mut Ui) {
		const TAB_BAR_HEIGHT: f32 = 32.0;
		const TAB_HORIZONTAL_PADDING: f32 = 16.0;
		const TAB_ROUNDNESS: u8 = 4;
		const SEPERATOR_HALF_WIDTH: f32 = 1.0;
		const FONT_SIZE: f32 = 14.0;
		const TAB_PADDING: f32 = 10.0;
		const HIGHLIGHT_OUTLINE_WIDTH: f32 = 2.0;
		const HIGHLIGHT_OPACITY: f32 = 0.5;

		let tree_space = ui.max_rect();

		let mut drag_data: Option<DragData> = None;  // Stores currently dragging tab
		let mut hover_data: Option<HoverData> = None;// Stores currently hovered panel
		for itree in 0..self.tree.len() {
			match &mut self.tree[itree] {
				Node::None => (),
				Node::Leaf { tabs, active, rect } => {
					if itree == 0 { *rect = tree_space } // fixes an error

					let tab_y = rect.min.y + TAB_BAR_HEIGHT;

					let tabbar_rect  = rect.intersect(Rect::everything_above(tab_y));
					let content_rect = rect.intersect(Rect::everything_below(tab_y));

					// Tab bar of tabs
					if !tabs.is_empty() {
						let mut ui = ui.new_child(UiBuilder::new().max_rect(tabbar_rect).layout(Layout::left_to_right(Align::Min)));

						ui.painter().rect_filled(tabbar_rect, 0, ui.visuals().faint_bg_color);

						ui.spacing_mut().item_spacing = egui::vec2(0.0, 0.0);

						for (itab, tab) in tabs.iter().enumerate() {
							// text
							const FONT: FontId = FontId::proportional(FONT_SIZE);
							let galley = ui.painter().layout_no_wrap(tab.title.clone(), FONT, ui.visuals().widgets.active.fg_stroke.color);
							// the thing
							let (tab, response) = ui.allocate_at_least(vec2(galley.size().x + TAB_HORIZONTAL_PADDING, TAB_BAR_HEIGHT), Sense::click_and_drag());
							let visual_tab = tab.shrink2(vec2(2.0, 4.0));

							// selection
							if response.is_pointer_button_down_on() {
								*active = itab;
							}

							// no longer manually differntiate clicks and drags so starting a drag also selects the tab because it feels more responsive that way, because using response.is_pointer_button_down_on() instead of response.clicked() does the exact same thing
							if response.dragged() && let Some(pointer) = ui.pointer_interact_pos() {
								// Tooltip
								ui.set_cursor_icon(CursorIcon::Grabbing);
								ui.set_clip_rect(Rect::EVERYTHING);

								let hover_tab = Rect::from_center_size(pointer, visual_tab.size());

								let paitner = ui.layer_painter(LayerId::new(Order::Tooltip, Id::new("emdock:dragging_tab")));
								paitner.rect_filled(hover_tab, TAB_ROUNDNESS, ui.visuals().widgets.active.bg_fill);
								paitner.galley(centered_position(hover_tab, galley.rect), galley, Color32::TRANSPARENT); // fallback colour is useless

								drag_data = Some(DragData { itree, itab, pointer, release: false });
							} else {
								if response.drag_stopped() && let Some(pointer) = ui.pointer_interact_pos() {
									drag_data = Some(DragData { itree, itab, pointer, release: true });
								}

								if response.hovered() {
									ui.set_cursor_icon(CursorIcon::PointingHand); // the response.on_hover_cursor function returns itself for no reason which sucks and it is inlined anyways so this is way better and does the same thing
								}

								// draw the actual tab visual
								ui.set_clip_rect(tabbar_rect);

								if *active == itab {
									ui.painter().rect_filled(visual_tab, TAB_ROUNDNESS, ui.visuals().widgets.active.bg_fill);
								} else if response.hovered() {
									ui.painter().rect_filled(visual_tab, TAB_ROUNDNESS, ui.visuals().widgets.noninteractive.weak_bg_fill);
								}
								ui.painter().galley(centered_position(visual_tab, galley.rect), galley, Color32::TRANSPARENT); // fallback colour is useless
							}
						}
					}

					// draw tab's contents
					if let Some(pane) = tabs.get(*active) {
						ui.painter().rect_filled(content_rect, 0, ui.visuals().window_fill);
						let mut ui = ui.new_child(UiBuilder::new().max_rect(content_rect.shrink(TAB_PADDING))); // add a bit of padding
						ui.set_clip_rect(content_rect);
						(pane.ui)(state, &mut ui);
					}

					// hover data
					if let Some(pointer) = ui.pointer_interact_pos() && rect.contains(pointer) {
						hover_data = Some(HoverData { itree, tabbar_rect, rect: *rect });
					}
				}
				Node::HSplit { ratio, rect } => {
					if itree == 0 { *rect = tree_space } // fixes an error

					let split = rect.min.x + rect.width() ** ratio;
					let seperator = rect.with_min_x(split - SEPERATOR_HALF_WIDTH).with_max_x(split + SEPERATOR_HALF_WIDTH);
					let response = ui.allocate_rect(seperator, Sense::drag()).on_hover_cursor(CursorIcon::ResizeHorizontal);

					*ratio = (*ratio + response.drag_delta().x/rect.width()).clamp(0.1, 0.9);

					// highlight the seperator
					if response.dragged() {
						ui.painter().rect_filled(seperator, 0, ui.style().visuals.widgets.active.fg_stroke.color);
					} else if response.hovered() {
						ui.painter().rect_filled(seperator, 0, ui.style().visuals.widgets.hovered.bg_stroke.color);
					}

					let left_rect = rect.intersect(Rect::everything_left_of(split - SEPERATOR_HALF_WIDTH));
					let right_rect = rect.intersect(Rect::everything_right_of(split + SEPERATOR_HALF_WIDTH));

					self.tree[itree*2 + 1].set_rect(left_rect);
					self.tree[itree*2 + 2].set_rect(right_rect);
				}
				Node::VSplit { ratio, rect } => {
					if itree == 0 { *rect = tree_space } // fixes an error

					let split = rect.min.y + rect.height() ** ratio;
					let seperator = rect.with_min_y(split - SEPERATOR_HALF_WIDTH).with_max_y(split + SEPERATOR_HALF_WIDTH);
					let response = ui.allocate_rect(seperator, Sense::drag()).on_hover_cursor(CursorIcon::ResizeVertical);

					*ratio = (*ratio + response.drag_delta().y/rect.height()).clamp(0.1, 0.9);

					// highlight the seperator
					if response.dragged() {
						ui.painter().rect_filled(seperator, 0, ui.style().visuals.widgets.active.fg_stroke.color);
					} else if response.hovered() {
						ui.painter().rect_filled(seperator, 0, ui.style().visuals.widgets.hovered.bg_stroke.color);
					}

					let top_rect = rect.intersect(Rect::everything_above(split - SEPERATOR_HALF_WIDTH));
					let bottom_rect = rect.intersect(Rect::everything_below(split + SEPERATOR_HALF_WIDTH));

					self.tree[itree*2 + 1].set_rect(top_rect);
					self.tree[itree*2 + 2].set_rect(bottom_rect);
				}
			}
		}
		// finally handle the docking
		if let (Some(DragData { itree: itree_tab, itab, pointer, release }), Some(HoverData { itree, tabbar_rect, rect })) = (drag_data, hover_data) {
			// calculate dock location
			let (split, split_visual_rect) = if tabbar_rect.contains(pointer) {
				(Split::None, tabbar_rect)
			} else {
				let center = rect.center();
				let n = (pointer - rect.min) / rect.size();
				match (n.x > n.y, n.x + n.y > 1.0) {
					(false,  false) => (Split::Left,  rect.intersect(Rect::everything_left_of (center.x))),
					(true,  true) => (Split::Right, rect.intersect(Rect::everything_right_of(center.x))),
					(true, false) => (Split::Above, rect.intersect(Rect::everything_above   (center.y))),
					(false, true) => (Split::Below, rect.intersect(Rect::everything_below   (center.y))),
				}
			};
			if release {
				// dock
				let tab = self.tree[itree_tab].remove_tab(itab);
				match split {
					Split::None => {
						self.tree[itree].push_tab(tab);
					},
					Split::Left => {
						let leaf = mem::replace(&mut self.tree[itree], Node::hsplit(0.5));
						let new_leaf = Node::leaf(tab);
						self.resize_and_set(itree*2 + 1, new_leaf);
						self.resize_and_set(itree*2 + 2, leaf);
					},
					Split::Right => {
						let leaf = mem::replace(&mut self.tree[itree], Node::hsplit(0.5));
						let new_leaf = Node::leaf(tab);
						self.resize_and_set(itree*2 + 1, leaf);
						self.resize_and_set(itree*2 + 2, new_leaf);
					},
					Split::Above => {
						let leaf = mem::replace(&mut self.tree[itree], Node::vsplit(0.5));
						let new_leaf = Node::leaf(tab);
						self.resize_and_set(itree*2 + 1, new_leaf);
						self.resize_and_set(itree*2 + 2, leaf);
					},
					Split::Below => {
						let leaf = mem::replace(&mut self.tree[itree], Node::vsplit(0.5));
						let new_leaf = Node::leaf(tab);
						self.resize_and_set(itree*2 + 1, leaf);
						self.resize_and_set(itree*2 + 2, new_leaf);
					},
				}
				// clean up empty leafs
			} else {
				ui.layer_painter(LayerId::new(Order::Foreground, Id::new("emdock:split_visual"))).rect(split_visual_rect, 0, ui.visuals().selection.bg_fill.linear_multiply(HIGHLIGHT_OPACITY), Stroke::new(HIGHLIGHT_OUTLINE_WIDTH, ui.visuals().selection.bg_fill), egui::StrokeKind::Inside);
			}
		}
	}
}