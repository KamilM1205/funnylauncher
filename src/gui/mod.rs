use crate::launcher::commands::Command;
use egui::{Style, Visuals};
use log::debug;
use std::sync::{
    mpsc::{Receiver, Sender},
    Arc, Mutex,
};

use crate::utils::constants::CAPTION;

use self::main_screen::MainScreen;

pub mod main_screen;
pub mod message_screen;
pub mod titlebar;
pub mod update_screen;

pub struct GUI {
    logic_sender: Sender<Command>,
    in_game: Arc<Mutex<bool>>,
}

impl GUI {
    pub fn new(logic_sender: Sender<Command>, in_game: Arc<Mutex<bool>>) -> Self {
        Self {
            logic_sender,
            in_game,
        }
    }

    pub fn run(&mut self, launcher_receiver: Receiver<Command>) -> Result<(), eframe::Error> {
        let options = eframe::NativeOptions {
            viewport: egui::ViewportBuilder::default().with_inner_size([640.0, 480.0]),
            ..Default::default()
        };

        let logic_sender = self.logic_sender.clone();
        let in_game = self.in_game.clone();

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

                Box::new(MainScreen::new(logic_sender, in_game, launcher_receiver))
            }),
        )?;

        Ok(())
    }
}
