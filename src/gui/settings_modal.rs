use egui::{Align, Context, Layout, TopBottomPanel, Vec2};
use serde_json::Value;

use crate::launcher::{config::AppConfig, locale::Locale};

pub struct SettingsModal {
    pub is_open: bool,
    locale: Value,
    curr_lang: String,
}

impl SettingsModal {
    pub fn new(locale: Value) -> Self {
        Self {
            is_open: false,
            curr_lang: locale["name"].as_str().unwrap().to_string(),
            locale,
        }
    }

    pub fn show(&mut self, ctx: &Context) {
        let langs = Locale::get_list();
        let screen_size = ctx.screen_rect().size();
        let mut is_open = self.is_open;

        egui::Window::new(self.locale["settings_title"].as_str().unwrap())
            .open(&mut self.is_open)
            .anchor(egui::Align2::CENTER_CENTER, Vec2::ZERO)
            .fixed_pos(egui::pos2(screen_size.x / 2., screen_size.y / 2.))
            .resizable(false)
            .collapsible(false)
            .show(ctx, |ui| {
                TopBottomPanel::top("settings_top").show_inside(ui, |ui| {
                    ui.horizontal(|ui| {
                        ui.label(self.locale["settings_language"].as_str().unwrap());
                        egui::ComboBox::from_id_source("language_selector")
                            .selected_text(format!("{}", self.curr_lang))
                            .show_ui(ui, |ui| {
                                for lang in langs {
                                    ui.selectable_value(&mut self.curr_lang, lang.clone(), &lang);
                                }
                            });
                    });
                    ui.with_layout(Layout::right_to_left(Align::Max), |ui| {
                        if ui
                            .button(self.locale["settings_save"].as_str().unwrap())
                            .clicked()
                        {
                            let config = AppConfig {
                                locale: self.curr_lang.clone(),
                            };
                            config.save();
                            is_open = false;
                        }
                    });
                });
            });

        if self.is_open {
            self.is_open = is_open;
        }
    }
}
