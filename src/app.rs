use crate::ui::{self, AppState};
use eframe::egui;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
#[serde(default)]
pub struct PergaminoApp {
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

        let potential_new_state = match &mut self.state {
            AppState::Welcome => {
                ui::welcome::show(ctx)
            },

            AppState::NamingProject { temp_name } => {
                ui::welcome::show(ctx);
                ui::create_project::show(ctx, temp_name)
            },

            AppState::Editor { project_name, snarl } => {
                ui::editor::show(ctx, project_name, snarl)
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