use egui::{CollapsingHeader, Color32};
use egui_snarl::{ui::PinInfo};
use serde::{Serialize, Deserialize};

use crate::{graph::{node::PergaminoNode, node_behavior::{GraphContext, NodeAction, NodeCategory, PergaminoNodeBehavior}, types::DataType}, ui::widgets::node_reference_text_edit::NodeReferenceTextEdit};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum MovementDirection {
	UpLeft, Up, UpRight,
	Left, Right,
	DownLeft, Down, DownRight
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct MovementNode {
	pub reference_id: String,
	pub direction: MovementDirection,
	#[serde(default)]
	pub amount: f32,
	#[serde(default)]
	pub pathfind: bool,
	#[serde(default)]
	pub wait_till_end: bool,
}

impl Default for MovementNode {
	fn default() -> Self {
		Self { 
			reference_id: Default::default(),
			direction: MovementDirection::Down,
			amount: 1.0,
			pathfind: true,
			wait_till_end: true,
		}
	}
}

impl PergaminoNodeBehavior for MovementNode {
	fn title(&self) -> String {
		"Movement".to_owned()
	}

	fn on_create(&mut self, _nodes: &[PergaminoNode]) {

	}

	fn inputs(&self) -> usize {
		1
	}

	fn show_input(&mut self, 
		_pin: &egui_snarl::InPin, 
		_ui: &mut egui::Ui,
		_context: &GraphContext
	) -> (PinInfo, NodeAction) {
		(
			PinInfo::circle().with_fill(DataType::RegularStatement.color()), 
			NodeAction::None
		)
	}

	fn outputs(&self) -> usize {
		1
	}

	fn show_output(
		&mut self, 
		_pin: &egui_snarl::OutPin, 
		_ui: &mut egui::Ui,
		_context: &GraphContext
	) -> (PinInfo, NodeAction) {
		(
			PinInfo::circle().with_fill(DataType::RegularStatement.color()), 
			NodeAction::None
		)
	}

	fn show_body(
		&mut self,
		_node_id:egui_snarl::NodeId,
		_inputs: &[egui_snarl::InPin],
		_outputs: &[egui_snarl::OutPin],
		ui: &mut egui::Ui,
		_nodes: &[PergaminoNode],
		ctx: &GraphContext
	) -> NodeAction {
		let mut action = NodeAction::None;

		let btn_size = egui::vec2(30.0, 30.0);

		ui.vertical(|ui| {
			ui.add_space(8.0);
			ui.set_max_width(94.0);
			ui.vertical(|ui| {
				ui.vertical_centered(|ui| {
					// ui.add(egui::TextEdit::singleline(&mut self.reference_id).hint_text("Node ref..."));
					if NodeReferenceTextEdit::new(&mut self.reference_id, ctx.node_references)
						.desired_width(94.0)
						.show(ui)
						.changed() {
						action = NodeAction::Update;
					}
				});

				ui.add_space(4.0);

				ui.horizontal(|ui| {
					ui.centered_and_justified(|ui| {
						let response = ui.add(
							egui::DragValue::new(&mut self.amount)
							.speed(0.05)
						);

						if response.changed() {
							if self.amount < 0.0 {
								self.amount = 0.0;
							}
							action = NodeAction::Update;
						}
					});
				});

				ui.add_space(4.0);

				let mut dir_btn = |ui: &mut egui::Ui, label: &str, dir: MovementDirection| {
					let is_selected = self.direction == dir;
					if ui.add_sized(btn_size, egui::Button::new(label).selected(is_selected)).clicked() {
						if dir != self.direction {
							action = NodeAction::Update;
						}
						self.direction = dir;
					}
				};

				egui::Grid::new("movement_grid")
					.spacing(egui::vec2(2.0, 2.0))
					.min_col_width(0.0)
					.show(ui, |ui| {

					dir_btn(ui, "↖", MovementDirection::UpLeft);
					dir_btn(ui, "⬆", MovementDirection::Up);
					dir_btn(ui, "↗", MovementDirection::UpRight);

					ui.end_row();

					dir_btn(ui, "⬅", MovementDirection::Left);
					ui.allocate_space(btn_size);
					dir_btn(ui, "➡", MovementDirection::Right);

					ui.end_row();

					dir_btn(ui, "↙", MovementDirection::DownLeft);
					dir_btn(ui, "⬇", MovementDirection::Down);
					dir_btn(ui, "↘", MovementDirection::DownRight);
				});

				ui.add_space(8.0);

				CollapsingHeader::new("Settings")
				.default_open(false)
				.show(ui, |ui| {
					if ui.checkbox(&mut self.pathfind, "Pathfind").changed() {
						action = NodeAction::Update;
					}
					if ui.checkbox(&mut self.wait_till_end, "Wait till end").changed() {
						action = NodeAction::Update;
					}
				});
			});
		});

		action
	}

	fn accent_color(&self) -> Color32 {
		Color32::from_rgb(196, 120, 39)
	}

	fn input_type(&self, _index: usize) -> Option<DataType> {
		Some(DataType::RegularStatement)
	}

	fn output_type(&self, _index: usize) -> Option<DataType> {
		Some(DataType::RegularStatement)
	}

	fn category(&self) -> crate::graph::node_behavior::NodeCategory {
		NodeCategory::Movement
	}
}