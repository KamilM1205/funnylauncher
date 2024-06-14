use crate::launcher::commands::Command;
use egui::{Style, Visuals};
use log::debug;
use serde_json::Value;
use std::sync::{
    mpsc::{Receiver, Sender},
    Arc, Mutex,
};

use crate::utils::constants::CAPTION;

use self::main_screen::MainScreen;

pub mod login_screen;
pub mod main_screen;
pub mod message_screen;
pub mod settings_modal;
pub mod update_screen;
pub mod window_frame;

pub struct GUI {
    locale: Value,
    logic_sender: Sender<Command>,
    in_game: Arc<Mutex<bool>>,
}

impl GUI {
    pub fn new(locale: Value, logic_sender: Sender<Command>, in_game: Arc<Mutex<bool>>) -> Self {
        Self {
            locale,
            logic_sender,
            in_game,
        }
    }

    pub fn run(&mut self, launcher_receiver: Receiver<Command>) -> Result<(), eframe::Error> {
        let options = eframe::NativeOptions {
            viewport: egui::ViewportBuilder::default()
                .with_inner_size([640.0, 480.0])
                .with_decorations(false),
            ..Default::default()
        };

        let logic_sender = self.logic_sender.clone();
        let in_game = self.in_game.clone();
        let locale = self.locale.clone();

        debug!("Starting main screen.");

        eframe::run_native(
            CAPTION,
            options,
            Box::new(|cc| {
                let style = Style {
                    visuals: Visuals::dark(),
                    ..Style::default()
                };
                cc.egui_ctx.set_style(style);

                egui_extras::install_image_loaders(&cc.egui_ctx);

                Box::new(MainScreen::new(
                    locale,
                    logic_sender,
                    in_game,
                    launcher_receiver,
                ))
            }),
        )?;

        Ok(())
    }
}
