 #![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

mod graph;
mod commands;
mod ui;
mod io;
mod nodes;

// modulos propios
use ui::{AppState};
use eframe::egui;

use serde::{Deserialize, Serialize};

fn main() -> eframe::Result {

    env_logger::init();

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([400.0, 240.0])
            .with_resizable(false)
            .with_maximize_button(false),
        centered: true,
        ..Default::default() // esto es el equivalente a hacer "...defaultOptions" en typescript
    };

    eframe::run_native(
        "Pergamino",
        options,
        // ### Box ###
        // - por defecto, rust asigna todo en el STACK (pila). este requiere que el compilador sepa
        //   el tamaño exacto de la variable en tiempo de compilación
        // - run_native espera recibir una closure (funcion lambda) para ejecutarla, pero tanto
        //   las Closures como los Traits pueden tener un tamaño dinámico o desconocido al compilar
        // - Box::new(x) reserva memoria en el HEAP y añade un puntero al STACK que apunta al HEAP
        Box::new(|cc| {
            // This gives us image support:
            egui_extras::install_image_loaders(&cc.egui_ctx);

            // esto es equivalente a poner "si cc.storage no es nulo (aunque los nulos no existen
            // en rust), crea una variable 'storage' con ese contenido y ejecuta el bloque"
            // if let Some(storage) = cc.storage {
            //  let app: PergaminoApp = eframe::get_value(storage, eframe::APP_KEY)
            //      .unwrap_or_default();

                // ### tipos de unwrap ###
                // - unwrap_or(valor): si es None/Err, retorna valor
                // - unwrap_or_default(): si es None/Err, retorna el valor por defecto
                //   (0 para números, "" para Strings, false para bools, o implementacion de Default)
                // - unwrap_or_else(|err| ...): si la operación principal tiene éxito, el código del
                //   else nunca se ejecuta. ideal si el plan B es costoso
                // - OPERADOR '?': si falla, retorna inmediatamente un error que se propaga hacia arriba.
                //   es azucar sintactico de:
                //      match operacion {
                //          Ok(valor) => valor,
                //          Err(e) => return Err(e), // Return prematuro
                //      }
            //  return Ok(Box::new(app));
            // }

            // cc.egui_ctx.send_viewport_cmd(egui::ViewportCommand::InnerSize([400.0, 240.0].into()));

            Ok(Box::<PergaminoApp>::default())
        }),
    )
}


#[derive(Deserialize, Serialize)]
#[serde(default)]

struct PergaminoApp {
    #[serde(skip)]
    state: AppState,
    #[serde(skip)]
    previous_state: Option<AppState>
}


impl Default for PergaminoApp {
    fn default() -> Self {
        Self {
            state:AppState::Welcome,
            previous_state:None
        }
    }
}


impl eframe::App for PergaminoApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.set_pixels_per_point(1.25);

        let potential_new_state = match &mut self.state {
            AppState::Welcome => {
                ui::welcome::show(ctx)
            },

            AppState::NamingProject { temp_name } => {
                ui::welcome::show(ctx);
                ui::create_project::show(ctx, temp_name)
            },

            AppState::Editor { project_name } => {
                ui::editor::show(ctx, project_name)
            }

        };


        if let Some(new_state) = potential_new_state {
            if new_state != self.state {
                match &new_state {
                    AppState::Welcome => ui::welcome::start(ctx),
                    AppState::NamingProject { .. } => ui::create_project::start(ctx),
                    AppState::Editor { .. } => ui::editor::start(ctx),
                }

                self.previous_state = Some(self.state.clone());
                self.state = new_state;

            }

        }

    }

    fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {
        // executes just before closing the app
    }
}