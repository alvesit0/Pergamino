use egui::{Color32};
use egui_snarl::{NodeId, ui::PinInfo};
use serde::{Serialize, Deserialize};

use crate::graph::{node_behavior::{NodeAction, PergaminoNodeBehavior}, types::DataType};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct StartNode {
	index: i16,
}

impl Default for StartNode {
	fn default() -> Self {
		Self { 
			index: Default::default() 
		}
	}
}

impl PergaminoNodeBehavior for StartNode {
	fn title(&self) -> String {
		"Start".to_owned()
	}

	fn inputs(&self) -> usize {
		0
	}

	fn show_input(&mut self, 
		_pin: &egui_snarl::InPin, 
		_ui: &mut egui::Ui
	) -> egui_snarl::ui::PinInfo {
		PinInfo::circle().with_fill(DataType::Any.color())
	}

	fn outputs(&self) -> usize {
		1
	}

	fn show_output(
		&mut self, 
		_pin: &egui_snarl::OutPin, 
		_ui: &mut egui::Ui,
	) -> egui_snarl::ui::PinInfo {
		PinInfo::circle().with_fill(DataType::Any.color())
	}

	fn show_body(
		&mut self,
		_node_id:egui_snarl::NodeId,
		_inputs: &[egui_snarl::InPin],
		_outputs: &[egui_snarl::OutPin],
		ui: &mut egui::Ui,
		_candidates: &[(NodeId,String)]
	) -> NodeAction {
		let mut _action = NodeAction::None;

		ui.vertical(|ui| {
			ui.set_min_width(60.0);

			ui.add_space(8.0);

			ui.centered_and_justified(|ui| {
				ui.label(self.index.to_string());
			})
		});

		_action
	}

	fn accent_color(&self) -> Color32 {
		Color32::from_rgb(233, 30, 99)
	}

	fn input_type(&self, _index: usize) -> Option<DataType> {
		Some(DataType::Any)
	}

	fn output_type(&self, _index: usize) -> Option<DataType> {
		Some(DataType::Any)
	}
}