use egui::{Frame, RichText, Ui};
use egui_snarl::{InPinId, OutPinId, Snarl, ui::{AnyPins, SnarlViewer}};

use crate::{commands::{graph::{
	add_connection::AddConnectionCommand, 
	add_node::{AddNodeCommand, PendingConnection}, 
	remove_connection::RemoveConnectionCommand, 
	remove_node::RemoveNodeCommand}, 
	invoker::CommandInvoker}, 
	graph::{
		node::PergaminoNode, 
		node_behavior::{
			GraphContext, 
			NodeAction, 
			PergaminoNodeBehavior, 
			UNLIMITED_CONNECTIONS
		}}, 
		io::project::{ProjectSettings, Variable}, 
		ui::theme::PergaminoTheme
	};

// ' indica lifetime, "a" podría ser cualquier cosa
// especificar esto cuando se tienen referencias se debe hacer porque si 
// no Rust dice: "y si se destruye esta referencia, yo que hago con ella?"
// 'a le promete al compilador que la estructura PergaminoViewer NUNCA vivirá
// más que las referencias 'a que contiene.
// al establecer esto, intentar matar a &theme o &mut invoker antes que a 
// PergaminoViewer provocará que el compilador se queje
pub struct PergaminoViewer<'a> {
	pub theme: &'a PergaminoTheme,
	pub invoker: &'a mut CommandInvoker,
	pub settings: &'a ProjectSettings,
	pub variables: &'a [Variable]
}

impl<'a> SnarlViewer<PergaminoNode> for PergaminoViewer<'a> {
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

		let ctx = self.get_context();

		let node_id = pin.id.node;

		let (pin_info, action) = snarl[node_id].show_input(pin, ui, &ctx);

		Self::process_action(snarl, action, node_id, self.invoker);

		pin_info
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

		let ctx = self.get_context();

		let node_id = pin.id.node;

		let (pin_info, action) = snarl[node_id].show_output(pin, ui, &ctx);

		Self::process_action(snarl, action, node_id, self.invoker);

		pin_info // cannot borrow `*snarl` as mutable more than once at a time
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
					self.enforce_pin_limits(snarl, Some(from.id), Some(to.id));

					let cmd = AddConnectionCommand { from: from.id, to: to.id };
					self.invoker.execute_command(cmd, snarl);
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
		let cmd = RemoveConnectionCommand { from: from.id, to: to.id };
		self.invoker.execute_command(cmd, snarl);
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

		let current_nodes: Vec<PergaminoNode> = snarl.node_ids()
			.map(|(_, n)| n.clone())
			.collect();

