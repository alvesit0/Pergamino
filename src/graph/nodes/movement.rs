use egui::{Color32};
use egui_snarl::{ui::PinInfo};
use serde::{Serialize, Deserialize};

use crate::graph::{node::PergaminoNode, node_behavior::{GraphContext, NodeAction, PergaminoNodeBehavior}, types::DataType};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct MovementNode {
	pub reference_id: String,
}

impl Default for MovementNode {
	fn default() -> Self {
		Self { 
			reference_id: Default::default()
		}
	}
}

impl PergaminoNodeBehavior for MovementNode {
	fn title(&self) -> String {
		"Movement".to_owned()
	}

	fn on_create(&mut self, _nodes: &[PergaminoNode]) {

	}

	fn inputs(&self) -> usize {
		1
	}

	fn show_input(&mut self, 
		_pin: &egui_snarl::InPin, 
		_ui: &mut egui::Ui,
		_context: &GraphContext
	) -> (PinInfo, NodeAction) {
		(
			PinInfo::circle().with_fill(DataType::RegularStatement.color()), 
			NodeAction::None
		)
	}

	fn outputs(&self) -> usize {
		1
	}

	fn show_output(
		&mut self, 
		_pin: &egui_snarl::OutPin, 
		_ui: &mut egui::Ui,
		_context: &GraphContext
	) -> (PinInfo, NodeAction) {
		(
			PinInfo::circle().with_fill(DataType::RegularStatement.color()), 
			NodeAction::None
		)
	}

	fn show_body(
		&mut self,
		_node_id:egui_snarl::NodeId,
		_inputs: &[egui_snarl::InPin],
		_outputs: &[egui_snarl::OutPin],
		ui: &mut egui::Ui,
		_nodes: &[PergaminoNode],
		_context: &GraphContext
	) -> NodeAction {
		let mut _action = NodeAction::None;

		let btn_size = egui::vec2(30.0, 30.0);

		ui.vertical(|ui| {
			ui.add_space(8.0);
			ui.set_max_width(160.0);
			ui.horizontal(|ui| {
				ui.vertical_centered(|ui| {
					ui.add(egui::TextEdit::singleline(&mut self.reference_id));
				});

				ui.add_space(8.0);

				egui::Grid::new("movement_grid")
					.spacing(egui::vec2(2.0, 2.0))
					.show(ui, |ui| {

					if ui.add_sized(btn_size, egui::Button::new("↖")).clicked() {}
					if ui.add_sized(btn_size, egui::Button::new("⬆")).clicked() {}
					if ui.add_sized(btn_size, egui::Button::new("↗")).clicked() {}

					ui.end_row();

					if ui.add_sized(btn_size, egui::Button::new("⬅")).clicked() {}
					ui.allocate_space(btn_size);
					if ui.add_sized(btn_size, egui::Button::new("➡")).clicked() {}

					ui.end_row();

					if ui.add_sized(btn_size, egui::Button::new("↙")).clicked() {}
					if ui.add_sized(btn_size, egui::Button::new("⬇")).clicked() {}
					if ui.add_sized(btn_size, egui::Button::new("↘")).clicked() {}
				})
			});
		});

		_action
	}

	fn accent_color(&self) -> Color32 {
		Color32::from_rgb(196, 120, 39)
	}

	fn input_type(&self, _index: usize) -> Option<DataType> {
		Some(DataType::RegularStatement)
	}

	fn output_type(&self, _index: usize) -> Option<DataType> {
		Some(DataType::RegularStatement)
	}
}