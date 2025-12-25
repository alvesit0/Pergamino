use egui_snarl::{NodeId, Snarl};
use enum_dispatch::enum_dispatch;
use crate::graph::node::PergaminoNode;

use crate::graph::nodes::{add::AddNode, number::NumberNode, complex::ComplexNode};

pub enum NodeAction {
	None,
	Disconnect(egui_snarl::OutPin, egui_snarl::InPin),
	Connect(egui_snarl::OutPin, egui_snarl::InPin),
	RemoveSelf
}

#[enum_dispatch(PergaminoNode)]
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
	fn has_body(&self) -> bool { false }

	fn show_node_menu(
			&mut self,
			node: egui_snarl::NodeId,
			_inputs: &[egui_snarl::InPin],
			_outputs: &[egui_snarl::OutPin],
			ui: &mut egui::Ui,
			snarl: &mut egui_snarl::Snarl<PergaminoNode>,
		) {
		if ui.button("Remove node").clicked() {
			snarl.remove_node(node);
			ui.close();
		}
	}

	fn show_body(
		&mut self,
		node_id: egui_snarl::NodeId,
		inputs: &[egui_snarl::InPin],
		outputs: &[egui_snarl::OutPin],
		ui: &mut egui::Ui,
		candidates: &[(NodeId, String)]
	) -> NodeAction;
}