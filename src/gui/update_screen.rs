use std::{
    any::Any,
    sync::mpsc::{channel, Receiver},
};

use egui::{CentralPanel, ProgressBar, Style, Ui, Visuals};
use log::{debug, error};
use serde_json::Value;

use crate::{
    launcher::launcher_update::{download_launcher, need_update, Command, UpdateData},
    utils::constants::CAPTION,
};

use super::window_frame::{windowframe, WindowFrameData};

const UPDATE: &str = "UPDATESCREEN";

#[derive(Default)]
pub struct UpdateScreen {
    data_receiver: Option<Receiver<Command>>,
    wframe: WindowFrameData,
    locale: Value,
}

impl UpdateScreen {
    pub fn new(locale: Value, data_receiver: Receiver<Command>) -> Self {
        Self {
            data_receiver: Some(data_receiver),
            wframe: WindowFrameData::new(locale.clone(), locale["update_title"].as_str().unwrap())
                .with_closable(false)
                .with_resizable(false)
                .with_minimaizable(false)
                .with_movable(false),
            locale,
        }
    }

    pub fn run(&mut self, locale: Value) -> Result<(), Box<dyn Any + Send>> {
        let (data_sender, data_receiver) = channel::<Command>();
        let data_sender_thread = data_sender.clone();

        let update_thread = std::thread::spawn(move || {
            // Update
            let nu = need_update().unwrap_or_else(|e| {
                error!("{}", e);
                false
            });

            if !nu {
                data_sender_thread.send(Command::Completed).unwrap_or_else(|_| {
                    error!(target: UPDATE, "Error while send \"Completed\" command to control thread.");
                    msgbox::create("Fatal error", &format!("Error while send \"Completed\" command to control thread."), msgbox::IconType::Error)
                    .unwrap_or_else(|e| {
                            error!(target: UPDATE, "Couldn't show msgbox: {e}");
                            panic!();
                        });
                });
                return;
            }

            download_launcher(data_sender_thread).unwrap_or_else(|e| {
                error!(target: UPDATE, "{e}");
                msgbox::create("Error", &e.to_string(), msgbox::IconType::Error).unwrap_or_else(
                    |e| {
                        error!(target: UPDATE, "Couldn't show msgbox: {e}");
                        panic!();
                    },
                );
            });
        });

        // GUI
        let options = eframe::NativeOptions {
            viewport: egui::ViewportBuilder::default()
                .with_inner_size([640.0, 70.0])
                .with_resizable(false)
                .with_decorations(false),
            ..Default::default()
        };

        debug!("Starting eframe window.");

        eframe::run_native(
            CAPTION,
            options,
            Box::new(|cc| {
                let style = Style {
                    visuals: Visuals::dark(),
                    ..Style::default()
                };
                cc.egui_ctx.set_style(style);

                egui::ViewportCommand::center_on_screen(&cc.egui_ctx);

                Box::new(Self::new(locale, data_receiver))
            }),
        )
        .unwrap();

        match update_thread.join() {
            Ok(_) => Ok(()),
            Err(e) => {
                error!("Error while joining download thread.");
                Err(e)
            }
        }?;

        Ok(())
    }

    fn draw_contents(&self, ui: &mut Ui) {
        let mut data = UpdateData {
            downloaded: 0,
            size: 100,
        };

        let mut is_checking = true;

        if self.data_receiver.is_some() {
            let recv = self.data_receiver.as_ref().unwrap().try_recv();
            if recv.is_ok() {
                match recv.unwrap() {
                    Command::Data(data_) => {
                        is_checking = false;
                        data = data_
                    }
                    Command::Completed => ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close),
                    Command::Abort => ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close),
                }
            }
        }

        CentralPanel::default().show_inside(ui, |ui| {
            if is_checking {
                ui.with_layout(
                    egui::Layout::centered_and_justified(egui::Direction::LeftToRight),
                    |ui| {
                        ui.label(self.locale["update_label_check"].as_str().unwrap());
                    },
                );
            } else {
                ui.label(self.locale["update_label_update"].as_str().unwrap());
                let progress =
                    ProgressBar::new(data.downloaded as f32 / data.size as f32).text(format!(
                        "{}Mb/{}Mb",
                        data.downloaded / (1024 * 2),
                        data.size / (1024 * 2)
                    ));
                ui.add(progress);
            }
        });

        ui.ctx().request_repaint();
    }
}

impl eframe::App for UpdateScreen {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        windowframe::show(&mut self.wframe.clone(), ctx, |ui| self.draw_contents(ui));
    }
}
