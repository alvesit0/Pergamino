#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

// modulos propios
mod graph;
mod commands;
mod ui;
mod io;

mod app;

use crate::app::PergaminoApp;

fn main() -> eframe::Result {

    env_logger::init();

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([480.0, 320.0])
            .with_decorations(false)
            .with_transparent(true)
            .with_resizable(true),
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
			cc.egui_ctx.set_pixels_per_point(1.25);
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

			let app: PergaminoApp = if let Some(storage) = cc.storage {
				eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default()
			} else {
				PergaminoApp::default()
			};

            // cc.egui_ctx.send_viewport_cmd(egui::ViewportCommand::InnerSize([400.0, 240.0].into()));

            Ok(Box::new(app))
        }),
    )
}
