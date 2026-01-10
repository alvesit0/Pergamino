use std::path::PathBuf;

use egui_snarl::Snarl;

use crate::{commands::invoker::CommandInvoker, graph::node::PergaminoNode, io::project::{NodeReference, ProjectSettings, Variable}};

pub mod welcome;
pub mod editor;
pub mod window_frame;
pub mod theme;
pub mod widgets;

#[derive(Clone, Default)]
pub struct EditorUiState {
	pub show_settings_modal: bool,
	pub show_variables_modal: bool,
	pub show_node_references_modal: bool,
	pub show_about_modal: bool,
}

#[derive(Clone)]
pub enum AppState {
    Welcome,
    NamingProject { temp_name: String },
    Editor { 
		project_name: String,
		// ruta del archivo actual
		file_path: Option<PathBuf>,

		snarl: Snarl<PergaminoNode>,
		invoker: CommandInvoker,

		settings: ProjectSettings,
		variables: Vec<Variable>,
		node_references: Vec<NodeReference>,

		ui_state: EditorUiState
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