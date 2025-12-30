use egui::Color32;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DataType {
    Number,
    String,
    Any,
}

impl DataType {
	pub fn color(&self) -> Color32 {
		match self {
			DataType::Number => Color32::from_rgb(100, 200, 255),
			DataType::String => Color32::from_rgb(255, 200, 100),
			DataType::Any => Color32::GRAY,
		}
	}

	pub fn is_compatible_with(&self, other: &DataType) -> bool {
		match (self, other) {
			(DataType::Any, _) => true,
			(_, DataType::Any) => true,
			(a, b) => a == b,
		}
	}
}