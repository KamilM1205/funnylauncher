use std::process::exit;

use egui::{Color32, Layout, Style, Vec2, Visuals, WidgetText};
use log::{error, info};
use serde_json::Value;

use crate::{
    api::auth::{Auth, IS_ERROR, LP_ERROR},
    utils::constants::{CAPTION, REGISTRATION_URL},
};

use super::window_frame::{windowframe, WindowFrameData};

pub struct LoginScreen {
    login: String,
    password: String,
    wframe: WindowFrameData,
    is_error: bool,
    api: Auth,
    err_msg: String,
    locale: Value,
}

impl LoginScreen {
    pub fn new(locale: Value) -> Self {
        Self {
            login: String::new(),
            password: String::new(),
            wframe: WindowFrameData::new(locale.clone(), locale["login_title"].as_str().unwrap())
                .with_resizable(false),
            is_error: false,
            api: Default::default(),
            err_msg: String::new(),
            locale,
        }
    }
    pub fn run(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let auth = Auth::load();

        if !auth.is_err() {
            if auth.as_ref().unwrap().token != "" {
                info!("Token: {}", auth.unwrap().token);
                return Ok(());
            }
        } else {
            error!("{}", auth.err().unwrap().to_string())
        }

        let options = eframe::NativeOptions {
            viewport: egui::ViewportBuilder::default()
                .with_inner_size([320.0, 140.0])
                .with_decorations(false)
                .with_resizable(false),
            ..Default::default()
        };

        let locale = self.locale.clone();

        eframe::run_native(
            CAPTION,
            options,
            Box::new(|cc| {
                let style = Style {
                    visuals: Visuals::dark(),
                    ..Style::default()
                };
                cc.egui_ctx.set_style(style);

                Box::new(Self::new(locale))
            }),
        )?;

        Ok(())
    }
}

impl eframe::App for LoginScreen {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        windowframe::show(&mut self.wframe, ctx, |ui| {
            if ctx.input(|i| i.viewport().close_requested()) {
                info!("Exiting from login screen.");
                if self.api.token.is_empty() {
                    exit(0);
                }
            }

            if self.is_error {
                let frame = egui::containers::Frame {
                    fill: Color32::RED,
                    ..Default::default()
                };

                egui::TopBottomPanel::top("error_msg")
                    .frame(frame)
                    .show_inside(ui, |ui| {
                        ui.horizontal_centered(|ui| {
                            ui.vertical_centered(|ui| {
                                ui.label(
                                    WidgetText::from(self.err_msg.clone()).color(Color32::WHITE),
                                );
                            });
                        });
                    });

                ui.separator();
            }

            ui.horizontal(|ui| {
                ui.label(self.locale["login_login"].as_str().unwrap());
                ui.text_edit_singleline(&mut self.login);
            });

            ui.horizontal(|ui| {
                ui.label(self.locale["login_password"].as_str().unwrap());
                ui.text_edit_singleline(&mut self.password);
            });

            ui.separator();

            ui.vertical_centered(|ui| {
                ui.hyperlink_to(
                    self.locale["login_register"].as_str().unwrap(),
                    REGISTRATION_URL,
                );
            });

            ui.with_layout(Layout::right_to_left(egui::Align::Min), |ui| {
                if ui
                    .button(self.locale["login_submit"].as_str().unwrap())
                    .clicked()
                {
                    self.api.login = self.login.clone();
                    self.api.password = self.password.clone();
                    let resp = self.api.send();
                    if let Ok(()) = resp {
                        info!("Auth token requested");
                        if let Err(e) = self.api.save() {
                            let err = format!("Couldn't save token. Error: {e}");
                            error!("{}", err);
                            msgbox::create("Error", &err, msgbox::IconType::Error).unwrap_or_else(
                                |e| {
                                    error!("Couldn't show msgbox: {e}");
                                },
                            );
                        } else {
                            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                    } else if let Err(e) = resp {
                        if e.to_string() == LP_ERROR.to_string() {
                            self.err_msg = self.locale[LP_ERROR].as_str().unwrap().to_string();
                        } else {
                            self.err_msg = self.locale[IS_ERROR].as_str().unwrap().to_string();
                            error!("{}", self.err_msg);
                        }
                        self.is_error = true;
                    }

                    if self.is_error {
                        ctx.send_viewport_cmd(egui::ViewportCommand::InnerSize(Vec2::new(
                            320., 160.,
                        )));
                    }
                }
            });
        });
    }
}