		for node in PergaminoNode::prototypes() {
			if ui.button(node.title()).clicked() {
				let mut new_node = node.clone();

				new_node.on_create(&current_nodes);

				let cmd = AddNodeCommand {
					prototype: new_node,
					pos,
					initial_connection: None,
					last_created_id: None
				};
				self.invoker.execute_command(cmd, snarl);
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

		// obtener pin de salida, pin de entrada y tipo de pin de salida
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

		let current_nodes: Vec<PergaminoNode> = snarl.node_ids()
			.map(|(_, n)| n.clone())
			.collect();

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
					let mut new_node = prototype.clone();
					new_node.on_create(&current_nodes);

					self.enforce_pin_limits(snarl, src_out_id, src_in_id);

					let connection_info = if let Some(from_id) = src_out_id {
						Some(PendingConnection::FromOutput { 
							source: from_id, 
							target_input_idx: idx 
						})
					} else if let Some(to_id) = src_in_id {
						Some(PendingConnection::FromInput { 
							source: to_id, 
							target_output_idx: idx
						})
					} else {
						None
					};

					let cmd = AddNodeCommand {
						prototype: new_node,
						pos,
						initial_connection: connection_info,
						last_created_id: None
					};

					self.invoker.execute_command(cmd, snarl);
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

		Self::process_action(snarl, action, node_id, self.invoker);
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

		let ctx = self.get_context();

		let other_nodes: Vec<_> = snarl.node_ids()
			.filter(|(id, _)| *id != node_id)
			.map(|(_, node)| node.clone())
			.collect();
		

		let action = {
			let node = &mut snarl[node_id];
			node.show_body(node_id, inputs, outputs, ui, &other_nodes, &ctx)
		};

		Self::process_action(snarl, action, node_id, self.invoker);
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

		let full_rect = ui.max_rect();

		ui.scope_builder(
			egui::UiBuilder::new().max_rect(full_rect),
			|ui| {
				ui.vertical_centered(|ui| {
					ui.label(RichText::new(node.title()).strong().size(15.0));
				});
			}
		);
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

impl<'a> PergaminoViewer<'a> {
	fn get_context(&'_ self) -> GraphContext<'_> {
		GraphContext { 
			settings: self.settings,
			variables: self.variables
		}
	}

	fn apply_node_base_style(&self, ui: &mut egui::Ui) {
		// text color only for now lol
		ui.visuals_mut().override_text_color = Some(self.theme.node_text_color);
	}

	pub fn check_inputs(&mut self, ui: &mut Ui, snarl: &mut Snarl<PergaminoNode>) {
		if ui.input_mut(|i| i.consume_key(egui::Modifiers::CTRL, egui::Key::Z)) {
			self.invoker.undo_command(snarl);
		}

		if ui.input_mut(|i| i.consume_key(egui::Modifiers::CTRL, egui::Key::Y)) {
			self.invoker.redo_command(snarl);
		}
	}

	fn process_action(
		snarl: &mut Snarl<PergaminoNode>,
		action: NodeAction,
		node_id: egui_snarl::NodeId,
		invoker: &mut CommandInvoker
	) {
		match action {
			NodeAction::RemoveSelf => {
				let cmd = RemoveNodeCommand::new(node_id, snarl);
				invoker.execute_command(cmd, snarl);
			}
			NodeAction::Connect(from, to) => {
				let cmd = AddConnectionCommand { from: from.id, to: to.id };
				invoker.execute_command(cmd, snarl);
			}
			NodeAction::Disconnect(from, to) => {
				let cmd = RemoveConnectionCommand { from: from.id, to: to.id };
				invoker.execute_command(cmd, snarl);
			},
			NodeAction::Update => {
				invoker.force_dirty();
			}
			NodeAction::None => {}
		}
	}

	fn enforce_pin_limits(
		&mut self,
		snarl: &mut Snarl<PergaminoNode>,
		from: Option<OutPinId>,
		to: Option<InPinId>
	) {
		if let Some(out_pin) = from {
			let node = &snarl[out_pin.node];
			let max_wires = node.output_max_connections(out_pin.output);

			if max_wires < UNLIMITED_CONNECTIONS {
				let wires: Vec<_> = snarl.wires()
					.filter(|(src, _dst)| *src == out_pin)
					.collect();

				// si añadir un nodo excede el limite, borramos los mas antiguos
				if wires.len() >= max_wires {
					let to_remove_count = wires.len() - max_wires + 1;

					for i in 0..to_remove_count {
						let (src, dst) = wires[i];
						let cmd = RemoveConnectionCommand { from: src, to: dst };
						self.invoker.execute_command(cmd, snarl);
					}
				}
			}
		}

		if let Some(in_pin) = to {
			let node = &snarl[in_pin.node];
			let max_wires = node.input_max_connections(in_pin.input);

			if max_wires < UNLIMITED_CONNECTIONS {
				let wires: Vec<_> = snarl.wires()
					.filter(|(_src, dst)| *dst == in_pin)
					.collect();

				if wires.len() >= max_wires {
					let to_remove_count = wires.len() - max_wires + 1;

					for i in 0..to_remove_count {
						let (src, dst) = wires[i];
						let cmd = RemoveConnectionCommand { from: src, to: dst };
						self.invoker.execute_command(cmd, snarl);
					}
				}
			}
		}
	}
}