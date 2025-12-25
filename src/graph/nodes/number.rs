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
		todo!()
	}

	fn inputs(&self) -> usize {
		todo!()
	}

	fn show_input(
			&mut self,
			pin: &egui_snarl::InPin,
			ui: &mut egui::Ui,
			) -> egui_snarl::ui::PinInfo {
		PinInfo::circle().with_fill(Color32::WHITE)
	}

	fn outputs(&self) -> usize {
		todo!()
	}

	fn show_output(
			&mut self,
			pin: &egui_snarl::OutPin,
			ui: &mut egui::Ui,
		) -> egui_snarl::ui::PinInfo {
		PinInfo::circle().with_fill(Color32::RED)
	}
	
	fn show_body(
		&mut self,
		node_id:egui_snarl::NodeId,
		inputs: &[egui_snarl::InPin],
		outputs: &[egui_snarl::OutPin],
		ui: &mut egui::Ui,
		candidates: &[(NodeId, String)]
	) -> NodeAction {
		todo!()
	}
}