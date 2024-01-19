use crate::launcher::commands::Command;
use crate::minecraft::downloader::download_minecraft;
use crate::minecraft::validate::is_valid_files;
use crate::{gui::GUI, minecraft::Minecraft};
use std::sync::{Arc, Mutex};
use std::{any::Any, sync::mpsc::channel};

pub struct LauncherController {}

impl LauncherController {
    pub fn new() -> Self {
        Self {}
    }

    pub fn run(&mut self) -> Result<(), Box<dyn Any + Send>> {
        let in_game = Arc::new(Mutex::new(false));
        let in_game_thread = Arc::clone(&in_game);

        let (logic_sender, logic_receiver) = channel::<Command>();
        let logic_sender_thread = logic_sender.clone();

        let (launcher_sender, launcher_receiver) = channel::<Command>();
        let launcher_sender_thread = launcher_sender.clone();

        let logic_thread = std::thread::spawn(move || {
            loop {
                match logic_receiver.recv().unwrap() {
                    Command::RUN => {
                        let mut in_game_guard = in_game_thread.lock().unwrap();
                        *in_game_guard = true;

                        let logic_sender = logic_sender_thread.clone();
                        let launcher_sender = launcher_sender_thread.clone();

                        std::thread::spawn(move || {
                            launcher_sender.send(Command::VALIDATE).unwrap();
                            if !is_valid_files() {
                                download_minecraft(launcher_sender.clone()).unwrap();
                            }

                            let minecraft = Minecraft::new().unwrap();
                            minecraft.run().unwrap().wait().unwrap();
                            logic_sender.send(Command::CONTINUE).unwrap();
                        });
                    },
                    Command::CONTINUE => {
                        let mut in_game_guard = in_game_thread.lock().unwrap();
                        *in_game_guard = false;
                    },
                    Command::EXIT => break,
                    _ => (),
                }
            }
        });


        let mut gui = GUI::new(logic_sender.clone(), in_game);
        gui.run(launcher_receiver);

        logic_thread.join().unwrap();

        Ok(())
    }
}
