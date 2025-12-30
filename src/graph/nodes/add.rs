use egui::Color32;
use egui_snarl::{NodeId, ui::PinInfo};
use serde::{Serialize, Deserialize};

use crate::graph::{node_behavior::{NodeAction, PergaminoNodeBehavior}, types::DataType};

// #[derive(Default)]
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct AddNode {
}

impl Default for AddNode {
	fn default() -> Self {
		Self {}
	}
}

impl PergaminoNodeBehavior for AddNode {
	fn title(&self) -> String {
		"Add".to_owned()
	}

	fn inputs(&self) -> usize {
		2
	}

	fn show_input(
			&mut self,
			_pin: &egui_snarl::InPin,
			ui: &mut egui::Ui,
			) -> egui_snarl::ui::PinInfo {
		
		ui.label("Add in");
		PinInfo::circle().with_fill(DataType::String.color())
	}

	fn outputs(&self) -> usize {
		1
	}

	fn show_output(
			&mut self,
			_pin: &egui_snarl::OutPin,
			ui: &mut egui::Ui,
		) -> egui_snarl::ui::PinInfo {
		
		ui.label("Out");
		PinInfo::circle().with_fill(Color32::RED)
	}

	// fn show_node_menu(
	// 		&mut self,
	// 		_node: egui_snarl::NodeId,
	// 		_inputs: &[egui_snarl::InPin],
	// 		_outputs: &[egui_snarl::OutPin],
	// 		_ui: &mut egui::Ui,
	// 	) -> NodeAction {
	// 	NodeAction::None
	// }
	
	fn show_body(
		&mut self,
		_node_id:egui_snarl::NodeId,
		_inputs: &[egui_snarl::InPin],
		_outputs: &[egui_snarl::OutPin],
		_ui: &mut egui::Ui,
		_candidates: &[(NodeId, String)]
	) -> NodeAction {
		NodeAction::None
	}

	fn accent_color(&self) -> Color32 {
		Color32::from_rgb(255, 200, 100)
	}

	fn input_type(&self, _index: usize) -> Option<DataType> {
		Some(DataType::String)
	}

	fn output_type(&self, _index: usize) -> Option<DataType> {
		Some(DataType::String)
	}
}