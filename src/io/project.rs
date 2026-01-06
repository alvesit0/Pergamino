use egui_snarl::Snarl;
use serde::{Serialize, Deserialize};

use crate::graph::node::PergaminoNode;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum VariableType {
	String,
	Number,
	Boolean
}

impl Default for VariableType {
	fn default() -> Self {
		Self::String
	}
}

impl ToString for VariableType {
	fn to_string(&self) -> String {
		match self {
			VariableType::String => "String".to_owned(),
			VariableType::Number => "Number".to_owned(),
			VariableType::Boolean => "Boolean".to_owned(),
		}
	}
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Variable {
	pub name: String,
	pub kind: VariableType,
	pub value: String
}

impl Default for Variable {
	fn default() -> Self {
		Self {
			name: "new_var".to_owned(),
			kind: VariableType::String,
			value: "".to_owned()
		}
	}
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ProjectSettings {
	pub language: String,
	pub max_dialogue_chars: usize
}

impl Default for ProjectSettings {
	fn default() -> Self {
		Self { 
			language: "en".to_owned(),
			max_dialogue_chars: 140
		}
	}
}

#[derive(Serialize, Deserialize)]
pub struct ProjectMetadata {
	pub project_name: String,
	pub created_at: String,
	#[serde(default)]
	pub settings: ProjectSettings,
	#[serde(default)]
	pub variables: Vec<Variable>
}

impl Default for ProjectMetadata {
	fn default() -> Self {
		Self { 
			project_name: "Untitled".to_owned(), 
			created_at: chrono::Local::now().to_rfc3339(),
			settings: ProjectSettings::default(),
			variables: Vec::new()
		}
	}
}

#[derive(Serialize, Deserialize)]
pub struct PergaminoProjectFile {
	pub meta: ProjectMetadata,
	pub data: Snarl<PergaminoNode>
}