use egui::{Color32};
use egui_snarl::{ui::PinInfo};
use serde::{Serialize, Deserialize};

use crate::{
	graph::{node::PergaminoNode, node_behavior::{GraphContext, NodeAction, NodeCategory, PergaminoNodeBehavior}, types::DataType},
	ui::widgets::{variable_text_edit::VariableTextEdit}
};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct ConditionalNode {
	pub expression: String,
}

impl Default for ConditionalNode {
	fn default() -> Self {
		Self { 
			expression: Default::default(),
		}
	}
}

impl PergaminoNodeBehavior for ConditionalNode {
	fn title(&self) -> String {
		"Conditional".to_owned()
	}

	fn inputs(&self) -> usize {
		1
	}

	fn show_input(&mut self, 
		_pin: &egui_snarl::InPin, 
		_ui: &mut egui::Ui,
		_ctx: &GraphContext
	) -> (PinInfo, NodeAction) {
		(
			PinInfo::circle().with_fill(DataType::RegularStatement.color()), 
			NodeAction::None
		)
	}

	fn outputs(&self) -> usize {
		2
	}

	fn show_output(
		&mut self,
		pin: &egui_snarl::OutPin,
		ui: &mut egui::Ui,
		_ctx: &GraphContext
	) -> (PinInfo, NodeAction) {
		let idx = pin.id.output;

		match idx {
			0 => {
				ui.add_sized(
					[20.0, 36.0],
					egui::Label::new("True")
				);
				(
					PinInfo::circle().with_fill(Color32::GREEN), 
					NodeAction::None
				)
			},
			_ => {
				ui.add_sized(
					[20.0, 10.0],
					egui::Label::new("False")
				);
				(
					PinInfo::circle().with_fill(Color32::RED), 
					NodeAction::None
				)
			}
		}
	}

	fn show_body(
		&mut self,
		_node_id: egui_snarl::NodeId,
		_inputs: &[egui_snarl::InPin],
		_outputs: &[egui_snarl::OutPin],
		ui: &mut egui::Ui,
		_nodes: &[PergaminoNode],
		ctx: &GraphContext
	) -> NodeAction {
		let mut action = NodeAction::None;

		ui.vertical(|ui| {
			ui.set_max_width(240.0);
			ui.add_space(8.0);

			ui.horizontal(|ui| {
				let response = VariableTextEdit::new(&mut self.expression, ctx.variables)
					.multiline(3)
					.desired_width(200.0)
					.show(ui);

				if response.changed() {
					action = NodeAction::Update;
				}
			});

			ui.add_space(4.0);
		});

		action
	}

	fn accent_color(&self) -> Color32 {
		Color32::from_rgb(196, 1, 50)
	}

	fn input_type(&self, _index: usize) -> Option<DataType> {
		Some(DataType::RegularStatement)
	}

	fn output_type(&self, _index: usize) -> Option<DataType> {
		Some(DataType::RegularStatement)
	}

	fn category(&self) -> NodeCategory {
		NodeCategory::Logic
	}
}