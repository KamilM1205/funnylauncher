use std::sync::{
    mpsc::{Receiver, Sender},
    Arc, Mutex,
};

use egui::ProgressBar;
use log::{debug, error};

use crate::launcher::commands::Command;

use super::message_screen::MsgBoxScreen;

const MAINSCREEN: &str = "MAINSCREEN";

#[derive(Eq, PartialEq)]
enum State {
    Updating,
    Idle,
}

pub struct MainScreen {
    logic_sender: Sender<Command>,
    launcher_receiver: Receiver<Command>,
    in_game: Arc<Mutex<bool>>,
    state: State,
    text: String,
    progress: f32,
    error_msg: MsgBoxScreen,
}

impl MainScreen {
    pub fn new(
        logic_sender: Sender<Command>,
        in_game: Arc<Mutex<bool>>,
        launcher_receiver: Receiver<Command>,
    ) -> Self {
        Self {
            logic_sender,
            in_game,
            launcher_receiver,
            state: State::Idle,
            text: String::from("Готов к запуску"),
            progress: 1.0,
            error_msg: MsgBoxScreen::default(),
        }
    }

    fn handle_commands(&mut self) {
        let recv = self.launcher_receiver.try_recv();
        if recv.is_ok() {
            let r = match recv {
                Ok(r) => r,
                Err(_) => {
                    error!(target: MAINSCREEN, "Fatal error: Couldn't receive command from control thread.");
                    match msgbox::create(
                        "Fatal error",
                        "Couldn't reveive command from control thread.",
                        msgbox::IconType::Error,
                    ) {
                        Ok(()) => (),
                        Err(e) => {
                            error!(target: MAINSCREEN, "Couldn't show msgbox: {e}");
                            panic!();
                        }
                    };
                    panic!();
                }
            };

            match r {
                Command::RUN => {
                    debug!(target: MAINSCREEN, "RUN command.");

                    self.state = State::Idle;
                    self.text = String::from("В игре")
                }
                Command::CONTINUE => {
                    debug!(target: MAINSCREEN, "CONTINUE command.");

                    self.state = State::Idle;
                    self.text = String::from("Готов к запуску")
                }
                Command::VALIDATE => {
                    debug!(target: MAINSCREEN, "VALIDATE command.");

                    self.state = State::Updating;
                    self.text = String::from("Проверка файлов")
                }
                Command::DOWNLOAD((downloaded, size)) => {
                    debug!(target: MAINSCREEN, "DOWNLOAD command.");

                    self.state = State::Updating;
                    self.text = format!("{}Mb/{}Mb", downloaded / (1024 * 2), size / (1024 * 2));
                    self.progress = downloaded as f32 / size as f32;
                }
                Command::UNZIPING => {
                    debug!(target: MAINSCREEN, "UNZIPING command.");

                    self.state = State::Updating;
                    self.text = String::from("Распаковка игры...")
                }
                Command::PLAY => {
                    debug!(target: MAINSCREEN, "PLAY command.");

                    self.state = State::Idle;
                    self.text = String::from("В игре")
                }
                Command::ERROR(e) => {
                    debug!(target: MAINSCREEN, "ERROR command.");

                    self.state = State::Idle;
                    self.error_msg = MsgBoxScreen::error("Error", e)
                }
                _ => (),
            }
        }
    }
}

impl eframe::App for MainScreen {
    fn update(&mut self, ctx: &egui::Context, _: &mut eframe::Frame) {
        if ctx.input(|i| i.viewport().close_requested()) {
            match self.logic_sender.send(Command::EXIT) {
                Ok(_) => (),
                Err(_) => {
                    error!(target: MAINSCREEN, "Error while send \"Exit\" command to control thread.")
                }
            };
        }

        self.handle_commands();

        egui::TopBottomPanel::bottom("bottom").show(ctx, |ui| {
            ui.with_layout(egui::Layout::right_to_left(egui::Align::LEFT), |ui| {
                let in_game_guard = match self.in_game.lock() {
                    Ok(g) => g,
                    Err(e) => {
                        error!(target: MAINSCREEN, "Couldn't lock game mutex: {e}");
                        match msgbox::create(
                            "Fatal error",
                            "Couldn't lock game mutex",
                            msgbox::IconType::Error,
                        ) {
                            Ok(_) => (),
                            Err(e) => error!(target: MAINSCREEN, "Couldn't show msgbox: {e}"),
                        };
                        panic!("{e}");
                    }
                };

                if ui
                    .add_enabled(!*in_game_guard, egui::Button::new("Play"))
                    .clicked()
                {
                    match self.logic_sender.send(Command::RUN) {
                        Ok(()) => (),
                        Err(_) => {
                            error!(target: MAINSCREEN, "Couldn't send \"RUN\" command.");
                            match msgbox::create(
                                "Error",
                                "Couldn't send command to run game.",
                                msgbox::IconType::Error,
                            ) {
                                Ok(_) => (),
                                Err(e) => error!(target: MAINSCREEN, "Couldn't show msgbox: {e}"),
                            };
                        }
                    };
                }

                if self.state == State::Idle {
                    ui.with_layout(egui::Layout::left_to_right(egui::Align::LEFT), |ui| {
                        ui.label(&self.text);
                    });
                } else {
                    let progress = ProgressBar::new(self.progress)
                        .show_percentage()
                        .text(&self.text);
                    ui.add(progress);
                }
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("Some news will be here");
        });

        // Modal messages
        self.error_msg.show(ctx);

        ctx.request_repaint();
    }
}
