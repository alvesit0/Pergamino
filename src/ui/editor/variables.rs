use eframe::egui;
use egui_extras::{TableBuilder, Column};
use crate::{io::project::{Variable, VariableType}, ui::EditorUiState};

pub fn show(ctx: &egui::Context, variables: &mut Vec<Variable>, ui_state: &mut EditorUiState) {
	egui::Modal::new(egui::Id::new("variables_modal"))
		.show(ctx, |ui| {
			ui.set_min_width(500.0);
			ui.set_max_height(400.0);
			ui.heading("Project Variables");
			ui.separator();

			TableBuilder::new(ui)
				.striped(true)
				.resizable(true)
				.cell_layout(egui::Layout::left_to_right(egui::Align::Center))
				.column(Column::auto()) // Delete button
				.column(Column::initial(150.0).resizable(true)) // Name
				.column(Column::initial(100.0).resizable(true)) // Type
				.column(Column::remainder()) // Value
				.min_scrolled_height(0.0)
				.header(20.0, |mut header| {
					header.col(|_| {});
					header.col(|ui| { ui.strong("Name"); });
					header.col(|ui| { ui.strong("Type"); });
					header.col(|ui| { ui.strong("Value"); });
				})
				.body(|mut body| {
					let mut to_remove = None;
					for (i, var) in variables.iter_mut().enumerate() {
						body.row(20.0, |mut row| {
							row.col(|ui| {
								if ui.button("🗑").clicked() {
									to_remove = Some(i);
								}
							});
							row.col(|ui| {
								ui.text_edit_singleline(&mut var.name);
							});
							row.col(|ui| {
								egui::ComboBox::from_id_salt(format!("combo_{}", i))
									.selected_text(var.kind.to_string())
									.show_ui(ui, |ui| {
										ui.selectable_value(&mut var.kind, VariableType::String, "String");
										ui.selectable_value(&mut var.kind, VariableType::Number, "Number");
										ui.selectable_value(&mut var.kind, VariableType::Boolean, "Boolean");
									});
							});
							row.col(|ui| {
								ui.text_edit_singleline(&mut var.value);
							});
						});
					}

					if let Some(idx) = to_remove {
						variables.remove(idx);
					}
				});

			ui.add_space(10.0);
			if ui.button("+ Add Variable").clicked() {
				variables.push(Variable::default());
			}

			ui.separator();
			ui.horizontal(|ui| {
				if ui.button("Close").clicked() {
					ui_state.show_variables_modal = false;
				}
			});
		});
}