use std::path::PathBuf;

use eframe::egui;
use egui_snarl::ui::{SnarlStyle};
use rfd::FileDialog;
use crate::{commands::invoker::CommandInvoker, graph::{node::PergaminoNode, viewer::PergaminoViewer}, io, ui::theme::PergaminoTheme};

use super::{AppState, window_frame};

pub fn start(ctx: &egui::Context) {
	let width = 1024.0;
	let height = 720.0;
	
	ctx.send_viewport_cmd(egui::ViewportCommand::InnerSize([width, height].into()));

	let monitor_size = ctx.input(|i| i.viewport().monitor_size);

	if let Some(monitor_size) = monitor_size {
		let x = (monitor_size.x - width) / 2.0;
		let y = (monitor_size.y - height) / 2.0;

		ctx.send_viewport_cmd(egui::ViewportCommand::OuterPosition([x, y].into()));
	} else {
		println!("Monitor change cannot be detected");
	}
}

pub fn show(
	ctx: &egui::Context,
	project_name: &mut String,
	file_path: &mut Option<PathBuf>,
	snarl: &mut egui_snarl::Snarl<PergaminoNode>,
	invoker: &mut CommandInvoker
) -> Option<AppState> {
    let mut _next_state = None;

	let is_dirty = file_path.is_none() || invoker.is_dirty();

    let config = window_frame::WindowConfig {
        title: format!("Pergamino - {} {}", project_name, if is_dirty { "*" } else { "" }),
        resizable: true,
        maximizable: true,
        closeable: true,
    };

	if ctx.input_mut(|i| i.consume_key(egui::Modifiers::CTRL, egui::Key::S)) {
		save_project(file_path, snarl, project_name, invoker);
	}

    window_frame::show(ctx, config, |ui| {
        egui::MenuBar::new().ui(ui, |ui| {
            ui.menu_button("File", |io_menu| {
                if io_menu.button("Save").clicked() {
					save_project(file_path, snarl, project_name, invoker);
					io_menu.close();
				}

				if io_menu.button("Save As...").clicked() {
					let dialog = FileDialog::new().set_file_name(format!("{}.json", project_name));
					
					if let Some(mut path) = dialog.save_file() {
						if path.extension().unwrap_or_default() != "json" {
							path.set_extension("json");
						}

						if let Some(stem) = path.file_stem() {
							*project_name = stem.to_string_lossy().to_string();
						}

						if let Err(e) = io::export::save_to_file(&path, snarl, project_name) {
							eprintln!("Export error: {}", e);
						} else {
							// actualizar ruta si el path es el mismo
							*file_path = Some(path);
						}
					}
					io_menu.close();
				}

				io_menu.separator();

                if io_menu.button("Quit").clicked() {
                    ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                }
            });

            if ui.button("Undo").clicked() { invoker.undo_command(snarl); }
            if ui.button("Redo").clicked() { invoker.redo_command(snarl); }
        });

        // ui.separator();
		ui.add_space(4.0);

		let theme = PergaminoTheme::default();

		let mut style = SnarlStyle::new();
		theme.apply_to_snarl_style(&mut style);

		let mut viewer = PergaminoViewer { 
			theme: &theme, 
			invoker: invoker
		};
		let snarl_id = egui::Id::new("pergamino_graph_id");

		viewer.check_inputs(ui, snarl);

		snarl.show(&mut viewer, &style, snarl_id, ui);
    });

    _next_state
}

pub fn save_project(
	file_path: &mut Option<PathBuf>, 
	snarl: &mut egui_snarl::Snarl<PergaminoNode>, 
	project_name: &mut String, 
	invoker: &mut CommandInvoker
) {
	let path_option = if let Some(p) = file_path {
		Some(p.clone())
	} else {
		FileDialog::new().set_file_name(format!("{}.json", project_name)).save_file()
	};

	if let Some(mut path) = path_option {
		if path.extension().unwrap_or_default() != "json" {
			path.set_extension("json");
		}

		if file_path.is_none() {
			if let Some(stem) = path.file_stem() {
				*project_name = stem.to_string_lossy().to_string();
			}
		}

		if let Err(e) = io::export::save_to_file(&path, snarl, project_name) {
			eprintln!("Save error: {}", e);
		} else {
			// actualizar ruta si el path es el mismo
			*file_path = Some(path);
			invoker.mark_as_saved();
		}
	}
}