use egui::{Id, IdMap, Ui, ahash::HashMapExt};

pub struct TileBuilder (pub(crate) IdMap<(String, Box<dyn FnOnce(&mut Ui)>)>);

impl TileBuilder {
	pub(crate) fn new() -> Self {
		Self(IdMap::new())
	}
	pub fn add(&mut self, title: impl Into<String>, content: impl FnOnce(&mut Ui) + 'static) {
		let title = title.into();
		self.0.insert(Id::new(&title), (title, Box::new(content)));
	}
}