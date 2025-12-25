use egui::Color32;
use egui_snarl::{NodeId, ui::PinInfo};
use serde::{Serialize, Deserialize};

use crate::graph::node_behavior::{NodeAction, PergaminoNodeBehavior};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct NumberNode {
	pub value: f64,
}

impl PergaminoNodeBehavior for NumberNode {
	fn title(&self) -> String {
		"Number".to_owned()
	}

	fn inputs(&self) -> usize {
		1
	}

	fn show_input(
			&mut self,
			pin: &egui_snarl::InPin,
			ui: &mut egui::Ui,
			) -> egui_snarl::ui::PinInfo {
		
		ui.label("Number input");

		if !pin.remotes.is_empty() {
			PinInfo::triangle().with_fill(Color32::BLUE)
		} else {
			PinInfo::circle().with_fill(Color32::BLUE)
		}
	}

	fn outputs(&self) -> usize {
		1
	}

	fn show_output(
			&mut self,
			_pin: &egui_snarl::OutPin,
			ui: &mut egui::Ui,
		) -> egui_snarl::ui::PinInfo {
		
		ui.add(egui::DragValue::new(&mut self.value).speed(0.1));
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
}