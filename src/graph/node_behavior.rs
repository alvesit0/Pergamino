use egui::Color32;
use egui_snarl::{NodeId};
use enum_dispatch::enum_dispatch;
use crate::graph::node::PergaminoNode;

use crate::graph::nodes::{
	add::AddNode, 
	number::NumberNode, 
	complex::ComplexNode, 
	dialogue::DialogueNode, 
	start::StartNode
};
use crate::graph::types::DataType;

#[allow(dead_code)]
pub enum NodeAction {
	None,
	Disconnect(egui_snarl::OutPin, egui_snarl::InPin),
	Connect(egui_snarl::OutPin, egui_snarl::InPin),
	RemoveSelf
}

#[enum_dispatch]
pub trait PergaminoNodeBehavior {
	fn title(&self) -> String;

	fn inputs(&self) -> usize;
	fn show_input(
		&mut self,
		pin: &egui_snarl::InPin,
		ui: &mut egui::Ui,
		) -> egui_snarl::ui::PinInfo;

	fn outputs(&self) -> usize;
	fn show_output(
		&mut self,
		pin: &egui_snarl::OutPin,
		ui: &mut egui::Ui,
	) -> egui_snarl::ui::PinInfo;

	fn has_node_menu(&self) -> bool { true }
	fn has_body(&self) -> bool { true }

	fn show_node_menu(
			&mut self,
			_node: egui_snarl::NodeId,
			_inputs: &[egui_snarl::InPin],
			_outputs: &[egui_snarl::OutPin],
			ui: &mut egui::Ui,
		) -> NodeAction {
		if ui.button("Remove node").clicked() {
			ui.close();
			return NodeAction::RemoveSelf
		}

		NodeAction::None
	}

	fn show_body(
		&mut self,
		node_id: egui_snarl::NodeId,
		inputs: &[egui_snarl::InPin],
		outputs: &[egui_snarl::OutPin],
		ui: &mut egui::Ui,
		candidates: &[(NodeId, String)]
	) -> NodeAction;

	fn accent_color(&self) -> Color32 {
		Color32::from_gray(100)
	}

	fn input_type(&self, _index: usize) -> Option<DataType> {
		Some(DataType::Any)
	}

	fn output_type(&self, _index: usize) -> Option<DataType> {
		Some(DataType::Any)
	}
}