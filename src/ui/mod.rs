use std::path::PathBuf;

use egui_snarl::Snarl;

use crate::{commands::invoker::CommandInvoker, graph::node::PergaminoNode};

pub mod welcome;
pub mod editor;
pub mod window_frame;
pub mod theme;

#[derive(Clone)]
pub enum AppState {
    Welcome,
    NamingProject { temp_name: String },
    Editor { 
		project_name: String,
		// ruta del archivo actual
		file_path: Option<PathBuf>,

		snarl: Snarl<PergaminoNode>,
		invoker: CommandInvoker
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