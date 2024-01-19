use crate::launcher::commands::Command;
use egui::{Style, Visuals};
use std::sync::{mpsc::{Sender, Receiver}, Arc, Mutex};

use crate::utils::constants::CAPTION;

use self::main_screen::MainScreen;

pub mod error_screen;
pub mod main_screen;
pub mod update_screen;

pub struct GUI {
    logic_sender: Sender<Command>,
    in_game: Arc<Mutex<bool>>,
}

impl GUI {
    pub fn new(logic_sender: Sender<Command>, in_game: Arc<Mutex<bool>>, ) -> Self {
        Self { logic_sender, in_game }
    }

    pub fn run(&mut self, launcher_receiver: Receiver<Command>) {
        let options = eframe::NativeOptions {
            viewport: egui::ViewportBuilder::default().with_inner_size([640.0, 480.0]),
            ..Default::default()
        };

        let logic_sender = self.logic_sender.clone();
        let in_game = self.in_game.clone();
        
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
        )
        .unwrap();
    }
}
