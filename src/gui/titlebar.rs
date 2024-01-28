use egui::Layout;

#[derive(Default)]
pub struct TitleBar {
    title: String,
    closable: bool,
    resizable: bool,
    minimaizable: bool,
    is_fullscreen: bool,
}

impl TitleBar {
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            closable: true,
            resizable: true,
            minimaizable: true,
            is_fullscreen: false,
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

    pub fn show(&mut self, ctx: &egui::Context) {
        egui::TopBottomPanel::top("titlebar_panel").show(ctx, |ui| {
            ui.centered_and_justified(|ui| {
                ui.horizontal_centered(|ui| {
                    ui.label(&self.title);

                    ui.with_layout(Layout::left_to_right(egui::Align::Max), |ui| {
                        if self.minimaizable {
                            if ui.button("_").clicked() {
                                ctx.send_viewport_cmd(egui::ViewportCommand::Minimized(true));
                            }
                        }

                        if self.resizable {
                            if ui.button("[]").clicked() {
                                ctx.send_viewport_cmd(egui::ViewportCommand::Fullscreen(
                                    !self.is_fullscreen,
                                ));
                                self.is_fullscreen = !self.is_fullscreen;
                            }
                        }

                        if self.closable {
                            if ui.button("X").clicked() {
                                ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                            }
                        }
                    });
                });
            });
        });
    }
}
