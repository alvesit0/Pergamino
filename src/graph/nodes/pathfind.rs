use egui::{Color32};
use egui_snarl::{ui::PinInfo};
use serde::{Serialize, Deserialize};

use crate::graph::{node::PergaminoNode, node_behavior::{GraphContext, NodeAction, PergaminoNodeBehavior}, types::DataType};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct PathfindNode {

}

impl Default for PathfindNode {
	fn default() -> Self {
		Self { 
			
		}
	}
}

impl PergaminoNodeBehavior for PathfindNode {
	fn title(&self) -> String {
		"Pathfind".to_owned()
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

		ui.vertical(|ui| {
			ui.set_min_width(80.0);
			ui.add_space(8.0);
			ui.centered_and_justified(|ui| {
				ui.label("tetete");
			})
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