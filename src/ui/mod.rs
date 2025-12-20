use serde::{Deserialize, Serialize};

pub mod welcome;
pub mod create_project;
pub mod editor;

// partialeq permite compararlos, retornando false si cualquier campo es desigual
#[derive(Deserialize, Serialize, PartialEq)]
pub enum AppState {
	Welcome,
	NamingProject { temp_name: String },
	Editor { project_name: String }
}

impl Default for AppState {
	fn default() -> Self {
		Self::Welcome
	}
}