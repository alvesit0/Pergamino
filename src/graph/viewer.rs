use egui::Frame;
use egui_snarl::{Snarl, ui::{SnarlViewer}};

use crate::{graph::{node::PergaminoNode, node_behavior::{NodeAction, PergaminoNodeBehavior}, nodes::{add::AddNode, complex::ComplexNode, number::NumberNode}}, ui::theme::PergaminoTheme};

pub struct PergaminoViewer {
	pub theme: PergaminoTheme,
}

impl SnarlViewer<PergaminoNode> for PergaminoViewer {
	fn title(&mut self, node: &PergaminoNode) -> String {
		node.title()
	}
	
	fn inputs(&mut self, node: &PergaminoNode) -> usize {
		node.inputs()
	}
	
	fn show_input(
		&mut self,
		pin: &egui_snarl::InPin,
		ui: &mut egui::Ui,
		snarl: &mut egui_snarl::Snarl<PergaminoNode>,
		) -> impl egui_snarl::ui::SnarlPin + 'static {
		self.apply_node_base_style(ui);

		let node = &mut snarl[pin.id.node];
		node.show_input(pin, ui)
	}
	
	fn outputs(&mut self, node: &PergaminoNode) -> usize {
		node.outputs()
	}
	
	fn show_output(
		&mut self,
		pin: &egui_snarl::OutPin,
		ui: &mut egui::Ui,
		snarl: &mut egui_snarl::Snarl<PergaminoNode>,
	) -> impl egui_snarl::ui::SnarlPin + 'static {
		self.apply_node_base_style(ui);

		let node = &mut snarl[pin.id.node];
		node.show_output(pin, ui) // cannot borrow `*snarl` as mutable more than once at a time
	}

	fn connect(
		&mut self, 
		from: &egui_snarl::OutPin, 
		to: &egui_snarl::InPin, 
		snarl: &mut egui_snarl::Snarl<PergaminoNode>) {

		snarl.connect(from.id, to.id);
	}

	fn disconnect(
		&mut self, 
		from: &egui_snarl::OutPin, 
		to: &egui_snarl::InPin, 
		snarl: &mut egui_snarl::Snarl<PergaminoNode>) {
		
		snarl.disconnect(from.id, to.id);
	}

	fn has_graph_menu(&mut self, _pos: egui::Pos2, _snarl: &mut egui_snarl::Snarl<PergaminoNode>) -> bool {
		true
	}

	fn has_node_menu(&mut self, node: &PergaminoNode) -> bool {
		node.has_node_menu()
	}

	fn show_graph_menu(
		&mut self, pos: 
		egui::Pos2, 
		ui: &mut egui::Ui, 
		snarl: &mut egui_snarl::Snarl<PergaminoNode>) {
		
		ui.label("Add node");
		if ui.button("Number").clicked() {
            snarl.insert_node(pos, PergaminoNode::Number(NumberNode { value: 0.0 }));
            ui.close();
        }
        if ui.button("Add").clicked() {
            snarl.insert_node(pos, PergaminoNode::Add(AddNode {}));
            ui.close();
        }
		ui.separator();
        if ui.button("Complex").clicked() {
            snarl.insert_node(pos, PergaminoNode::Complex(ComplexNode {
                num: 10.0,
                text: "Hello".to_string(),
                selected_target: None,
            }));
            ui.close();
        }
	}

	fn show_node_menu(
			&mut self,
			node_id: egui_snarl::NodeId,
			inputs: &[egui_snarl::InPin],
			outputs: &[egui_snarl::OutPin],
			ui: &mut egui::Ui,
			snarl: &mut egui_snarl::Snarl<PergaminoNode>,
		) {
		let action = {
			let node = &mut snarl[node_id];
			node.show_node_menu(node_id, inputs, outputs, ui)
		};

		Self::process_action(snarl, action, node_id);
	}

	fn has_body(&mut self, node: &PergaminoNode) -> bool {
		node.has_body()
	}

	fn show_body(
		&mut self,
		node_id: egui_snarl::NodeId,
		inputs: &[egui_snarl::InPin],
		outputs: &[egui_snarl::OutPin],
		ui: &mut egui::Ui,
		snarl: &mut Snarl<PergaminoNode>,
	) {
		self.apply_node_base_style(ui);

		let candidates: Vec<_> = snarl.node_ids()
			.filter(|(id, _)| *id != node_id)
			.map(|(id, _)| (id, format!("Node {:?}", id)))
			.collect();

		let action = {
			let node = &mut snarl[node_id];
			node.show_body(node_id, inputs, outputs, ui, &candidates)
		};

		Self::process_action(snarl, action, node_id);
	}

	fn show_header(
			&mut self,
			node_id: egui_snarl::NodeId,
			_inputs: &[egui_snarl::InPin],
			_outputs: &[egui_snarl::OutPin],
			ui: &mut egui::Ui,
			snarl: &mut Snarl<PergaminoNode>,
		) {
		self.apply_node_base_style(ui);
		let node = &snarl[node_id];

		ui.vertical_centered(|ui| {
			ui.label(node.title());
		});	
	}

	fn header_frame(
	    &mut self,
	    _default: Frame,
	    node_id: egui_snarl::NodeId,
	    _inputs: &[egui_snarl::InPin],
	    _outputs: &[egui_snarl::OutPin],
	    snarl: &Snarl<PergaminoNode>,
	) -> Frame {
		let node = &snarl[node_id];

		let color = node.accent_color();

		self.theme.header_frame(color)
	}

	fn node_frame(
	    &mut self,
	    _default: Frame,
	    node_id: egui_snarl::NodeId,
	    _inputs: &[egui_snarl::InPin],
	    _outputs: &[egui_snarl::OutPin],
	    snarl: &Snarl<PergaminoNode>,
	) -> Frame {
		let node = &snarl[node_id];

		let color = node.accent_color();

		self.theme.node_frame(color)
	}
}

impl PergaminoViewer {
	fn apply_node_base_style(&self, ui: &mut egui::Ui) {
		ui.visuals_mut().override_text_color = Some(self.theme.node_text_color);
	}

	fn process_action(
		snarl: &mut Snarl<PergaminoNode>,
		action: NodeAction,
		node_id: egui_snarl::NodeId
	) {
		match action {
			NodeAction::RemoveSelf => {
				snarl.remove_node(node_id);
			}
			NodeAction::Connect(from, to) => {
				snarl.connect(from.id, to.id);
			}
			NodeAction::Disconnect(from, to) => {
				snarl.disconnect(from.id, to.id);
			}
			NodeAction::None => {}
		}
	}
}