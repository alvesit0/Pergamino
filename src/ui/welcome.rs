use eframe::egui;

use super::AppState;

pub fn start(ctx: &egui::Context) {
    egui::ViewportCommand::center_on_screen(ctx);
    egui::ViewportCommand::InnerSize([400.0, 240.0].into());
    ctx.send_viewport_cmd(egui::ViewportCommand::Resizable(false));
    ctx.send_viewport_cmd(egui::ViewportCommand::Maximized(false));
}


pub fn show(ctx: &egui::Context) -> Option<AppState> {
    let mut next_state = None;

    egui::CentralPanel::default().show(ctx, |ui| {
        ui.with_layout(egui::Layout::top_down_justified(egui::Align::Center), |ui| {
            ui.add_space(30.0);
            ui.heading("📜 Pergamino");
            ui.add_space(30.0);
            if ui.add(egui::Button::new("Abrir Último Proyecto").min_size([200.0, 40.0].into())).clicked() {
                next_state = Some(AppState::NamingProject { temp_name: String::new() });
            }

            if ui.add(egui::Button::new("Crear Nuevo Proyecto").min_size([200.0, 40.0].into())).clicked() {
                next_state = Some(AppState::NamingProject { temp_name: String::new() });
            }
        });
    });

    next_state
}