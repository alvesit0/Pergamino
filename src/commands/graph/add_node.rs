use egui::Pos2;
use egui_snarl::{InPinId, NodeId, OutPinId};

use crate::{commands::command::PergaminoCommand, graph::node::PergaminoNode};

#[derive(Clone)]
pub struct AddNodeCommand {
	pub prototype: PergaminoNode,
	pub pos: Pos2,

	pub initial_connection: Option<PendingConnection>,

	pub last_created_id: Option<NodeId>
}

#[derive(Clone)]
pub enum PendingConnection {
	// si arrastramos desde salida existente
	FromOutput { source: OutPinId, target_input_idx: usize },
	// si arrastramos desde entrada existente
	FromInput { source: InPinId, target_output_idx: usize },
}

impl PergaminoCommand for AddNodeCommand {
	fn execute(&mut self, snarl: &mut egui_snarl::Snarl<PergaminoNode>) {
		let new_id = snarl.insert_node(self.pos, self.prototype.clone());
		self.last_created_id = Some(new_id);

		if let Some(connection) = &self.initial_connection {
			match connection {
				PendingConnection::FromOutput { source, target_input_idx } => {
					let target = InPinId { node: new_id, input: *target_input_idx };
					snarl.connect(*source, target);
				},
				PendingConnection::FromInput { source, target_output_idx } => {
					let target = OutPinId { node: new_id, output: *target_output_idx };
					snarl.connect(target, *source);
				}
			}
		}
	}

	fn undo(&mut self, snarl: &mut egui_snarl::Snarl<PergaminoNode>) {
		if let Some(id) = self.last_created_id {
			snarl.remove_node(id);
			self.last_created_id = None;
		}
	}
	
	fn clone_box(&self) -> Box<dyn PergaminoCommand> {
		Box::new(self.clone())
	}
}