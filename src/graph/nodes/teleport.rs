use egui::{Color32, RichText};
use egui_snarl::{ui::PinInfo};
use serde::{Serialize, Deserialize};

use crate::{graph::{node::PergaminoNode, node_behavior::{GraphContext, NodeAction, NodeCategory, PergaminoNodeBehavior}, types::DataType}, ui::widgets::node_reference_text_edit::NodeReferenceTextEdit};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct TeleportNode {
	pub reference_id: String,
	pub x: f32,
	pub y: f32,
	pub z: f32,
}

impl Default for TeleportNode {
	fn default() -> Self {
		Self { 
			reference_id: Default::default(),
			x: Default::default(),
			y: Default::default(),
			z: Default::default(),
		}
	}
}

impl PergaminoNodeBehavior for TeleportNode {
	fn title(&self) -> String {
		"Teleport".to_owned()
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

		ui.vertical(|ui| {
			ui.add_space(8.0);
			ui.set_max_width(187.0);
			
			ui.vertical_centered(|ui| {
				if NodeReferenceTextEdit::new(&mut self.reference_id, ctx.node_references)
					.desired_width(187.0)
					.show(ui)
					.changed() {
					action = NodeAction::Update;
				}
			});

			ui.add_space(6.0);

			ui.horizontal(|ui| {
				ui.style_mut().spacing.item_spacing.x = 4.0;
				
				let mut axis_ui = |ui: &mut egui::Ui, val: &mut f32, label: &str, color: Color32| {
					let btn_txt = RichText::new(label).color(Color32::WHITE).strong();
					let btn = egui::Button::new(btn_txt)
						.fill(color)
						.min_size(egui::vec2(14.0, 14.0))
						.small();

					if ui.add(btn).on_hover_text(label).clicked() {
						*val = 0.0;
						action = NodeAction::Update;
					}

					if ui.add(egui::DragValue::new(val)
						.speed(0.01)
						.fixed_decimals(2)
					).changed() {
						action = NodeAction::Update;
					}
				};

				axis_ui(ui, &mut self.x, "X", Color32::from_rgb(200, 60, 60));
				axis_ui(ui, &mut self.y, "Y", Color32::from_rgb(60, 180, 60));
				axis_ui(ui, &mut self.z, "Z", Color32::from_rgb(60, 60, 200));
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