use std::path::PathBuf;

use crate::{ui::{self, AppState}};
use eframe::egui;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Default)]
pub struct AppConfig {
	pub last_project_path: Option<PathBuf>,
}

#[derive(Deserialize, Serialize)]
#[serde(default)]
pub struct PergaminoApp {
    #[serde(skip)]
    state: AppState,

	config: AppConfig
}

impl Default for PergaminoApp {
    fn default() -> Self {
        Self {
            state: AppState::Welcome,
			config: AppConfig::default()
        }
    }
}

impl eframe::App for PergaminoApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {

        let potential_new_state = match &mut self.state {
            AppState::Welcome => {
                ui::welcome::show(ctx, &self.config.last_project_path)
            },

            AppState::NamingProject { temp_name } => {
                ui::welcome::show(ctx, &self.config.last_project_path);
                ui::create_project::show(ctx, temp_name)
            },

            AppState::Editor { project_name, file_path, snarl, invoker } => {
                ui::editor::show(ctx, project_name, file_path, snarl, invoker)
            }

        };


        if let Some(new_state) = potential_new_state {
            if new_state != self.state {
                match &new_state {
					AppState::Welcome => ui::welcome::start(ctx),
                    AppState::NamingProject { .. } => ui::create_project::start(ctx),
					AppState::Editor { .. } => ui::editor::start(ctx),
				}

				self.state = new_state;
            }

			// si estamos en el editor y tenemos un archivo guardado, actualizamos la config global
			if let AppState::Editor { file_path: Some(path), .. } = &self.state {
				if self.config.last_project_path.as_ref() != Some(path) {
					self.config.last_project_path = Some(path.clone())
				}
			}
        }

    }

	fn save(&mut self, storage: &mut dyn eframe::Storage) {
		eframe::set_value(storage, eframe::APP_KEY, self);
	}

	fn persist_egui_memory(&self) -> bool {
		false
	}

    fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {
        // executes just before closing the app
    }
}