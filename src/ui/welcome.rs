use std::{path::PathBuf};

use eframe::egui;
use rfd::FileDialog;
use crate::{commands::invoker::{CommandInvoker}, io::{self}};

use super::{AppState, window_frame};

pub fn start(ctx: &egui::Context) {
	// let width = 480.0;
	// let height = 320.0;
	
	// ctx.send_viewport_cmd(egui::ViewportCommand::InnerSize([width, height].into()));
	
    ctx.send_viewport_cmd(egui::ViewportCommand::Maximized(false));
}

pub fn show(ctx: &egui::Context, last_project: &Option<PathBuf>) -> Option<AppState> {
    let mut next_state = None;

    let config = window_frame::WindowConfig {
        title: "".to_string(),
        resizable: false,
        maximizable: false,
        closeable: true,
    };

	let width = 360.0;
	let height = 260.0;

	ctx.send_viewport_cmd(egui::ViewportCommand::InnerSize([width, height].into()));

    window_frame::show(ctx, config, |ui| {
        ui.with_layout(egui::Layout::top_down_justified(egui::Align::Center), |ui| {
            ui.add_space(30.0);
            ui.heading("📜 Pergamino");
            ui.add_space(30.0);

            if let Some(path) = last_project {
				let btn_text = format!("Open Last: {}", path.file_name().unwrap_or_default().to_string_lossy());

				if ui.add(egui::Button::new(btn_text).min_size([200.0, 40.0].into())).clicked() {
					match io::import::load_from_file(path) {
							Ok(project) => {
								let mut invoker = CommandInvoker::default();
								invoker.reset_saved_state();

								next_state = Some(AppState::Editor {
									project_name: project.meta.project_name,
									file_path: Some(path.clone()),
									snarl: project.data,
									invoker
								}) 
							},
							Err(e) => eprintln!("Error: {}", e),
						}
				} 
			} else {
				ui.add_enabled(false, egui::Button::new("No Recent Project").min_size([200.0, 40.0].into()));
			}

			if ui.add(egui::Button::new("Open Project").min_size([200.0, 40.0].into())).clicked() {
				if let Some(path) = FileDialog::new().add_filter("Pergamino", &["json"]).pick_file() {
					match io::import::load_from_file(&path) {
						Ok(project) => {
							let mut invoker = CommandInvoker::default();
							invoker.reset_saved_state();

							next_state = Some(AppState::Editor {
								project_name: project.meta.project_name,
								file_path: Some(path),
								snarl: project.data,
								invoker
							})
						},
						Err(e) => eprintln!("Error: {}", e)
					}
				}
            }

            if ui.add(egui::Button::new("Create New Project").min_size([200.0, 40.0].into())).clicked() {
                next_state = Some(AppState::NamingProject { temp_name: String::new() });
            }
        });
    });

    next_state
}