use eframe::egui;
use egui::{Color32};
use egui_snarl::Snarl;
use egui_snarl::ui::{PinInfo, SnarlStyle, SnarlViewer};
use super::{AppState, window_frame};
use serde::{Serialize, Deserialize};

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
pub enum MyNode {
	Number(f64),
	Add,
	Parametrizable {
		my_number: f64,
		my_text: String,
		selected_target: Option<egui_snarl::NodeId>
	}
}

pub struct MyViewer;

impl SnarlViewer<MyNode> for MyViewer {
	fn title(&mut self, node: &MyNode) -> String {
		match node {
			MyNode::Number(_) => "Number".to_owned(),
			MyNode::Add => "Sum".to_owned(),
			MyNode::Parametrizable { .. } => "Configuration".to_owned()
		}
	}
	
	fn inputs(&mut self, node: &MyNode) -> usize {
		match node {
			MyNode::Number(_) => 0,
			MyNode::Add => 2,
			MyNode::Parametrizable { .. } => 1
		}
	}
	
	fn show_input(
		&mut self,
		_pin: &egui_snarl::InPin,
		ui: &mut egui::Ui,
		_snarl: &mut egui_snarl::Snarl<MyNode>,
		) -> impl egui_snarl::ui::SnarlPin + 'static {
		
		ui.label("In");
		PinInfo::circle().with_fill(Color32::WHITE)
	}
	
	fn outputs(&mut self, node: &MyNode) -> usize {
		match node {
			MyNode::Number(_) => 1,
			MyNode::Add => 1,
			MyNode::Parametrizable { .. } => 1,
		}
	}
	
	fn show_output(
		&mut self,
		pin: &egui_snarl::OutPin,
		ui: &mut egui::Ui,
		snarl: &mut egui_snarl::Snarl<MyNode>,
	) -> impl egui_snarl::ui::SnarlPin + 'static {

		match snarl.get_node_mut(pin.id.node) {
			Some(MyNode::Number(val)) => {
				ui.add(egui::DragValue::new(val).speed(0.1));
			}
			Some(MyNode::Add) => {
				ui.label("Out");
			},
			Some(MyNode::Parametrizable { .. }) => {
				ui.label("Result");
			},
			None => {}
		}

		PinInfo::circle().with_fill(Color32::RED)
	}

	fn connect(
		&mut self, 
		from: &egui_snarl::OutPin, 
		to: &egui_snarl::InPin, 
		snarl: &mut egui_snarl::Snarl<MyNode>) {

		snarl.connect(from.id, to.id);
	}

	fn disconnect(
		&mut self, 
		from: &egui_snarl::OutPin, 
		to: &egui_snarl::InPin, 
		snarl: &mut egui_snarl::Snarl<MyNode>) {
		
		snarl.disconnect(from.id, to.id);
	}

	fn has_graph_menu(&mut self, _pos: egui::Pos2, _snarl: &mut egui_snarl::Snarl<MyNode>) -> bool {
		true
	}

	fn has_node_menu(&mut self, _node: &MyNode) -> bool {
		true
	}

	fn show_graph_menu(
		&mut self, pos: 
		egui::Pos2, 
		ui: &mut egui::Ui, 
		snarl: &mut egui_snarl::Snarl<MyNode>) {
		
		ui.label("Add node");
		if ui.button("Number").clicked() {
			snarl.insert_node(pos, MyNode::Number(0.0));
			ui.close();
		}
		if ui.button("Add").clicked() {
			snarl.insert_node(pos, MyNode::Add);
			ui.close();
		}
		ui.separator();
		if ui.button("Complex").clicked() {
			snarl.insert_node(pos, MyNode::Parametrizable {
				my_number: 10.0,
				my_text: "Hello".to_string(),
				selected_target: None
			});
			ui.close();
		}
	}

	fn show_node_menu(
			&mut self,
			node: egui_snarl::NodeId,
			_inputs: &[egui_snarl::InPin],
			_outputs: &[egui_snarl::OutPin],
			ui: &mut egui::Ui,
			snarl: &mut egui_snarl::Snarl<MyNode>,
		) {
		if ui.button("Remove node").clicked() {
			snarl.remove_node(node);
			ui.close();
		}
	}

	fn has_body(&mut self, node: &MyNode) -> bool {
		match node {
			MyNode::Parametrizable { .. } => true,
			_ => false
		}
	}

	fn show_body(
		&mut self,
		node_id: egui_snarl::NodeId,
		_inputs: &[egui_snarl::InPin],
		_outputs: &[egui_snarl::OutPin],
		ui: &mut egui::Ui,
		snarl: &mut Snarl<MyNode>,
	) {
		let mut candidates = Vec::new();

		// CORRECTO SEGÚN TU DOCUMENTACIÓN:
		// snarl.node_ids() devuelve tuplas (NodeId, &MyNode).
		// Desempaquetamos la tupla aquí mismo en el bucle.
		for (id, _) in snarl.node_ids() {
			if id != node_id {
				candidates.push((id, format!("Node {:?}", id)));
			}
		}

		// A partir de aquí todo sigue igual...
		if let Some(MyNode::Parametrizable { my_number, my_text, selected_target }) = snarl.get_node_mut(node_id) {
			
			ui.vertical(|ui| {
				ui.horizontal(|ui| {
					ui.label("Num:");
					ui.add(egui::DragValue::new(my_number));
				});

				ui.horizontal(|ui| {
					ui.label("Txt:");
					ui.text_edit_singleline(my_text);
				});

				ui.horizontal(|ui| {
					ui.label("Link:");
					
					let selected_text = match selected_target {
						Some(id) => format!("Node {:?}", id),
						None => "None".to_owned(),
					};

					egui::ComboBox::from_id_salt("node_selector")
						.selected_text(selected_text)
						.show_ui(ui, |ui| {
							
							ui.selectable_value(selected_target, None, "None");
							
							for (id, name) in candidates {
								ui.selectable_value(selected_target, Some(id), name);
							}
						});
				});
			});
		}
	}
}

pub fn start(ctx: &egui::Context) {
	let width = 1024.0;
	let height = 720.0;
	
	ctx.send_viewport_cmd(egui::ViewportCommand::InnerSize([width, height].into()));

	let monitor_size = ctx.input(|i| i.viewport().monitor_size);

	if let Some(monitor_size) = monitor_size {
		let x = (monitor_size.x - width) / 2.0;
		let y = (monitor_size.y - height) / 2.0;

		ctx.send_viewport_cmd(egui::ViewportCommand::OuterPosition([x, y].into()));
	} else {
		println!("Monitor change cannot be detected");
	}
}

pub fn show(ctx: &egui::Context, project_name: &str, snarl: &mut egui_snarl::Snarl<MyNode>) -> Option<AppState> {
    let mut _next_state = None;

    let config = window_frame::WindowConfig {
        title: format!("Project: {}", project_name),
        resizable: true,
        maximizable: true,
        closeable: true,
    };

    window_frame::show(ctx, config, |ui| {
        egui::MenuBar::new().ui(ui, |ui| {
            ui.menu_button("File", |io| {
                if io.button("Save").clicked() { }

                if io.button("Quit").clicked() {
                    ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                }
            });

            if ui.button("Undo").clicked() { }
            if ui.button("Redo").clicked() { }
        });

        ui.separator();

		let mut viewer = MyViewer;
		let style = SnarlStyle::new();
		let snarl_id = egui::Id::new("pergamino_graph_id");

		snarl.show(&mut viewer, &style, snarl_id, ui);
    });

    _next_state
}