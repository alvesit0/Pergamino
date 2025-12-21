use eframe::egui;
use super::AppState;


pub fn start(ctx: &egui::Context) {
    egui::ViewportCommand::center_on_screen(ctx);
    // egui::ViewportCommand::InnerSize([400.0, 240.0].into());
}


pub fn show(ctx: &egui::Context, temp_name: &mut String) -> Option<AppState> {
    let mut next_state = None;

    let new_window_size = egui::vec2(300.0, 150.0);

    // 2. Obtenemos el rectángulo de la ventana actual (padre) en coordenadas de pantalla
    let parent_rect = ctx.viewport_rect();

    // 3. Calculamos la posición "top-left" para que la nueva ventana quede centrada
    // Restamos la mitad del tamaño de la nueva ventana al centro de la ventana padre
    let calculated_position = parent_rect.center() - (new_window_size / 2.0);


    ctx.show_viewport_immediate(
        egui::ViewportId::from_hash_of("create_project_window"),
        egui::ViewportBuilder::default()
            .with_title("Nuevo Proyecto")
            .with_inner_size(new_window_size)
            .with_resizable(false)
            .with_position(calculated_position)
            .with_always_on_top(),
            |_cts, _class| {

                if ctx.input(|i| i.viewport().close_requested()) {
                    next_state = Some(AppState::Welcome);
                }

                egui::CentralPanel::default().show(ctx, |ui| {
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
                })
        }
    );

    next_state
} 