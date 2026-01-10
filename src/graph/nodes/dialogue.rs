use egui::{Color32, CollapsingHeader};
use egui_snarl::{ui::PinInfo};
use serde::{Serialize, Deserialize};

use crate::{
	graph::{node::PergaminoNode, node_behavior::{GraphContext, NodeAction, NodeCategory, PergaminoNodeBehavior}, types::DataType},
	ui::widgets::{bbcode_text_edit::BBCodeTextEdit, variable_text_edit::VariableTextEdit}
};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct DialogueNode {
	pub text: String,
	#[serde(default)]
	pub bbcode_text: String,
}

impl Default for DialogueNode {
	fn default() -> Self {
		Self { 
			text: Default::default(),
			bbcode_text: Default::default(),
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
		_ctx: &GraphContext
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
		_ctx: &GraphContext
	) -> (PinInfo, NodeAction) {
		(
			PinInfo::circle().with_fill(DataType::RegularStatement.color()), 
			NodeAction::None
		)
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
				let prev_text = self.text.clone();

				let response = VariableTextEdit::new(&mut self.text, ctx.variables)
					.multiline(3)
					.desired_width(200.0)
					.char_limit(ctx.settings.max_dialogue_chars)
					.show(ui);

				if response.changed() {
					action = NodeAction::Update;
					
					if self.bbcode_text.is_empty() {
						self.bbcode_text = self.text.clone();
					} else {
						self.sync_bbcode_text(&prev_text);
					}
				}
			});

			ui.add_space(4.0);

			CollapsingHeader::new("BBCode Modifier")
				.default_open(false)
				.show(ui, |ui| {
					let bb_response = BBCodeTextEdit::new(&mut self.bbcode_text)
						.rows(4)
						.desired_width(182.0)
						.show(ui);
					
					if bb_response.changed() {
						action = NodeAction::Update;
					}
				});
		});

		action
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

	fn category(&self) -> NodeCategory {
		NodeCategory::Text
	}
}

impl DialogueNode {
	// this is entirely made by the ai, probably glitchy
	fn sync_bbcode_text(&mut self, prev_text: &str) {
		let new_text = &self.text;
		
		let prefix_len = common_prefix_len(prev_text, new_text);
		let suffix_len = common_suffix_len(prev_text, new_text, prefix_len);

		let removed_len = prev_text.len() - prefix_len - suffix_len;
		
		let inserted_text = &new_text[prefix_len .. new_text.len() - suffix_len];

		let start_idx = map_logical_to_physical_index(&self.bbcode_text, prefix_len);
		let end_idx = map_logical_to_physical_index(&self.bbcode_text, prefix_len + removed_len);

		if start_idx <= self.bbcode_text.len() && end_idx <= self.bbcode_text.len() {
			self.bbcode_text.replace_range(start_idx..end_idx, inserted_text);
		} else {
			eprintln!("Warning: Desync in BBCode mapping, forcing rewrite.");
			self.bbcode_text = self.text.clone();
		}
	}
}

fn common_prefix_len(a: &str, b: &str) -> usize {
	a.bytes().zip(b.bytes())
		.take_while(|(x, y)| x == y)
		.count()
}

fn common_suffix_len(a: &str, b: &str, prefix_len: usize) -> usize {
	let a_bytes = a.as_bytes();
	let b_bytes = b.as_bytes();
	
	let max_suffix_len = std::cmp::min(a.len() - prefix_len, b.len() - prefix_len);
	
	let mut suffix_len = 0;
	while suffix_len < max_suffix_len {
		if a_bytes[a.len() - 1 - suffix_len] != b_bytes[b.len() - 1 - suffix_len] {
			break;
		}
		suffix_len += 1;
	}
	suffix_len
}

fn map_logical_to_physical_index(bbcode: &str, logical_target: usize) -> usize {
	let mut logical_count = 0;
	let mut physical_idx = 0;
	let mut in_tag = false;
	let mut chars = bbcode.char_indices().peekable();

	while let Some((idx, c)) = chars.next() {
		if logical_count == logical_target && !in_tag {
			return idx;
		}

		if c == '[' {
			in_tag = true;
		} else if c == ']' {
			in_tag = false;
			if logical_count == logical_target {
				return idx + 1;
			}
		} else if !in_tag {
			logical_count += 1;
		}
		
		physical_idx = idx + c.len_utf8();
	}

	if logical_count == logical_target {
		return physical_idx;
	}

	physical_idx
}