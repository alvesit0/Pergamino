use std::sync::Arc;

use egui::{Ui, Response, TextBuffer, TextEdit, Color32};

pub struct BBCodeTextEdit<'a> {
	text: &'a mut String,
	desired_width: f32,
	desired_rows: usize
}

impl<'a> BBCodeTextEdit<'a> {
	pub fn new(text: &'a mut String) -> Self {
		Self {
			text,
			desired_width: 200.0,
			desired_rows: 3,
		}
	}

	pub fn desired_width(mut self, width: f32) -> Self {
		self.desired_width = width;
		self
	}

	pub fn rows(mut self, rows: usize) -> Self {
		self.desired_rows = rows;
		self
	}

	pub fn show(self, ui: &mut Ui) -> Response {
		let BBCodeTextEdit { 
			text, 
			desired_width, 
			desired_rows 
		} = self;

		let mut layouter = |ui: &Ui, buffer: &dyn TextBuffer, wrap_width: f32| -> Arc<egui::Galley> {
			let mut layout_job = egui::text::LayoutJob::default();
			let string = buffer.as_str();
			
			let gray_format = egui::TextFormat {
				color: Color32::GRAY,
				..Default::default()
			};
			let yellow_format = egui::TextFormat {
				color: Color32::YELLOW,
				..Default::default()
			};

			// algoritmo de resaltado simple
			// todo lo que esté entre '[' y ']' es amarillo. el resto es gris.
			let mut in_tag = false;
			let mut start_idx = 0;

			for (i, c) in string.char_indices() {
				if c == '[' && !in_tag {
					if i > start_idx {
						layout_job.append(&string[start_idx..i], 0.0, gray_format.clone());
					}
					start_idx = i;
					in_tag = true;
				} else if c == ']' && in_tag {
					let end_tag = i + c.len_utf8();
					layout_job.append(&string[start_idx..end_tag], 0.0, yellow_format.clone());
					start_idx = end_tag;
					in_tag = false;
				}
			}
			if start_idx < string.len() {
				let format = if in_tag { yellow_format } else { gray_format };
				layout_job.append(&string[start_idx..], 0.0, format);
			}

			layout_job.wrap.max_width = wrap_width;
			
			ui.fonts_mut(|f| f.layout_job(layout_job))
		};

		let edit = TextEdit::multiline(text)
			.desired_rows(desired_rows)
			.desired_width(desired_width)
			.layouter(&mut layouter);

		ui.add(edit)
	}
}