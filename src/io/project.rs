use egui_snarl::Snarl;
use serde::{Serialize, Deserialize};

use crate::graph::node::PergaminoNode;


#[derive(Serialize, Deserialize)]
pub struct ProjectMetadata {
	pub project_name: String,
	pub language: String,
	pub created_at: String
}

impl Default for ProjectMetadata {
	fn default() -> Self {
		Self { 
			project_name: "Untitled".to_owned(), 
			language: "en".to_owned(), 
			created_at: chrono::Local::now().to_rfc3339()
		}
	}
}

#[derive(Serialize, Deserialize)]
pub struct PergaminoProjectFile {
	pub meta: ProjectMetadata,
	pub data: Snarl<PergaminoNode>
}