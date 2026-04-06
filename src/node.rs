use egui::Id;

// This is the hierarcheal format that stores the tree state
pub enum Node {
	None,
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
impl Default for Node {fn default() -> Self {Self::None}}

// impl Node {
// 	fn split_left(&mut self, ratio: f32, other: Box<Node>) {
// 		*self = Node::HSplit { ratio, left: other, right: Box::new(std::mem::take(self)) };
// 	}
// 	fn split_right(&mut self, ratio: f32, other: Box<Node>) {
// 		*self = Node::HSplit { ratio, left: Box::new(std::mem::take(self)), right: other };
// 	}
// 	fn split_up(&mut self, ratio: f32, other: Box<Node>) {
// 		*self = Node::VSplit { ratio, top: other, bottom: Box::new(std::mem::take(self)) };
// 	}
// 	fn split_down(&mut self, ratio: f32, other: Box<Node>) {
// 		*self = Node::VSplit { ratio, top: Box::new(std::mem::take(self)), bottom: other };
// 	}
// }