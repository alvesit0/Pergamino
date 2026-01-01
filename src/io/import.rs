use std::{fs::File, path::Path};
use crate::io::{PergaminoProjectFile};

pub fn load_from_file(path: &Path) -> std::io::Result<PergaminoProjectFile> {
	let file = File::open(path)?;

	let project: PergaminoProjectFile = serde_json::from_reader(file)?;

	Ok(project)
}