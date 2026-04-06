use egui::{Id, IdMap, Ui, ahash::HashMapExt};

pub struct TileBuilder {
	pub(crate) contents: IdMap<(String, Box<dyn FnOnce(&mut Ui)>)> // The actual functions
}

impl TileBuilder {
	pub(crate) fn new() -> Self {
		Self {
			contents: IdMap::new(),
		}
	}
	pub fn add(&mut self, title: impl Into<String>, content: impl FnOnce(&mut Ui) + 'static) {
		let title = title.into();
		self.contents.insert(Id::new(&title), (title, Box::new(content)));
	}
}