use egui::{Color32};
use egui_snarl::{ui::PinInfo};
use serde::{Serialize, Deserialize};

use crate::graph::{node::{PergaminoNode}, node_behavior::{NodeAction, PergaminoNodeBehavior}, types::DataType};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct PassiveRoutineNode {
}

impl Default for PassiveRoutineNode {
	fn default() -> Self {
		Self { 
		}
	}
}

impl PergaminoNodeBehavior for PassiveRoutineNode {
	fn title(&self) -> String {
		"Passive Routine".to_owned()
	}

	fn on_create(&mut self, _nodes: &[PergaminoNode]) {
	}

	fn inputs(&self) -> usize {
		0
	}

	fn show_input(&mut self, 
		_pin: &egui_snarl::InPin, 
		_ui: &mut egui::Ui
	) -> egui_snarl::ui::PinInfo {
		PinInfo::circle().with_fill(DataType::RegularStatement.color())
	}

	fn outputs(&self) -> usize {
		1
	}

	fn show_output(
		&mut self, 
		_pin: &egui_snarl::OutPin, 
		_ui: &mut egui::Ui,
	) -> egui_snarl::ui::PinInfo {
		PinInfo::circle().with_fill(DataType::RegularStatement.color())
	}

	fn show_body(
		&mut self,
		_node_id:egui_snarl::NodeId,
		_inputs: &[egui_snarl::InPin],
		_outputs: &[egui_snarl::OutPin],
		ui: &mut egui::Ui,
		_nodes: &[PergaminoNode]
	) -> NodeAction {
		let mut _action = NodeAction::None;

		ui.vertical(|ui| {
			ui.set_min_width(80.0);
		});

		_action
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
		0
	}
}