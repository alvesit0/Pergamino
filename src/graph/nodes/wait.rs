use egui::{Color32};
use egui_snarl::{ui::PinInfo};
use serde::{Serialize, Deserialize};

use crate::graph::{node::PergaminoNode, node_behavior::{GraphContext, NodeAction, PergaminoNodeBehavior, UNLIMITED_CONNECTIONS}, types::DataType};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct WaitNode {
	pub amount: f32,
}

impl Default for WaitNode {
	fn default() -> Self {
		Self { 
			amount: 1.0
		}
	}
}

impl PergaminoNodeBehavior for WaitNode {
	fn title(&self) -> String {
		"Wait".to_owned()
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
		let mut action = NodeAction::None;

		ui.vertical(|ui| {
			ui.set_min_width(80.0);
			ui.add_space(8.0);
			ui.centered_and_justified(|ui| {
				let valid_range = 0..=999;

				let response = ui.add(
					egui::DragValue::new(&mut self.amount)
					.speed(0.1)
					.range(valid_range.clone())
				);

				if response.changed() {
					if self.amount < 0.0 {
						self.amount = 0.0;
					}
					action = NodeAction::Update;
				}
			})
		});

		action
	}

	fn accent_color(&self) -> Color32 {
		Color32::from_rgb(150, 145, 4)
	}

	fn input_type(&self, _index: usize) -> Option<DataType> {
		Some(DataType::RegularStatement)
	}

	fn output_type(&self, _index: usize) -> Option<DataType> {
		Some(DataType::RegularStatement)
	}

	fn output_max_connections(&self,_index:usize) -> usize {
		1

		// match index {
		// 	0 => UNLIMITED_CONNECTIONS,
		// 	_ => 1
		// }
	}

	fn input_max_connections(&self,_index:usize) -> usize {
		UNLIMITED_CONNECTIONS
	}
}