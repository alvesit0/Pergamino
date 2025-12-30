use egui::{Color32, CornerRadius, Frame, Margin, Shadow, Stroke, Vec2};
use egui_snarl::ui::{BackgroundPattern, Grid, NodeLayout, PinPlacement, SnarlStyle, WireStyle};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PergaminoTheme {
	// pub editor_bg_color: Color32,
	pub editor_grid_color: Color32,
	pub editor_grid_spacing: Vec2,

	pub wire_width: f32,

	pub node_header_rouding: u8,
	pub node_bg_color: Color32,
	pub node_text_color: Color32,
}

impl Default for PergaminoTheme {
	fn default() -> Self {
		Self {
			// editor_bg_color: Color32::from_rgb(10, 10, 15),
			editor_grid_color: Color32::from_rgb(50, 50, 55),
			editor_grid_spacing: Vec2::new(40.0, 40.0),

			wire_width: 2.5,

			node_header_rouding: 8,
			node_bg_color: Color32::from_rgb(45, 45, 48),
			node_text_color: Color32::WHITE,
		}
	}
}

impl PergaminoTheme {
	pub fn apply_to_snarl_style(&self, style: &mut SnarlStyle) {
		style.bg_pattern = Some(BackgroundPattern::Grid(
			Grid { spacing: self.editor_grid_spacing, angle: 0.0 }
		));
		// style.bg_frame = Some(Frame::default().fill(self.editor_bg_color));

		style.bg_pattern_stroke = Some(Stroke { width: 1.0, color: self.editor_grid_color });

		style.wire_width = Some(self.wire_width);
		style.wire_style = Some(WireStyle::Bezier3);
		style.wire_smoothness = Some(0.0);
		style.header_drag_space = Some(Vec2::new(0.0, 0.0));

		style.node_layout = Some(NodeLayout::coil());

		style.pin_placement = Some(PinPlacement::Edge);

		style.collapsible = Some(false);

		style.pin_stroke = None;
	}

	pub fn node_frame(&self, color: Color32) -> Frame {
		Frame::default()
			.fill(self.node_bg_color)
			.stroke(Stroke::new(1.0, color))
			.corner_radius(CornerRadius::same(self.node_header_rouding))
			.inner_margin(Margin::symmetric(6, 10))
			.shadow(Shadow { 
				offset: [2, 2], 
				blur: 0, 
				spread: 0, 
				color: Color32::from_black_alpha(120)
			})
	}

	pub fn header_frame(&self, color: Color32) -> Frame {
		Frame::default()
			.fill(color)
			.corner_radius(CornerRadius::same(self.node_header_rouding - 2))
			// .corner_radius(CornerRadius {
			// 	nw: self.node_header_rouding,
			// 	ne: self.node_header_rouding,
			// 	sw: 0,
			// 	se: 0
			// })
			.inner_margin(Margin::symmetric(0, 6))
	}
}