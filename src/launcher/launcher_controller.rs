use egui::TextBuffer;
use log::{debug, error};
use serde_json::Value;

use crate::api::account::Account;
use crate::api::auth::Auth;
use crate::launcher::commands::Command;
use crate::minecraft;
use crate::minecraft::downloader::{self, download_minecraft};
use crate::minecraft::validate::{self, is_valid_files};
use crate::utils::relaunch::relaunch;
use crate::{gui::GUI, minecraft::Minecraft};
use std::process::{exit, ExitStatus};
use std::sync::mpsc::channel;
use std::sync::{Arc, Mutex};

const CONTROLLER: &str = "LAUNCHERCONTROLLER";

pub struct LauncherController {
    locale: Value,
    pub account: Account,
}

impl LauncherController {
    pub fn new(locale: Value) -> Result<Self, Box<dyn std::error::Error>> {
        let account = Account::new()?;
        Ok(Self { locale, account })
    }

    pub fn run(&mut self) -> Result<(), Box<dyn std::error::Error + Send>> {
        if let Err(e) = self.account.send_online() {
            error!("Couldn't send online status. Error: {e}");
            Auth::remove_token().unwrap_or_else(|e| {
                error!("Couldn't remove token. Error: {e}");
            });

            relaunch().unwrap_or_else(|e| {
                error!("Couldn't relaunch launcher. Error: {e}");
                exit(-1);
            });

            return Ok(());
        }

        let in_game = Arc::new(Mutex::new(false));
        let in_game_thread = Arc::clone(&in_game);

        let (logic_sender, logic_receiver) = channel::<Command>();
        let (launcher_sender, launcher_receiver) = channel::<Command>();

        let logic_sender_thread = logic_sender.clone();

        let launcher_sender_thread = launcher_sender.clone();

        let logic_thread = std::thread::spawn(move || loop {
            match logic_receiver.recv().unwrap() {
                Command::RUN => {
                    debug!(target: CONTROLLER, "RUN command.");

                    let mut in_game_guard = in_game_thread.lock().unwrap();
                    *in_game_guard = true;

                    let logic_sender = logic_sender_thread.clone();
                    let launcher_sender = launcher_sender_thread.clone();

                    std::thread::spawn(move || {
                        launcher_sender.send(Command::VALIDATE).unwrap();
                        'run: {
                            let is_valid = match is_valid_files() {
                                Ok(v) => v,
                                Err(e) => {
                                    error!(target: validate::VALIDATOR, "{e}");
                                    launcher_sender.send(Command::ERROR("Couldn't connect to update server. Check your internet connection.".to_string()))
                                        .unwrap_or_else(|_| {
                                            error!(target: CONTROLLER, "Error while sending \"ERROR\" command.");
                                            panic!();
                                        });
                                    break 'run;
                                }
                            };
                            if !is_valid {
                                match download_minecraft(launcher_sender.clone()) {
                                    Ok(_) => (),
                                    Err(e) => {
                                        error!(target: downloader::DOWNLOAD, "Error while downloading minecraft. Error: {e}");
                                        launcher_sender.send(Command::ERROR(format!("Error while connecting to update server: {e}"))).unwrap_or_else(|_| {
                                        error!(target: CONTROLLER, "Error while sending \"ERROR\" command.");
                                        panic!();
                                    });
                                        break 'run;
                                    }
                                };
                            }

                            let minecraft = match Minecraft::new() {
                                Ok(m) => m,
                                Err(e) => {
                                    error!(target: minecraft::MINECRAFT, "Error while initializing minecraft. Error: {e}");
                                    launcher_sender.send(Command::ERROR(format!("Error while initializing minecraft: {e}"))).unwrap_or_else(|_| {
                                    error!(target: CONTROLLER, "Error while sending \"ERROR\" command.");
                                    panic!();
                                });
                                    break 'run;
                                }
                            };
                            match minecraft.run() {
                            Ok(c) => c,
                            Err(e) => {
                                error!(target: minecraft::MINECRAFT, "Error while launching minecraft. Error: {e}");
                                launcher_sender.send(Command::ERROR(format!("Error while launching minecraft: {e}"))).unwrap_or_else(|_| {
                                    error!(target: CONTROLLER, "Error while sending \"ERROR\" command.");
                                    panic!();
                                });
                                break 'run;
                            }
                        }.wait().unwrap_or_else(|e| {
                                error!(target: CONTROLLER, "Error while waiting closing game: {e}");
                                launcher_sender.send(Command::ERROR(format!("Error while waitng closing game: {e}"))).unwrap_or_else(|_| {
                                    error!(target: CONTROLLER, "Error while sending \"ERROR\" command.");
                                    panic!();
                                });
                                ExitStatus::default()
                            });
                        }

                        logic_sender.send(Command::CONTINUE).unwrap_or_else(|_| {
                            error!(target: CONTROLLER, "Error while sending \"CONTINUE\" command.");
                        });
                        launcher_sender.send(Command::CONTINUE).unwrap_or_else(|_| {
                            error!(target: CONTROLLER, "Error while sending \"CONTINUE\" command.");
                        });
                    });
                }
                Command::CONTINUE => {
                    debug!(target: CONTROLLER, "CONTINUE command.");

                    let mut in_game_guard = match in_game_thread.lock() {
                        Ok(g) => g,
                        Err(e) => {
                            error!(target: CONTROLLER, "Error while locking game mutex. Error: {e}");
                            launcher_sender.send(Command::ERROR(format!("Error while locking game mutex: {e}"))).unwrap_or_else(|_| {
                                    error!(target: CONTROLLER, "Error while sending \"ERROR\" command.");
                                    panic!();
                                });
                            return;
                        }
                    };

                    *in_game_guard = false;
                }
                Command::EXIT => {
                    debug!(target: CONTROLLER, "EXIT command.");

                    break;
                }
                _ => (),
            }
        });

        let mut gui = GUI::new(self.locale.clone(), logic_sender.clone(), in_game);
        match gui.run(launcher_receiver) {
            Ok(_) => (),
            Err(e) => {
                error!(target: CONTROLLER, "{e}");
                msgbox::create(
                    "Fatal error",
                    &format!("Error while running gui: {e}"),
                    msgbox::IconType::Error,
                )
                .unwrap_or_else(|_| {
                    error!(target: CONTROLLER, "Couldn't show msgbox: {e}");
                    panic!();
                })
            }
        };

        logic_thread.join().unwrap();

        if let Err(e) = self.account.send_offline() {
            error!("Couldn't send online status. Error: {e}");
            Auth::remove_token().unwrap_or_else(|e| {
                error!("Couldn't remove token. Error: {e}");
            });

            relaunch().unwrap_or_else(|e| {
                error!("Couldn't relaunch launcher. Error: {e}");
                exit(-1);
            });

            return Ok(());
        }

        Ok(())
    }
}
