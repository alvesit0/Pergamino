use eframe::egui::{self, RichText};
use crate::{io::project::ProjectSettings, ui::EditorUiState};

pub fn show(ctx: &egui::Context, _settings: &mut ProjectSettings, ui_state: &mut EditorUiState) {
	egui::Modal::new(egui::Id::new("about_modal"))
		.show(ctx, |ui| {
			ui.set_min_width(320.0);
			
			ui.vertical_centered(|ui| {
				ui.add_space(10.0);
				
				ui.label(RichText::new("PERGAMINO").heading().strong().size(24.0));
				
				ui.label(RichText::new(format!("v{}", env!("CARGO_PKG_VERSION"))).small().weak());
				
				ui.add_space(10.0);

				ui.label("A lightweight node-based JSON editor");
				ui.label("for writing game dialogue and events.");

				ui.add_space(10.0);
				ui.separator();
				ui.add_space(10.0);

				ui.label("Created by Adrián Alves (alvesito)");
				ui.label("Originally developed for RÓDRICA");

				ui.add_space(10.0);

				if ui.link("Source Code on GitHub").clicked() {
					let _ = ui.ctx().open_url(egui::OpenUrl::new_tab("https://github.com/alvesit0/Pergamino"));
				}
				
				ui.add_space(4.0);
				
				ui.small("Licensed under MIT License");

				ui.add_space(10.0);
				ui.weak("Built with Rust, egui & egui-snarl");
			});

			ui.add_space(15.0);
			ui.separator();
			
			ui.horizontal(|ui| {
				ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
					if ui.button("Close").clicked() {
						ui_state.show_about_modal = false;
					}
				});
			});
		});
}