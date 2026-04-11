use eframe::egui;
use emdock::{node::Node, tree::Tree};

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
		Self{ tiles: Tree(vec![
			Node::hsplit(0.5),
				Node::vsplit(0.6),
				Node::leaf(&["KOLJDFSKLIHFGKSGDHKJFGFSGFG"]),
					Node::leaf(&["Page"]),
					Node::leaf(&["Hello", "Foo"]),
		]) }
	}
}

impl eframe::App for MyEguiApp {
	fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {self.tiles.show(ui, |tiles| {
		tiles.add("Hello", |ui| {
			ui.heading("World");
		});
		tiles.add("Foo", |ui| {
			ui.heading("B A R");
			ui.button("Baz").clicked();
		});
		tiles.add("Page", |ui| {
			ui.heading("This is very a page");
			ui.button("Button").clicked();
			ui.button("Button").clicked();
			ui.button("Button").clicked();
			ui.button("Button").clicked();
		});
		tiles.add("KOLJDFSKLIHFGKSGDHKJFGFSGFG", |ui| {
			ui.heading("FJISGHUFYtgruifghuisdhfgui");
			ui.heading("$Y*#%^&*^%&*^$#&*%^&*#$%^*&#$^*%");
			ui.heading("SFihuisfguyrgiyuhdflHJKGFYhryugf");
			ui.heading("úíüúéígé");
			ui.heading("Gyatt");
		});
	});}
}