use std::sync::{
    mpsc::{Receiver, Sender},
    Arc, Mutex,
};

use egui::{Image, ProgressBar};
use log::{debug, error};
use serde_json::Value;

use crate::launcher::commands::Command;

use super::{
    message_screen::MsgBoxScreen,
    settings_modal::SettingsModal,
    window_frame::{windowframe, WindowFrameData},
};

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
    settings_modal: SettingsModal,
    state: State,
    text: String,
    progress: f32,
    error_msg: MsgBoxScreen,
    wframe: WindowFrameData,
    locale: Value,
}

impl MainScreen {
    pub fn new(
        locale: Value,
        logic_sender: Sender<Command>,
        in_game: Arc<Mutex<bool>>,
        launcher_receiver: Receiver<Command>,
    ) -> Self {
        Self {
            logic_sender,
            in_game,
            launcher_receiver,
            state: State::Idle,
            text: locale["main_ready"].as_str().unwrap().to_owned(),
            settings_modal: SettingsModal::new(locale.clone()),
            progress: 1.0,
            error_msg: MsgBoxScreen::default(),
            wframe: WindowFrameData::new(locale.clone(), "FunnyLauncher"),
            locale,
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
                    self.text = self.locale["main_run"].as_str().unwrap().to_owned()
                }
                Command::CONTINUE => {
                    debug!(target: MAINSCREEN, "CONTINUE command.");

                    self.state = State::Idle;
                    self.text = self.locale["main_ready"].as_str().unwrap().to_owned()
                }
                Command::VALIDATE => {
                    debug!(target: MAINSCREEN, "VALIDATE command.");

                    self.state = State::Updating;
                    self.text = self.locale["main_check"].as_str().unwrap().to_owned()
                }
                Command::DOWNLOAD((downloaded, size)) => {
                    debug!(target: MAINSCREEN, "DOWNLOAD command.");

                    self.state = State::Updating;
                    self.text = format!(
                        "{} {}Mb/{}Mb",
                        self.locale["main_download"].as_str().unwrap(),
                        downloaded / (1024 * 2),
                        size / (1024 * 2)
                    );
                    self.progress = downloaded as f32 / size as f32;
                }
                Command::UNZIPING => {
                    debug!(target: MAINSCREEN, "UNZIPING command.");

                    self.state = State::Updating;
                    self.text = self.locale["main_unpack"].as_str().unwrap().to_owned()
                }
                Command::PLAY => {
                    debug!(target: MAINSCREEN, "PLAY command.");

                    self.state = State::Idle;
                    self.text = self.locale["main_run"].to_string().to_owned()
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
        windowframe::show(&self.wframe.clone(), ctx, |ui| {
            if ui.ctx().input(|i| i.viewport().close_requested()) {
                match self.logic_sender.send(Command::EXIT) {
                    Ok(_) => (),
                    Err(_) => {
                        error!(target: MAINSCREEN, "Error while send \"Exit\" command to control thread.")
                    }
                };
            }

            self.handle_commands();

            egui::TopBottomPanel::bottom("bottom").show_inside(ui, |ui| {
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

                    let image = Image::new(egui::include_image!("../../assets/setting.png"))
                        .max_size(egui::Vec2::new(64., 64.));
                    if ui.add(egui::ImageButton::new(image)).clicked() {
                        self.settings_modal.is_open = true;
                    }

                    if ui
                        .add_enabled(
                            !*in_game_guard,
                            egui::Button::new(self.locale["main_btn_play"].as_str().unwrap()),
                        )
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
                                    Err(e) => {
                                        error!(target: MAINSCREEN, "Couldn't show msgbox: {e}")
                                    }
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

            egui::CentralPanel::default().show_inside(ui, |ui| {
                ui.label("Some news will be here");
            });

            // Modal messages
            self.error_msg.show(ui.ctx());
            self.settings_modal.show(ctx);

            ui.ctx().request_repaint();
        });
    }
}
