use std::usize;

use egui_snarl::Snarl;

use crate::{commands::command::PergaminoCommand, graph::node::PergaminoNode};

#[derive(Clone)]
pub struct CommandInvoker {
	undo_stack: Vec<Box<dyn PergaminoCommand>>,
	redo_stack: Vec<Box<dyn PergaminoCommand>>,
	saved_undo_count: usize
}

impl Default for CommandInvoker {
	fn default() -> Self {
		Self { 
			undo_stack: Vec::default(), 
			redo_stack: Vec::default(),
			saved_undo_count: 0
		}
	}
}

impl CommandInvoker {
	// añadir + 'static significa que ese parámetro NO contiene referencias prestadas
	// de corta duración. es dueño de TODOS SUS DATOS o sólo contiene referencias
	// que viven para siempre
	// no confundir con su funcionalidad en referencias (&static' str), que significa
	// que la referencia es válida durante toda la ejecución del progrma
	pub fn execute_command(&mut self, mut command: impl PergaminoCommand + 'static, snarl: &mut Snarl<PergaminoNode>) {
		command.execute(snarl);
		self.redo_stack.clear();
		self.undo_stack.push(Box::new(command));
		// consume command
	}

	pub fn undo_command(&mut self, snarl: &mut Snarl<PergaminoNode>) {
		if let Some(mut command) = self.undo_stack.pop() {
			command.undo(snarl);
			self.redo_stack.push(command);
			// command.undo(); // esto ya no se puede hacer porque push consume command
		}
	}

	pub fn redo_command(&mut self, snarl: &mut Snarl<PergaminoNode>) {
		if let Some(mut command) = self.redo_stack.pop() {
			command.execute(snarl);
			self.undo_stack.push(command);
		}
	}

	// llamar cuando se guarda el archivo con éxito
	pub fn mark_as_saved(&mut self) {
		self.saved_undo_count = self.undo_stack.len();
	}

	pub fn is_dirty(&self) -> bool {
		self.undo_stack.len() != self.saved_undo_count
	}

	// pub fn force_dirty(&mut self) {
	// 	self.saved_undo_count = usize::MAX;
	// }

	pub fn reset_saved_state(&mut self) {
		self.saved_undo_count = self.undo_stack.len();
	}
}

impl Clone for Box<dyn PergaminoCommand> {
	fn clone(&self) -> Self {
		self.clone_box()
	}
}