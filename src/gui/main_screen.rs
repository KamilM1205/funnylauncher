use std::sync::{mpsc::{Sender, Receiver}, Arc, Mutex};

use egui::ProgressBar;

use crate::launcher::commands::Command;

pub struct MainScreen {
    logic_sender: Sender<Command>,
    launcher_receiver: Receiver<Command>,
    in_game: Arc<Mutex<bool>>,
    text: String,
    progress: f32
}

impl MainScreen {
    pub fn new(logic_sender: Sender<Command>, in_game: Arc<Mutex<bool>>, launcher_receiver: Receiver<Command>) -> Self {
        Self {
            logic_sender,
            in_game,
            launcher_receiver,
            text: String::new(),
            progress: 1.0,
        }
    }
}

impl eframe::App for MainScreen {
    fn update(&mut self, ctx: &egui::Context, _: &mut eframe::Frame) {
        if ctx.input(|i| i.viewport().close_requested()) {
            self.logic_sender.send(Command::EXIT).unwrap();
        }

        let recv = self.launcher_receiver.try_recv();
        if recv.is_ok() {
            match recv.unwrap() {
                Command::RUN => self.text = String::from("В игре"),
                Command::CONTINUE => self.text = String::from("Вы используете последнюю версию"),
                Command::VALIDATE => self.text = String::from("Проверка файлов"),
                Command::DOWNLOAD((downloaded, size)) => {
                    self.text = format!("{}Kb/{}Kb", downloaded, size);
                    self.progress = downloaded as f32/size as f32;
                }
                Command::UNZIPING => self.text = String::from("Распаковка игры..."),
                Command::PLAY => self.text = String::from("В игре"),
                _ => (),
            }
        }

        egui::TopBottomPanel::bottom("bottom").show(ctx, |ui| {
            ui.with_layout(egui::Layout::right_to_left(egui::Align::LEFT), |ui| {
                let in_game_guard = self.in_game.lock().unwrap();

                if ui
                    .add_enabled(!*in_game_guard, egui::Button::new("Play"))
                    .clicked()
                {
                    self.logic_sender.send(Command::RUN).unwrap();
                }

                let progress = ProgressBar::new(self.progress)
                    .show_percentage()
                    .text(&self.text);
                ui.add(progress);
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("Some news will be here");
        });

        ctx.request_repaint();
    }
}
