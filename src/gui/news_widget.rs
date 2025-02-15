use egui::{vec2, Color32, Layout, Pos2, Ui, Vec2, ViewportCommand};
use egui_commonmark::{CommonMarkCache, CommonMarkViewer};
use log::info;
use serde_json::Value;

use crate::api::news::News;

pub struct NewsWidget {
    news: Vec<News>,
    locale: Value,
    hover_id: String,
    is_modal_open: bool,
    show_news: Option<News>,
    cache: CommonMarkCache,
}

impl NewsWidget {
    pub fn new(locale: Value) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            news: News::load()?,
            locale,
            hover_id: "".to_string(),
            is_modal_open: false,
            show_news: None,
            cache: CommonMarkCache::default(),
        })
    }

    fn draw_news(&mut self, news: &mut News, ui: &mut Ui) {
        let resp = egui::Frame::none()
            .fill(if self.hover_id == news.id {
                Color32::DARK_GRAY
            } else {
                Color32::TRANSPARENT
            })
            .show(ui, |ui| {
                ui.spacing();
                ui.with_layout(Layout::left_to_right(egui::Align::Min), |ui| {
                    ui.vertical(|ui| {
                        ui.label(&news.title);
                    });
                });

                ui.with_layout(Layout::right_to_left(egui::Align::Min), |ui| {
                    ui.label(&news.created_at);
                });

                ui.separator();
            });

        if resp.response.interact(egui::Sense::click()).clicked() {
            info!("Showing modal");
            self.is_modal_open = true;
            self.show_news = Some(news.clone());
        }

        if resp.response.hovered() && self.hover_id != news.id {
            self.hover_id = news.id.clone();
        } else if !resp.response.hovered() && self.hover_id == news.id {
            self.hover_id = String::new();
        }
    }

    pub fn show_modal(&mut self, ui: &mut Ui) {
        let mut open = self.is_modal_open;

        egui::Window::new("News")
            .open(&mut open)
            .fixed_size(vec2(
                ui.ctx().screen_rect().width() - 4.,
                ui.ctx().screen_rect().height(),
            ))
            .fixed_pos(Pos2::new(-2., 32.))
            .collapsible(false)
            .title_bar(false)
            .min_height(ui.ctx().screen_rect().height())
            .show(ui.ctx(), |ui| {
                ui.horizontal(|ui| {
                    if ui.button("<").clicked() {
                        self.is_modal_open = false;
                    }
                    ui.label(&self.show_news.as_ref().unwrap().title);
                });
                ui.separator();
                CommonMarkViewer::new("viewer")
                    .max_image_width(Some(512))
                    .show_scrollable(ui, &mut self.cache, &self.show_news.as_ref().unwrap().body);
                let mut size = ui.available_size();
                size.y = size.y - 40.;
                ui.allocate_space(size);
            });

        if self.is_modal_open {
            self.is_modal_open = open;
        }
    }

    pub fn draw(&mut self, ui: &mut Ui) {
        egui::ScrollArea::vertical()
            .auto_shrink([false, false])
            .show(ui, |ui| {
                let mut news = self.news.clone();

                if self.news.len() != 0 {
                    for news in &mut news {
                        self.draw_news(news, ui);
                    }
                } else {
                    ui.label(self.locale["news_no_news"].as_str().unwrap());
                }
            });
    }
}
