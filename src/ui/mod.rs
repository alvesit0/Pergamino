use serde::{Deserialize, Serialize};
use egui_snarl::Snarl;

use crate::graph::node::PergaminoNode;

pub mod welcome;
pub mod create_project;
pub mod editor;
pub mod window_frame;
pub mod theme;

#[derive(Deserialize, Serialize, Clone)]
pub enum AppState {
    Welcome,
    NamingProject { temp_name: String },
    Editor { 
		project_name: String,
		snarl: Snarl<PergaminoNode>
	}
}

impl Default for AppState {
    fn default() -> Self {
        Self::Welcome
    }
}

impl PartialEq for AppState {
	fn eq(&self, other: &Self) -> bool {
		if std::mem::discriminant(self) != std::mem::discriminant(other) {
			return false;
        } else {
			return true;
		} 
	}
}