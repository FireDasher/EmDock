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
		if ui.button("Baz").clicked() {
			tfd::MessageBox::new("BaZ", "Quux").run_modal_yes_no_cancel(tfd::YesNoCancel::Yes);
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