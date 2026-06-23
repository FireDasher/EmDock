use eframe::egui;
use emdock::Tree;

fn main() {
	let native_options = eframe::NativeOptions::default();

	let mut tree: Tree<State> = Tree::new();

	// layout
	{
		let (left, right) = tree.hsplit(0, 0.5);
			let (top, bottom) = tree.vsplit(left, 0.6);
				tree.tab(top, "Page".into(), State::page);
				tree.tab(bottom, "Hello".into(), State::hello);
				tree.tab(bottom, "Foo".into(), State::foo);
			tree.tab(right, "KOLJDFSKLIHFGKSGDHKJFGFSGFG".into(), State::keysmash);
	}

	// state
	let mut state = State::default();

	eframe::run_ui_native("My egui App", native_options, move |ui, _frame| {
		tree.show(&mut state, ui);
	}).unwrap();
}

struct State {
	amount: i64, // allow massive values
	counter: i64, // allow massive values
}

impl Default for State {
	fn default() -> Self {
    	Self {amount: 2, counter: 1}
	}
}

impl State {
	fn hello(&mut self, ui: &mut egui::Ui) {
		ui.heading("World");
	}
	fn foo(&mut self, ui: &mut egui::Ui) {
		ui.heading("B A R");
		if ui.button("Baz").is_pointer_button_down_on() {
			ui.heading("Lorem ipsum dolor sit amet, consectetur adipiscing elit. Cras leo mi, auctor vitae dui at, rutrum egestas purus. Proin nulla dui, auctor eget leo vel, laoreet pellentesque sapien. Donec feugiat eros dolor, non volutpat odio mattis pulvinar. Mauris blandit sem vitae neque tempor pretium. Donec volutpat nulla vitae augue imperdiet, eleifend facilisis velit dignissim. Vivamus semper pharetra luctus. Aenean augue nunc, convallis at commodo eu, auctor sit amet lorem. Interdum et malesuada fames ac ante ipsum primis in faucibus. Lorem ipsum dolor sit amet, consectetur adipiscing elit. Ut sit amet quam molestie, placerat orci ut, commodo lacus. Nam ut accumsan orci. Suspendisse congue mollis eros at aliquet. Mauris arcu erat, finibus sit amet sagittis nec, vehicula vel tortor.");
		};
	}
	fn page(&mut self, ui: &mut egui::Ui) {
		ui.heading("This is very a page");

		ui.label("Amount: ");
		ui.add(egui::DragValue::new(&mut self.amount));

		if ui.button("*").clicked() { self.counter *= self.amount };
		if ui.button("/").clicked() { self.counter /= self.amount };
		if ui.button("+").clicked() { self.counter += self.amount };
		if ui.button("-").clicked() { self.counter -= self.amount };
	}
	fn keysmash(&mut self, ui: &mut egui::Ui) {
		ui.heading("FJISGHUFYtgruifghuisdhfgui");
		ui.heading("$Y*#%^&*^%&*^$#&*%^&*#$%^*&#$^*%");
		ui.heading("SFihuisfguyrgiyuhdflHJKGFYhryugf");
		ui.heading("úíüúéígéüíóéüáíóáéüíóáíóáéüíóáéüíó");
		ui.heading("97867578678568726578623879568237465");

		ui.label(egui::RichText::new("Counter").size(100.0).strong());
		ui.label(self.counter.to_string());
	}
}