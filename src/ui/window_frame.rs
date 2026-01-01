use eframe::egui::{self, Id, RichText, Sense, UiBuilder, ViewportCommand, vec2, Align2, FontId};

pub struct WindowConfig {
	pub title: String,
	pub resizable: bool,
	pub maximizable: bool,
	pub closeable: bool,
}

impl Default for WindowConfig {
	fn default() -> Self {
		Self {
			title: "App".to_string(),
			resizable: true,
			maximizable: true,
			closeable: true,
		}
	}
}

pub fn show(
	ctx: &egui::Context,
	config: WindowConfig,
	add_contents: impl FnOnce(&mut egui::Ui),
) {
	egui::CentralPanel::default()
		// .frame(egui::Frame::NONE)
		.show(ctx, |ui| {
			let app_rect = ui.max_rect();

			let panel_frame = egui::Frame::new()
				.fill(ui.visuals().window_fill()) 
				.corner_radius(4.0) 
				.inner_margin(4.0)
				.stroke(ui.visuals().widgets.noninteractive.fg_stroke)
				.outer_margin(0.0); 

			panel_frame.show(ui, |ui| {
				ui.expand_to_include_rect(app_rect);

				// --- Barra de Título ---
				let title_bar_height = 22.0;
				let title_bar_rect = {
					let mut rect = app_rect;
					rect.max.y = rect.min.y + title_bar_height;
					rect
				};

				render_title_bar(ui, title_bar_rect, &config);

				let content_rect = {
					let mut rect = app_rect;
					rect.min.y = title_bar_rect.max.y;
					rect
				}
				.shrink(4.0);

				let mut content_ui = ui.new_child(UiBuilder::new().max_rect(content_rect));
				add_contents(&mut content_ui);

				if config.resizable {
					render_resize_grip(ui, app_rect);
				}
			});
		});
}

fn render_title_bar(ui: &mut egui::Ui, rect: egui::Rect, config: &WindowConfig) {
	let painter = ui.painter();

	let title_bar_response = ui.interact(
		rect,
		Id::new("title_bar"),
		Sense::click_and_drag(),
	);

	painter.text(
		rect.center(),
		Align2::CENTER_CENTER,
		&config.title,
		FontId::proportional(12.0),
		ui.visuals().text_color(),
	);

	painter.line_segment(
		[
			rect.left_bottom() + vec2(1.0, 0.0),
			rect.right_bottom() + vec2(-1.0, 0.0),
		],
		ui.visuals().widgets.noninteractive.bg_stroke,
	);

	if title_bar_response.double_clicked_by(egui::PointerButton::Primary) && config.maximizable {
		let is_maximized = ui.input(|i| i.viewport().maximized.unwrap_or(false));
		ui.ctx().send_viewport_cmd(ViewportCommand::Maximized(!is_maximized));
	}

	if title_bar_response.drag_started() && ui.input(|i| i.pointer.button_down(egui::PointerButton::Primary)) {
		ui.ctx().send_viewport_cmd(ViewportCommand::StartDrag);
	}

	ui.scope_builder(
		UiBuilder::new()
			.max_rect(rect)
			.layout(egui::Layout::right_to_left(egui::Align::Center)),
		|ui| {
			ui.spacing_mut().item_spacing.x = 4.0;
			ui.visuals_mut().button_frame = false;
			ui.add_space(8.0);

			if config.closeable {
				if ui.add(egui::Button::new(RichText::new("🗙").size(12.0))).clicked() {
					ui.ctx().send_viewport_cmd(ViewportCommand::Close);
				}
			}

			if config.maximizable {
				let is_maximized = ui.input(|i| i.viewport().maximized.unwrap_or(false));
				let icon = if is_maximized { "🗗" } else { "🗖" };
				if ui.add(egui::Button::new(RichText::new(icon).size(12.0))).clicked() {
					ui.ctx().send_viewport_cmd(ViewportCommand::Maximized(!is_maximized));
				}
			}

			if ui.add(egui::Button::new(RichText::new("🗕").size(12.0))).clicked() {
				ui.ctx().send_viewport_cmd(ViewportCommand::Minimized(true));
			}
		},
	);
}

fn render_resize_grip(ui: &mut egui::Ui, app_rect: egui::Rect) {
	let grip_size = vec2(12.0, 12.0);
	let grip_rect = egui::Rect::from_min_size(app_rect.max - grip_size, grip_size);
	let grip_response = ui.interact(grip_rect, Id::new("resize_grip"), Sense::drag());

	ui.painter().line_segment(
		[grip_rect.max - vec2(-4.0, 8.0), grip_rect.max - vec2(8.0, -4.0)],
		ui.visuals().widgets.noninteractive.fg_stroke,
	);

	if grip_response.hovered() {
		ui.ctx().set_cursor_icon(egui::CursorIcon::ResizeNwSe);
	}

	if grip_response.dragged() {
		if let Some(delta) = grip_response.drag_delta().try_into().ok() {
			if let Some(inner_rect) = ui.ctx().input(|i| i.viewport().inner_rect) {
				let current_size = inner_rect.size();

				let min_size = vec2(300.0, 200.0);

				let new_size = (current_size + delta).max(min_size);

				ui.ctx().send_viewport_cmd(ViewportCommand::InnerSize(new_size));
			}
		}
	}
}