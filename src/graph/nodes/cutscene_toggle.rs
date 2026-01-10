use egui::{Color32};
use egui_snarl::{ui::PinInfo};
use serde::{Serialize, Deserialize};

use crate::graph::{node::PergaminoNode, node_behavior::{GraphContext, NodeAction, NodeCategory, PergaminoNodeBehavior}, types::DataType};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct CutsceneToggleNode {
	pub value: bool
}

impl Default for CutsceneToggleNode {
	fn default() -> Self {
		Self {
			value: false
		}
	}
}

impl PergaminoNodeBehavior for CutsceneToggleNode {
	fn title(&self) -> String {
		"Cutscene Toggle".to_owned()
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
		let mut action = NodeAction::None;

		ui.vertical(|ui| {
			ui.set_min_width(80.0);
			ui.add_space(8.0);

			ui.centered_and_justified(|ui| {
				if ui.checkbox(&mut self.value, "").changed() {
					action = NodeAction::Update;
				}
			})
		});

		action
	}

	fn accent_color(&self) -> Color32 {
		Color32::from_rgb(232, 30, 158)
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
		1
	}

	fn category(&self) -> NodeCategory {
		NodeCategory::Routine
	}
}