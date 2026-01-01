use std::{fs::File, path::Path};
use egui_snarl::Snarl;
use crate::{graph::node::PergaminoNode, io::{PergaminoProjectFile, ProjectMetadata}};

pub fn save_to_file(path: &Path, snarl: &Snarl<PergaminoNode>, name: &str) -> std::io::Result<()> {
	let project_file = PergaminoProjectFile {
		meta: ProjectMetadata {
			project_name: name.to_string(),
			..Default::default()
		},
		data: snarl.clone()
	};

	let file = File::create(path)?;

	serde_json::to_writer_pretty(file, &project_file)?;
	
	Ok(())
}