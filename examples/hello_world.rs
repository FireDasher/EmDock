use eframe::egui;
use egui::Id;
use egui_imdock::tree::{Node, Tree};

fn main() {
	let native_options = eframe::NativeOptions::default();
	eframe::run_native("My egui App", native_options, Box::new(|cc| Ok(Box::new(MyEguiApp::new(cc))))).unwrap();
}

struct MyEguiApp {
	tiles: Tree,
}

impl MyEguiApp {
	fn new(cc: &eframe::CreationContext<'_>) -> Self {
		cc.egui_ctx.set_visuals(egui::Visuals::dark());
		// Directly make the layout just for prototyping
		Self{ tiles: Tree(vec![Node::HSplit { ratio: 0.3, left: Box::new(Node::Leaf { tabs: vec![Id::new("Hello"), Id::new("Ohio")], active: 0 }), right: Box::new(Node::Leaf { tabs: vec![Id::new("Rizz")], active: 0 }) }]) }
	}
}

impl eframe::App for MyEguiApp {
	fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {self.tiles.show(ui, |tiles| {
		tiles.add("Hello", |ui| {
			ui.heading("World");
		});
		tiles.add("Ohio", |ui| {
			ui.heading("Skibidi ohio rizz gyatt!!!!!!!");
			ui.button("Skizz").clicked();
		});
		tiles.add("Rizz", |ui| {
			ui.heading("Gyatt");
			ui.button("DJOLFGHidjksygh").clicked();
			ui.button("dfg").clicked();
			ui.button("fg").clicked();
			ui.button("dfgsgsdfgsds").clicked();
		});
	});}
}