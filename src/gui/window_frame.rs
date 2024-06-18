use serde_json::Value;

#[derive(Default, Clone)]
pub struct WindowFrameData {
    title: String,
    closable: bool,
    resizable: bool,
    minimaizable: bool,
    movable: bool,
    show_account: bool,
    locale: Value,
    popup_visible: bool,
}

impl WindowFrameData {
    pub fn new(locale: Value, title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            closable: true,
            resizable: true,
            minimaizable: true,
            movable: true,
            show_account: false,
            locale,
            popup_visible: false,
        }
    }

    pub fn with_closable(mut self, closable: bool) -> Self {
        self.closable = closable;
        self
    }

    pub fn with_resizable(mut self, resizable: bool) -> Self {
        self.resizable = resizable;
        self
    }

    pub fn with_minimaizable(mut self, minimaizable: bool) -> Self {
        self.minimaizable = minimaizable;
        self
    }

    pub fn with_movable(mut self, movable: bool) -> Self {
        self.movable = movable;
        self
    }

    pub fn with_account(mut self, account: bool) -> Self {
        self.show_account = account;
        self
    }
}

pub mod windowframe {
    use std::process::exit;

    use egui::Image;
    use log::error;

    use crate::{
        api::{account::Account, auth::Auth},
        utils::relaunch::relaunch,
    };

    use super::WindowFrameData;

    pub fn show(
        data: &mut WindowFrameData,
        ctx: &egui::Context,
        add_contents: impl FnOnce(&mut egui::Ui),
    ) {
        let panel_frame = egui::Frame {
            fill: ctx.style().visuals.widgets.open.weak_bg_fill,
            stroke: ctx.style().visuals.widgets.noninteractive.fg_stroke,
            outer_margin: 0.5.into(),
            ..Default::default()
        };

        egui::CentralPanel::default()
            .frame(panel_frame)
            .show(ctx, |ui| {
                let app_rect = ui.max_rect();

                let title_bar_height = 32.0;
                let title_bar_rect = {
                    let mut rect = app_rect;
                    rect.max.y = rect.min.y + title_bar_height;
                    rect
                };
                titlebar(data, ui, title_bar_rect);

                let content_rect = {
                    let mut rect = app_rect;
                    rect.min.y = title_bar_rect.max.y;
                    rect
                }
                .shrink(4.0);

                let mut content_ui = ui.child_ui(content_rect, *ui.layout());
                add_contents(&mut content_ui);
            });
    }

    fn titlebar(data: &mut WindowFrameData, ui: &mut egui::Ui, titlebar_rect: egui::epaint::Rect) {
        use egui::*;

        let titlebar_response = ui.interact(titlebar_rect, Id::new("titlebar"), Sense::click());

        let painter = ui.painter();

        painter.text(
            titlebar_rect.center(),
            Align2::CENTER_CENTER,
            &data.title,
            FontId::proportional(20.0),
            ui.style().visuals.text_color(),
        );

        painter.line_segment(
            [
                titlebar_rect.left_bottom() + vec2(1.0, 0.0),
                titlebar_rect.right_bottom() + vec2(-1.0, 0.0),
            ],
            ui.visuals().widgets.noninteractive.bg_stroke,
        );

        ui.allocate_ui_at_rect(titlebar_rect, |ui| {
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                ui.spacing_mut().item_spacing.x = 8.0;
                ui.visuals_mut().button_frame = false;
                ui.add_space(8.0);

                titlebar_buttons(ui, data);
            });
        });

        if titlebar_response.double_clicked() && data.resizable {
            let is_maximized = ui.input(|i| i.viewport().maximized.unwrap_or(false));

            ui.ctx()
                .send_viewport_cmd(ViewportCommand::Maximized(!is_maximized));
        }

        if titlebar_response.is_pointer_button_down_on() && data.movable {
            ui.ctx().send_viewport_cmd(ViewportCommand::StartDrag);
        }
    }

    fn titlebar_buttons(ui: &mut egui::Ui, data: &mut WindowFrameData) {
        use egui::{Button, RichText};

        let button_height = 20.0;

        if data.closable {
            let close_responce = ui
                .add(Button::new(RichText::new("X").size(button_height)))
                .on_hover_text(data.locale["titlebar_close"].as_str().unwrap());

            if close_responce.clicked() {
                ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
            }
        }

        if data.resizable {
            let is_maximized = ui.input(|i| i.viewport().maximized.unwrap_or(false));

            let maximized_response = ui
                .add(Button::new(RichText::new("🗗").size(button_height)))
                .on_hover_text(data.locale["titlebar_maximize"].as_str().unwrap());

            if maximized_response.clicked() {
                ui.ctx()
                    .send_viewport_cmd(egui::ViewportCommand::Maximized(!is_maximized));
            }
        }

        if data.minimaizable {
            let minimized_response = ui
                .add(Button::new(RichText::new("-").size(button_height)))
                .on_hover_text(data.locale["titlebar_minimize"].as_str().unwrap());

            if minimized_response.clicked() {
                ui.ctx()
                    .send_viewport_cmd(egui::ViewportCommand::Minimized(true));
            }
        }

        if data.show_account {
            let image = Image::new(egui::include_image!("../../assets/user.png"))
                .max_size(egui::Vec2::new(button_height, button_height));
            let popup_id = ui.make_persistent_id("user_popup_id");

            let resp = ui.add(egui::ImageButton::new(image));
            if resp.hovered() {
                ui.memory_mut(|mem| mem.open_popup(popup_id));
            } 

            egui::popup::popup_below_widget(ui, popup_id, &resp, |ui| {
                ui.set_min_width(90.);
                if ui.button(data.locale["titlebar_logout"].as_str().unwrap()).clicked() {
                    Auth::remove_token().unwrap();
                    relaunch().unwrap_or_else(|e| {
                        error!("Couldn't relaunch. Error: {e}");
                    });
                    exit(0);
                }
            });
        }
    }
}
