use eframe::egui;
use egui_extras::{TableBuilder, Column};
use crate::{io::project::{NodeReference}, ui::EditorUiState};

pub fn show(ctx: &egui::Context, node_references: &mut Vec<NodeReference>, ui_state: &mut EditorUiState) {
	egui::Modal::new(egui::Id::new("node_references_modal"))
		.show(ctx, |ui| {
			ui.set_width(300.0);
			ui.set_max_height(400.0);
			ui.heading("Node References");
			ui.separator();

			let available_height = ui.available_height();
			let footer_height = 80.0;
			let table_height = (available_height - footer_height).max(50.0);

			TableBuilder::new(ui)
				.striped(true)
				.resizable(false)
				.cell_layout(egui::Layout::left_to_right(egui::Align::Center))
				.column(Column::auto()) 
				.column(Column::remainder())
				.min_scrolled_height(0.0)
				.max_scroll_height(table_height)
				.header(20.0, |mut header| {
					header.col(|_| {});
					header.col(|ui| { ui.strong("Name"); });
				})
				.body(|mut body| {
					let mut to_remove = None;
					for (i, reference) in node_references.iter_mut().enumerate() {
						body.row(20.0, |mut row| {
							row.col(|ui| {
								if ui.button("🗑").clicked() {
									to_remove = Some(i);
								}
							});
							row.col(|ui| {
								ui.text_edit_singleline(&mut reference.name);
							});
						});
					}

					if let Some(idx) = to_remove {
						node_references.remove(idx);
					}
				});

			ui.add_space(10.0);
			if ui.button("+ Add Reference").clicked() {
				node_references.push(NodeReference::default());
			}

			ui.separator();

			let layout = egui::Layout::right_to_left(egui::Align::TOP);
			ui.with_layout(layout,|ui| {
				if ui.button("Close").clicked() {
					ui_state.show_node_references_modal = false;
				}
			});
		});
}