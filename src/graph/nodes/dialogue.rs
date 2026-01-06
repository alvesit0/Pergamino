use egui::{Color32, TextEdit};
use egui_snarl::{ui::PinInfo};
use serde::{Serialize, Deserialize};

use crate::graph::{node::PergaminoNode, node_behavior::{NodeAction, PergaminoNodeBehavior}, types::DataType};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct DialogueNode {
	pub text: String,
}

impl Default for DialogueNode {
	fn default() -> Self {
		Self { 
			text: Default::default(),
		}
	}
}

impl PergaminoNodeBehavior for DialogueNode {
	fn title(&self) -> String {
		"Dialogue".to_owned()
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
		_node_id: egui_snarl::NodeId,
		_inputs: &[egui_snarl::InPin],
		_outputs: &[egui_snarl::OutPin],
		ui: &mut egui::Ui,
		_nodes: &[PergaminoNode]
	) -> NodeAction {
		let mut _action = NodeAction::None;

		ui.vertical(|ui| {
			ui.set_max_width(240.0);
			ui.add_space(8.0);

			ui.horizontal(|ui| {
				let output = TextEdit::multiline(&mut self.text)
					.desired_rows(3)
					.desired_width(200.0)
					.char_limit(80)
					.show(ui);

				if output.response.changed() {
					if let Some(mut state) = egui::widgets::text_edit::TextEditState::load(ui.ctx(), output.response.id) {
						if let Some(primary_cursor) = state.cursor.char_range().map(|r| r.primary) {
							let galley = output.galley;
							let new_text = galley.rows.iter().map(|row| row.text()).collect::<Vec<String>>().join("\n");

							if new_text != self.text {
								let old_len = self.text.len();
								let new_len = new_text.len();
								self.text = new_text;
								let new_cursor_idx = if primary_cursor.index >= old_len {
									new_len
								} else {
									primary_cursor.index + (new_len - old_len)
								};
								let new_ccursor = egui::text::CCursor::new(new_cursor_idx);
								state.cursor.set_char_range(Some(egui::text::CCursorRange::one(new_ccursor)));
								state.store(ui.ctx(), output.response.id);
							}
						}
					}
				}
			});
		});

		_action
	}

	fn accent_color(&self) -> Color32 {
		Color32::from_rgb(37, 150, 190)
	}

	fn input_type(&self, _index: usize) -> Option<DataType> {
		Some(DataType::RegularStatement)
	}

	fn output_type(&self, _index: usize) -> Option<DataType> {
		Some(DataType::RegularStatement)
	}
}