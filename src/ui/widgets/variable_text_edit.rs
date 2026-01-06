use std::sync::{Arc, OnceLock};
use egui::{Color32, TextBuffer, TextEdit, Ui, Response};
use crate::io::project::Variable;

pub struct VariableTextEdit<'a> {
	text: &'a mut String,
	variables: &'a [Variable],
	char_limit: usize,
	multiline: bool,
	desired_width: f32,
	desired_rows: usize,
}

impl<'a> VariableTextEdit<'a> {
	pub fn new(text: &'a mut String, variables: &'a [Variable]) -> Self {
		Self {
			text,
			variables,
			char_limit: usize::MAX,
			multiline: false,
			desired_width: 200.0,
			desired_rows: 1,
		}
	}

	pub fn multiline(mut self, rows: usize) -> Self {
		self.multiline = true;
		self.desired_rows = rows;
		self
	}

	pub fn singleline(mut self) -> Self {
		self.multiline = false;
		self.desired_rows = 1;
		self
	}

	pub fn char_limit(mut self, limit: usize) -> Self {
		self.char_limit = limit;
		self
	}

	pub fn desired_width(mut self, width: f32) -> Self {
		self.desired_width = width;
		self
	}

	pub fn show(self, ui: &mut Ui) -> Response {
		let VariableTextEdit { 
			text, 
			variables, 
			char_limit, 
			multiline, 
			desired_width, 
			desired_rows 
		} = self;

		if multiline && char_limit < usize::MAX {
			let char_count = text.chars().count();
			if char_count > char_limit {
				let byte_index = text.char_indices().nth(char_limit).map(|(i, _)| i).unwrap_or(text.len());
				text.truncate(byte_index);
			}
		}

		let mut layouter = |ui: &Ui, buffer: &dyn TextBuffer, wrap_width: f32| -> Arc<egui::Galley> {
			let mut layout_job = egui::text::LayoutJob::default();
			let string = buffer.as_str();
			
			let default_text_format = egui::TextFormat {
				color: ui.visuals().text_color(),
				..Default::default()
			};

			static REGEX: OnceLock<regex::Regex> = OnceLock::new();
			let regex = REGEX.get_or_init(|| {
				regex::Regex::new(r"(\$\{)([\w]+)(\})").unwrap()
			});
			
			let mut last_end = 0;

			for mat in regex.find_iter(string) {
				if mat.start() > last_end {
					layout_job.append(&string[last_end..mat.start()], 0.0, default_text_format.clone());
				}

				let cap = regex.captures(mat.as_str()).unwrap();
				let var_name = cap.get(2).unwrap().as_str();

				let exists = variables.iter().any(|v| v.name == var_name);
				let color = if exists { Color32::GREEN } else { Color32::RED };
				
				let var_format = egui::TextFormat {
					color,
					..Default::default()
				};

				layout_job.append(mat.as_str(), 0.0, var_format);
				last_end = mat.end();
			}

			if last_end < string.len() {
				layout_job.append(&string[last_end..], 0.0, default_text_format.clone());
			}

			layout_job.wrap.max_width = wrap_width;

			ui.fonts_mut(|f| f.layout_job(layout_job))
		};

		let mut edit = if multiline {
			TextEdit::multiline(text).desired_rows(desired_rows)
		} else {
			TextEdit::singleline(text)
		};

		edit = edit
			.desired_width(desired_width)
			.layouter(&mut layouter);

		if !multiline && char_limit < usize::MAX {
			edit = edit.char_limit(char_limit);
		}

		ui.add(edit)
	}
}