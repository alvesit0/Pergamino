use eframe::egui;
use super::AppState;

pub fn start(_ctx: &egui::Context) {
    // Ya no necesitamos lógica de viewport aquí
}

pub fn show(ctx: &egui::Context, temp_name: &mut String) -> Option<AppState> {
    let mut next_state = None;

    // Renderizamos una ventana modal dentro del contexto actual
    // en lugar de crear un Viewport del sistema.
    egui::Window::new("Nuevo Proyecto")
        .collapsible(false)
        .resizable(false)
        .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0]) // Centrado
        .fixed_size([300.0, 150.0])
        .show(ctx, |ui| {
            ui.with_layout(egui::Layout::top_down_justified(egui::Align::Center), |ui| {
                ui.add_space(10.0);
                ui.label("Nombre del proyecto");

                let response = ui.text_edit_singleline(temp_name);
                response.request_focus();

                ui.add_space(15.0);
                ui.horizontal(|ui| {
                    if ui.button("Cancelar").clicked() {
                        next_state = Some(AppState::Welcome);
                    }

                    let enter = response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter));

                    if (ui.button("Crear").clicked() || enter) && !temp_name.trim().is_empty() {
                        next_state = Some(AppState::Editor { project_name: temp_name.clone() });
                    }
                })
            })
        });

    next_state
}