use egui_snarl::Snarl;

use crate::graph::node::PergaminoNode;

pub trait PergaminoCommand {
	fn execute(&mut self, snarl: &mut Snarl<PergaminoNode>);
	fn undo(&mut self, snarl: &mut Snarl<PergaminoNode>);

	fn clone_box(&self) -> Box<dyn PergaminoCommand>;
}