use std::{any::Any, sync::mpsc::{channel, Receiver}};

use egui::{CentralPanel, ProgressBar, Visuals, Style};

use crate::{utils::constants::CAPTION, launcher::launcher_update::{need_update, UpdateData, Command, download_launcher}};

#[derive(Default)]
pub struct UpdateScreen {
	data_receiver: Option<Receiver<Command>>
}

impl UpdateScreen {
    pub fn new(data_receiver: Receiver<Command>) -> Self {
        Self {
        	data_receiver: Some(data_receiver)
        }
    }

    pub fn run(&mut self) -> Result<(), Box<dyn Any + Send>> {
    	// Update
    	if !need_update().unwrap_or(false) {
    		return Ok(())
    	}

    	let (data_sender, data_receiver) = channel::<Command>();

 		let download_thread = std::thread::spawn(move || {
 			download_launcher(data_sender).unwrap();
 		});

    	// GUI
        let options = eframe::NativeOptions {
            viewport: egui::ViewportBuilder::default().with_inner_size([640.0, 50.0]).with_resizable(false),
            ..Default::default()
        };

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

                Box::new(Self::new(data_receiver))
            }),
        ).unwrap();

        download_thread.join()?;

        Ok(())
    }
}

impl eframe::App for UpdateScreen {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
    	let mut data = UpdateData { downloaded: 0, size: 100 };

    	if self.data_receiver.is_some() {
            let recv = self.data_receiver.as_mut().unwrap().try_recv();
            if recv.is_ok() {
        		match recv.unwrap() {
                    Command::Data(data_) => data = data_,
                    Command::Completed => ctx.send_viewport_cmd(egui::ViewportCommand::Close),
                }
            }
    	}

        CentralPanel::default().show(ctx, |ui| {
            ui.label("Обновление");

            let progress = ProgressBar::new(data.downloaded as f32 / data.size as f32).text(format!("{}kb/{}kb", data.downloaded/1024, data.size/1024));
            ui.add(progress);
        });

        ctx.request_repaint();
    }
}
