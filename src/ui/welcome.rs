use eframe::egui;
use super::AppState;

pub fn show(ctx: &egui::Context) -> Option<AppState> {
	let mut next_state = None;

	egui::CentralPanel::default().show(ctx, |ui| {
		ui.vertical_centered(|ui| {
			ui.add_space(50.0);
			ui.heading("📜 Pergamino");
			ui.add_space(30.0);

			if ui.add(egui::Button::new("Crear Nuevo Proyecto").min_size([200.0, 40.0].into())).clicked() {
                // devolvemos el siguiente estado
                next_state = Some(AppState::NamingProject { temp_name: String::new() });
            }
		})
	});

	next_state
}