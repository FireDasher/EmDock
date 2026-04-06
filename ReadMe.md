# `EmDock`: Docking system for [egui](https://github.com/emilk/egui) with readable syntax

[![github](https://img.shields.io/badge/github-FireDasher%2FEmDock-8da0cb?logo=github)](https://github.com/FireDasher/EmDock)
[![unsafe_forbidden](https://img.shields.io/badge/unsafe-forbidden-success)](https://github.com/rust-secure-code/safety-dance/)
[![egui_version](https://img.shields.io/badge/egui-0.34.1-blue)](https://github.com/emilk/egui)

## Try it
`cargo run --example hello`

## Example of the syntax
```
use eframe::egui;
use egui::Id;
use emdock::Tree;

fn main() {
	let native_options = eframe::NativeOptions::default();
	eframe::run_native("My egui App", native_options, Box::new(|cc| Ok(Box::new(MyEguiApp::new(cc))))).unwrap();
}

struct MyEguiApp {
	tiles: Tree,
	checked: bool,
}

impl MyEguiApp {
	fn new(cc: &eframe::CreationContext<'_>) -> Self {
		cc.egui_ctx.set_visuals(egui::Visuals::dark());
		Self{ tiles: Tree::new(), checked: false }
	}
}

impl eframe::App for MyEguiApp {
	fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
		self.tiles.show(ui, |tiles| {
			tiles.add("Hello", |ui| {
				ui.heading("World");
			});
			tiles.add("Second tab", |ui| {
				ui.heading("This is the second tab");
				if ui.button("Button").clicked() {
					println!("Button clicked!");
				}
			});
			tiles.add("Other tab", |ui| {
				ui.heading("this tab is another tab");
				ui.checkbox(&mut self.checked, "Checkbox");
			});
		});
	}
}
```