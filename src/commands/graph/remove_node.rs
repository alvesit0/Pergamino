use egui::Pos2;
use egui_snarl::{InPinId, NodeId, OutPinId, Snarl};

use crate::{commands::command::PergaminoCommand, graph::node::PergaminoNode};

#[derive(Clone)]
pub struct RemoveNodeCommand {
	pub node_id: NodeId,
	pub saved_node: Option<PergaminoNode>,
	pub saved_pos: Option<Pos2>,
	pub saved_wires: Vec<(OutPinId, InPinId)>,
	pub restored_id: Option<NodeId>
}

impl RemoveNodeCommand {
	pub fn new(node_id: NodeId, snarl: &Snarl<PergaminoNode>) -> Self {
		let saved_wires: Vec<_> = snarl.wires()
			.filter(|(out_pin, in_pin)| out_pin.node == node_id || in_pin.node == node_id)
			.collect();

		Self { 
			node_id,
			saved_node: None,
			saved_pos: None,
			saved_wires,
			restored_id: None
		}
	}
}

impl PergaminoCommand for RemoveNodeCommand {
	fn execute(&mut self, snarl: &mut egui_snarl::Snarl<PergaminoNode>) {
		let target_id = self.restored_id.unwrap_or(self.node_id);

		if let Some(info) = snarl.get_node_info(target_id) {
			self.saved_pos = Some(info.pos);
		}

		if snarl.get_node(target_id).is_some() {
			self.saved_node = Some(snarl.remove_node(target_id));
		}
	}

	fn undo(&mut self, snarl: &mut egui_snarl::Snarl<PergaminoNode>) {
		if let (Some(node), Some(pos)) = (self.saved_node.take(), self.saved_pos) {
			// restauramos el nodo
			let new_id = snarl.insert_node(pos, node);
			self.restored_id = Some(new_id);

			// restauramos los cables
			for (from, to) in &self.saved_wires {
				let new_from = if from.node == self.node_id {
					OutPinId { node: new_id, output: from.output }
				} else { *from };

				let new_to = if to.node == self.node_id {
					InPinId { node: new_id, input: to.input }
				} else { *to };

				snarl.connect(new_from, new_to);
			}
		}
	}

	fn clone_box(&self) -> Box<dyn PergaminoCommand> {
		Box::new(self.clone())
	}
}