use std::collections::HashSet;

use egui::{Color32};
use egui_snarl::{ui::PinInfo};
use serde::{Serialize, Deserialize};

use crate::graph::{node::PergaminoNode, node_behavior::{GraphContext, NodeAction, NodeCategory, PergaminoNodeBehavior, UNLIMITED_CONNECTIONS}, types::DataType};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct InterruptNode {
	pub index: i16,
}

impl Default for InterruptNode {
	fn default() -> Self {
		Self { 
			index: Default::default() 
		}
	}
}

impl PergaminoNodeBehavior for InterruptNode {
	fn title(&self) -> String {
		"Interrupt".to_owned()
	}

	fn on_create(&mut self, nodes: &[PergaminoNode]) {
		let occupied: HashSet<i16> = nodes.iter()
			.filter_map(|n| match n {
				PergaminoNode::Interrupt(i) => Some(i.index),
				_ => None
			})
			.collect();
		
		for i in 1..=99 {
			if !occupied.contains(&i) {
				self.index = i;
				return;
			}
		}
	}

	fn inputs(&self) -> usize {
		0
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
		nodes: &[PergaminoNode],
		_context: &GraphContext
	) -> NodeAction {
		let mut action = NodeAction::None;

		let occupied_indices: HashSet<i16> = nodes.iter()
			.filter_map(|node| {
				match node {
					PergaminoNode::Interrupt(n) => Some(n.index),
					_ => None
				}
			})
			.collect();

		ui.vertical(|ui| {
			ui.set_min_width(80.0);
			ui.add_space(8.0);
			ui.centered_and_justified(|ui| {
				let valid_range = 1..=99;
				let mut desired_value = self.index;

				let response = ui.add(
					egui::DragValue::new(&mut desired_value)
					.speed(1.0)
					.range(valid_range.clone())
				);

				if response.changed() {
					let direction = if desired_value > self.index { 1 } else { -1 };
					let mut candidate = desired_value;

					while occupied_indices.contains(&candidate) {
						candidate += direction;
						if !valid_range.contains(&candidate) {
							candidate = self.index;
							break;
						}
					}

					if valid_range.contains(&candidate) && !occupied_indices.contains(&candidate) {
						self.index = candidate;
						action = NodeAction::Update;
					}
				}
			})
		});

		action
	}

	fn accent_color(&self) -> Color32 {
		Color32::from_rgb(233, 30, 99)
	}

	fn input_type(&self, _index: usize) -> Option<DataType> {
		Some(DataType::RegularStatement)
	}

	fn output_type(&self, _index: usize) -> Option<DataType> {
		Some(DataType::RegularStatement)
	}

	fn output_max_connections(&self,_index:usize) -> usize {
		1

		// match index {
		// 	0 => UNLIMITED_CONNECTIONS,
		// 	_ => 1
		// }
	}

	fn input_max_connections(&self,_index:usize) -> usize {
		UNLIMITED_CONNECTIONS
	}

	fn category(&self) -> NodeCategory {
		NodeCategory::Logic
	}
}