use eframe::egui;

use super::AppState;

pub fn start(ctx: &egui::Context) {
    ctx.send_viewport_cmd(egui::ViewportCommand::Resizable(true));
    ctx.send_viewport_cmd(egui::ViewportCommand::InnerSize([1024.0, 720.0].into()));
    ctx.send_viewport_cmd(egui::ViewportCommand::Maximized(true));
}


pub fn show(ctx: &egui::Context, project_name: &str) -> Option<AppState> {

    let mut _next_state = None;

    egui::CentralPanel::default().show(ctx, |ui| {
        egui::MenuBar::new().ui(ui, |ui| {
            ui.menu_button("File", |io| {
                if io.button("Save").clicked() {
                }

                if io.button("Quit").clicked() {
                    std::process::exit(0);
                }
            });

            if ui.button("Undo").clicked() {
            }
            if ui.button("Redo").clicked() {
            }
        });

        ui.separator();
        ui.heading(format!("Proyecto: {}", project_name));
        ui.label("Canvas infinito de nodos...");
    });

    _next_state
} 