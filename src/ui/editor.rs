use eframe::egui;
use egui_snarl::ui::{SnarlStyle};
use crate::{graph::{node::PergaminoNode, viewer::PergaminoViewer}, ui::theme::PergaminoTheme};

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

pub fn show(ctx: &egui::Context, project_name: &str, snarl: &mut egui_snarl::Snarl<PergaminoNode>) -> Option<AppState> {
    let mut _next_state = None;

    let config = window_frame::WindowConfig {
        title: format!("Project: {}", project_name),
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

        // ui.separator();
		ui.add_space(4.0);

		let theme = PergaminoTheme::default();

		let mut style = SnarlStyle::new();
		theme.apply_to_snarl_style(&mut style);

		let mut viewer = PergaminoViewer { theme };
		let snarl_id = egui::Id::new("pergamino_graph_id");

		snarl.show(&mut viewer, &style, snarl_id, ui);
    });

    _next_state
}