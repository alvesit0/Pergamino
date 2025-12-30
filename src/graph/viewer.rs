use egui::Frame;
use egui_snarl::{Snarl, ui::{AnyPins, SnarlViewer}};

use crate::{graph::{node::PergaminoNode, node_behavior::{NodeAction, PergaminoNodeBehavior}}, ui::theme::PergaminoTheme};

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

		let from_type = snarl[from.id.node].output_type(from.id.output);
		let to_type = snarl[to.id.node].input_type(to.id.input);

		match (from_type, to_type) {
			(Some(out_t), Some(in_t)) => {
				if out_t.is_compatible_with(&in_t) {
					snarl.connect(from.id, to.id);
				} else {
					println!("Incompatible connection!!");
				}
			},
			_ => {
				// if any of the node types returns None somehow
			}
		}
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

	fn has_dropped_wire_menu(&mut self, _src_pins: egui_snarl::ui::AnyPins, _snarl: &mut Snarl<PergaminoNode>) -> bool {
		true
	}

	fn show_graph_menu(
		&mut self, pos: 
		egui::Pos2, 
		ui: &mut egui::Ui, 
		snarl: &mut egui_snarl::Snarl<PergaminoNode>) {
		
		ui.label("Add node");
		ui.separator();

		for node in PergaminoNode::prototypes() {
			if ui.button(node.title()).clicked() {
				snarl.insert_node(pos, node);
				ui.close();
			}
		}
	}

	fn show_dropped_wire_menu(
			&mut self,
			pos: egui::Pos2,
			ui: &mut egui::Ui,
			src_pins: egui_snarl::ui::AnyPins,
			snarl: &mut Snarl<PergaminoNode>,
		) {
		ui.label("Connect to...");
		ui.separator();

		let (src_out_id, src_in_id, src_type) = match src_pins {
			AnyPins::Out(pins) => {
				if let Some(&pin_id) = pins.first() {
					let node = &snarl[pin_id.node];
					let data_type = node.output_type(pin_id.output);

					(Some(pin_id), None, data_type)
				} else {
					return;
				}
			},
			AnyPins::In(pins) => {
				if let Some(&pin_id) = pins.first() {
					let node = &snarl[pin_id.node];
					let data_type = node.output_type(pin_id.input);

					(None, Some(pin_id), data_type)
				} else {
					return;
				}
			}
		};

		let src_type = if let Some(t) = src_type { t } else { return };

		for prototype in PergaminoNode::prototypes() {
			// buscamos un pin compatible con el prototipo
			let target_pin_index = if src_out_id.is_some() {
				// si arrastro SALIDA, busco ENTRADA compatible en el nuevo nodo
				(0..prototype.inputs()).find(|&i| {
					prototype.input_type(i).map_or(false, |t| src_type.is_compatible_with(&t))
				})
			} else {
				// si arrastro ENTRADA, busco SALIDA compatible en el nuevo nodo
				(0..prototype.outputs()).find(|&i| {
					prototype.output_type(i).map_or(false, |t| src_type.is_compatible_with(&t))
				})
			};

			// si encontramos compatibilidad, pintamos el botón
			if let Some(idx) = target_pin_index {
				if ui.button(prototype.title()).clicked() {
					let new_node_id = snarl.insert_node(pos, prototype);

					if let Some(from_id) = src_out_id {
						let to_id = egui_snarl::InPinId { node: new_node_id, input: idx };
						snarl.connect(from_id, to_id);
					}
					else if let Some(to_id) = src_in_id {
						let from_id = egui_snarl::OutPinId { node: new_node_id, output: idx };
						snarl.connect(from_id, to_id);
					}

					ui.close();
				}
			}
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