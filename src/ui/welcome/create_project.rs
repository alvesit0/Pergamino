use eframe::egui;
use crate::{commands::invoker::CommandInvoker, io::project::ProjectSettings, ui::EditorUiState};

use super::super::AppState;
use egui_snarl::Snarl;

pub fn start(_ctx: &egui::Context) {
    // ya no necesitamos lógica de viewport aquí
}

pub fn show(ctx: &egui::Context, temp_name: &mut String) -> Option<AppState> {
    let mut next_state = None;

    egui::Window::new("New Project")
        .collapsible(false)
        .resizable(false)
        .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0]) // Centrado
        .fixed_size([300.0, 150.0])
        .show(ctx, |ui| {
            ui.with_layout(egui::Layout::top_down_justified(egui::Align::Center), |ui| {
                ui.add_space(8.0);
                ui.label("Project name");
				ui.add_space(4.0);

                let response = ui.text_edit_singleline(temp_name);
                response.request_focus();

                ui.add_space(12.0);

				let layout = egui::Layout::right_to_left(egui::Align::TOP);

                ui.with_layout(layout,|ui| {
                    let enter = response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter));

                    if (ui.button("Create").clicked() || enter) && !temp_name.trim().is_empty() {
                        next_state = Some(AppState::Editor {
							project_name: temp_name.clone(),
							file_path: None,
							snarl: Snarl::new(),
							invoker: CommandInvoker::default(),
                            settings: ProjectSettings::default(),
                            variables: Vec::new(),
							node_references: Vec::new(),
                            ui_state: EditorUiState::default(),
						});
                    }

					if ui.button("Cancel").clicked() {
                        next_state = Some(AppState::Welcome);
                    }
                })
            })
        });

    next_state
}