use eframe::egui;
use super::{AppState, window_frame};

pub fn start(ctx: &egui::Context) {
    // ctx.send_viewport_cmd(egui::ViewportCommand::InnerSize([400.0, 240.0].into()));
    ctx.send_viewport_cmd(egui::ViewportCommand::Maximized(false));
}

pub fn show(ctx: &egui::Context) -> Option<AppState> {
    let mut next_state = None;

    let config = window_frame::WindowConfig {
        title: "".to_string(),
        resizable: false,
        maximizable: false,
        closeable: true,
    };

    window_frame::show(ctx, config, |ui| {
        ui.with_layout(egui::Layout::top_down_justified(egui::Align::Center), |ui| {
            ui.add_space(30.0);
            ui.heading("📜 Pergamino");
            ui.add_space(30.0);

            if ui.add(egui::Button::new("Open Last Project").min_size([200.0, 40.0].into())).clicked() {
                next_state = Some(AppState::NamingProject { temp_name: String::new() });
            }

            if ui.add(egui::Button::new("Create New Project").min_size([200.0, 40.0].into())).clicked() {
                next_state = Some(AppState::NamingProject { temp_name: String::new() });
            }
        });
    });

    next_state
}