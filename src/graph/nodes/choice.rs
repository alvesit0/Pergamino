use egui::{Button, Color32, TextEdit};
use egui_snarl::{ui::PinInfo};
use serde::{Serialize, Deserialize};

use crate::graph::{node::PergaminoNode, node_behavior::{NodeAction, PergaminoNodeBehavior}, types::DataType};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct ChoiceNode {
	pub choices: Vec<String>,
}

impl Default for ChoiceNode {
	fn default() -> Self {
		Self { 
			choices: vec!["Yes".to_string(), "No".to_string()],
		}
	}
}

impl PergaminoNodeBehavior for ChoiceNode {
	fn title(&self) -> String {
		"Choice".to_owned()
	}

	fn inputs(&self) -> usize {
		1
	}

	fn show_input(&mut self, 
		_pin: &egui_snarl::InPin, 
		_ui: &mut egui::Ui,
	) -> egui_snarl::ui::PinInfo {
		PinInfo::circle().with_fill(DataType::RegularStatement.color())
	}

	fn outputs(&self) -> usize {
		self.choices.len() + 1
	}

	fn show_output(
		&mut self, 
		pin: &egui_snarl::OutPin, 
		ui: &mut egui::Ui,
	) -> egui_snarl::ui::PinInfo {
		let idx = pin.id.output;
		let count = self.choices.len();
		
		const ROW_WIDTH: f32 = 200.0;
		const BUTTON_SIZE: f32 = 20.0;

		if idx < count {
			let mut removed = false;
			
			ui.allocate_ui_with_layout(
				egui::vec2(ROW_WIDTH, ui.spacing().interact_size.y),
				egui::Layout::left_to_right(egui::Align::Center),
				|ui| {
					if ui.button("🗑").on_hover_text("Remove this choice").clicked() {
						self.choices.remove(idx);
						removed = true;
					}

					if !removed {
						let text = &mut self.choices[idx];
						let text_width = ui.available_width() - BUTTON_SIZE - ui.spacing().item_spacing.x;
						
						ui.add(TextEdit::singleline(text).desired_width(text_width));
					}
				}
			);

			if removed {
				return PinInfo::square()
					.with_fill(Color32::TRANSPARENT)
					.with_stroke(egui::Stroke::NONE);
			}
			
			PinInfo::triangle().with_fill(DataType::RegularStatement.color())

		} else {
			ui.add_space(4.0);
			
			ui.add_sized(
				egui::vec2(ROW_WIDTH, ui.spacing().interact_size.y), 
				Button::new("+ Add Choice")
			).clicked().then(|| {
				self.choices.push("New Option".to_string());
			});

			PinInfo::square()
				.with_fill(Color32::TRANSPARENT)
				.with_stroke(egui::Stroke::NONE)
				.with_wire_color(Color32::TRANSPARENT)
		}
	}

	fn show_body(
		&mut self,
		_node_id: egui_snarl::NodeId,
		_inputs: &[egui_snarl::InPin],
		_outputs: &[egui_snarl::OutPin],
		ui: &mut egui::Ui,
		_nodes: &[PergaminoNode]
	) -> NodeAction {
		ui.add_space(8.0);
		NodeAction::None
	}

	fn accent_color(&self) -> Color32 {
		Color32::from_rgb(37, 150, 190)
	}

	fn input_type(&self, _index: usize) -> Option<DataType> {
		Some(DataType::RegularStatement)
	}

	fn output_type(&self, index: usize) -> Option<DataType> {
		if index < self.choices.len() {
			Some(DataType::RegularStatement)
		} else {
			None
		}
	}
}