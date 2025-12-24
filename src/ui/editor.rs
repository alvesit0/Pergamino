use eframe::egui;
use super::{AppState, window_frame};

pub fn start(ctx: &egui::Context) {
    ctx.send_viewport_cmd(egui::ViewportCommand::InnerSize([1024.0, 720.0].into()));
    // ctx.send_viewport_cmd(egui::ViewportCommand::OuterPosition([200.0, 200.0].into()));
	egui::ViewportCommand::center_on_screen(ctx).unwrap();
}

pub fn show(ctx: &egui::Context, project_name: &str) -> Option<AppState> {
    let mut _next_state = None;

    let config = window_frame::WindowConfig {
        title: format!("Proyecto: {}", project_name),
        resizable: true,
        maximizable: true,
        closeable: true,
    };

    window_frame::show(ctx, config, |ui| {
        egui::MenuBar::new().ui(ui, |ui| {
            ui.menu_button("File", |io| {
                if io.button("Save").clicked() { }

                if io.button("Quit").clicked() {
                    ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                }
            });

            if ui.button("Undo").clicked() { }
            if ui.button("Redo").clicked() { }
        });

        ui.separator();
        ui.heading(format!("Editando: {}", project_name));
        ui.label("Canvas infinito de nodos...");
    });

    _next_state
}