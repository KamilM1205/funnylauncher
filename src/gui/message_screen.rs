#[derive(Clone, Copy, Default)]
pub enum MsgLevel {
    #[default]
    Info,
    Warn,
    Error,
}

#[derive(Default)]
pub struct MsgBoxScreen {
    title: String,
    msg: String,
    msg_level: MsgLevel,
    visible: bool,
}

impl MsgBoxScreen {
    pub fn new(title: impl Into<String>, msg: impl Into<String>, msg_level: MsgLevel) -> Self {
        Self {
            title: title.into(),
            msg: msg.into(),
            msg_level,
            visible: true,
        }
    }

    pub fn info(title: impl Into<String>, msg: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            msg: msg.into(),
            msg_level: MsgLevel::Info,
            visible: true,
        }
    }

    pub fn warn(title: impl Into<String>, msg: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            msg: msg.into(),
            msg_level: MsgLevel::Warn,
            visible: true,
        }
    }

    pub fn error(title: impl Into<String>, msg: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            msg: msg.into(),
            msg_level: MsgLevel::Error,
            visible: true,
        }
    }

    pub fn show(&mut self, ctx: &egui::Context) {
        let mut visible = self.visible;
        egui::Window::new(&self.title)
            .open(&mut visible)
            .collapsible(false)
            .fixed_size([
                ctx.screen_rect().width() / 2.0,
                ctx.screen_rect().height() / 3.0,
            ])
            .anchor(egui::Align2::CENTER_CENTER, egui::Vec2::ZERO)
            .show(ctx, |ui| {
                egui::TopBottomPanel::bottom("msg_panel").show_inside(ui, |ui| {
                    match self.msg_level {
                        MsgLevel::Info => {
                            ui.with_layout(egui::Layout::right_to_left(egui::Align::RIGHT), |ui| {
                                if ui.button("Ok").clicked() {
                                    self.visible = false;
                                }
                            });
                        }
                        MsgLevel::Warn => {
                            ui.with_layout(egui::Layout::right_to_left(egui::Align::RIGHT), |ui| {
                                if ui.button("Ok").clicked() {
                                    self.visible = false;
                                }
                            });
                        }
                        MsgLevel::Error => {
                            ui.with_layout(egui::Layout::right_to_left(egui::Align::RIGHT), |ui| {
                                if ui.button("Ok").clicked() {
                                    self.visible = false;
                                }
                            });
                        }
                    }
                });

                egui::CentralPanel::default()
                    .show_inside(ui, |ui| ui.add(egui::Label::new(&self.msg).wrap(true)));
            });
        if self.visible && !visible {
            self.visible = false;
        }
    }
}
