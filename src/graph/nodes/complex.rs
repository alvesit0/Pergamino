use egui::{Color32, TextEdit};
use egui_snarl::{NodeId, ui::PinInfo};
use serde::{Serialize, Deserialize};

use crate::graph::node_behavior::{NodeAction, PergaminoNodeBehavior};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct ComplexNode {
	pub num: f64,
	pub text: String,
	pub selected_target: Option<egui_snarl::NodeId>
}

impl Default for ComplexNode {
	fn default() -> Self {
		Self {
			num: 10.0,
			text: "".to_string(),
			selected_target: None
		}
	}
}

impl PergaminoNodeBehavior for ComplexNode {
	fn title(&self) -> String {
		"Complex".to_owned()
	}

	fn inputs(&self) -> usize {
		1
	}

	fn show_input(
			&mut self,
			_pin: &egui_snarl::InPin,
			_ui: &mut egui::Ui,
			) -> egui_snarl::ui::PinInfo {
		
		PinInfo::circle().with_fill(Color32::WHITE)
	}

	fn outputs(&self) -> usize {
		1
	}

	fn show_output(
			&mut self,
			_pin: &egui_snarl::OutPin,
			_ui: &mut egui::Ui,
		) -> egui_snarl::ui::PinInfo {
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
			_node_id: egui_snarl::NodeId,
			_inputs: &[egui_snarl::InPin],
			_outputs: &[egui_snarl::OutPin],
			ui: &mut egui::Ui,
			candidates: &[(NodeId, String)],
		) -> NodeAction {
		let mut action = NodeAction::None; // default action

		ui.vertical(|ui| {
			ui.set_max_width(240.0);

			ui.horizontal(|ui| {
				ui.label("Num:");
				ui.add(egui::DragValue::new(&mut self.num));
			});

			ui.horizontal(|ui| {
				ui.label("Txt:");
				ui.add(TextEdit::singleline(&mut self.text).desired_width(200.0));
			});

			ui.horizontal(|ui| {
				ui.label("Link:");

				let selected_text = match self.selected_target {
					Some(id) => format!("Node {:?}", id),
					None => "None".to_owned()
				};

				egui::ComboBox::from_id_salt("node_selector")
					.selected_text(selected_text)
					.show_ui(ui, |ui| {
						ui.selectable_value(&mut self.selected_target, None, "None");

						for (id, name) in candidates {
							ui.selectable_value(&mut self.selected_target, Some(*id), name);
						}
					})
			});

			ui.separator();
			
			ui.horizontal(|ui| {
				if ui.button("Delete self").clicked() {
				    action = NodeAction::RemoveSelf;
				}
			});
		});

		action
	}
}