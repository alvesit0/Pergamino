use std::sync::{Arc};
use egui::{Color32, TextBuffer, TextEdit, Ui, Response};
use crate::{io::project::NodeReference};

pub struct NodeReferenceTextEdit<'a> {
	text: &'a mut String,
	node_references: &'a [NodeReference],
	desired_width: f32,
}

impl<'a> NodeReferenceTextEdit<'a> {
	pub fn new(text: &'a mut String, node_references: &'a [NodeReference]) -> Self {
		Self {
			text,
			node_references: node_references,
			desired_width: 200.0,
		}
	}

	pub fn desired_width(mut self, width: f32) -> Self {
		self.desired_width = width;
		self
	}

	pub fn show(self, ui: &mut Ui) -> Response {
		let NodeReferenceTextEdit { 
			text, 
			node_references, 
			desired_width, 
		} = self;

		let mut layouter = |ui: &Ui, buffer: &dyn TextBuffer, wrap_width: f32| -> Arc<egui::Galley> {
			let string = buffer.as_str();
			let exists = node_references.iter().any(|r| r.name == string);
			let color = if exists { Color32::GREEN } else { Color32::RED };

			let layout_job = egui::text::LayoutJob::simple(
				string.to_owned(),
				egui::FontId::default(),
				color,
				wrap_width
			);

			ui.fonts_mut(|f| f.layout_job(layout_job))
		};

		ui.add(TextEdit::singleline(text)
			.desired_width(desired_width)
			.layouter(&mut layouter)
		)
	}
}