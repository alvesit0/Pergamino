use egui_snarl::{InPinId, OutPinId};

use crate::{commands::command::PergaminoCommand, graph::node::PergaminoNode};

#[derive(Clone)]
pub struct AddConnectionCommand {
	pub from: OutPinId,
	pub to: InPinId
}

impl PergaminoCommand for AddConnectionCommand {
	fn execute(&mut self, snarl: &mut egui_snarl::Snarl<PergaminoNode>) {
		snarl.connect(self.from, self.to);
	}

	fn undo(&mut self, snarl: &mut egui_snarl::Snarl<PergaminoNode>) {
		snarl.disconnect(self.from, self.to);
	}

	fn clone_box(&self) -> Box<dyn PergaminoCommand> {
		Box::new(self.clone())
	}
}