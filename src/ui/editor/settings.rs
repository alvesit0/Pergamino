use eframe::egui;
use crate::{io::project::ProjectSettings, ui::EditorUiState};

pub fn show(ctx: &egui::Context, settings: &mut ProjectSettings, ui_state: &mut EditorUiState) {
	egui::Modal::new(egui::Id::new("settings_modal"))
		.show(ctx, |ui| {
			ui.set_min_width(300.0);
			ui.heading("Project Settings");
			ui.separator();
			
			egui::Grid::new("settings_grid").num_columns(2).spacing([10.0, 10.0]).show(ui, |ui| {
				ui.label("Language:");
				ui.text_edit_singleline(&mut settings.language);
				ui.end_row();

				ui.label("Max Dialogue Chars:");
				ui.add(egui::DragValue::new(&mut settings.max_dialogue_chars).speed(1));
				ui.end_row();
			});

			ui.add_space(10.0);
			ui.separator();
			ui.horizontal(|ui| {
				if ui.button("Close").clicked() {
					ui_state.show_settings_modal = false;
				}
			});
		});
}