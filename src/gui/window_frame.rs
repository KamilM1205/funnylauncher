use egui::Layout;

#[derive(Default)]
pub struct WindowFrame {
    title: String,
    closable: bool,
    resizable: bool,
    minimaizable: bool,
}

impl WindowFrame {
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            closable: true,
            resizable: true,
            minimaizable: true,
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

    pub fn show(&self, ctx: &egui::Context, add_contents: impl FnOnce(&mut egui::Ui)) {
        let panel_frame = egui::Frame {
            fill: ctx.style().visuals.window_fill(),
            stroke: ctx.style().visuals.widgets.noninteractive.fg_stroke,
            outer_margin: 0.5.into(),
            ..Default::default()
        };

        egui::CentralPanel::default().frame(panel_frame).show(ctx, |ui| {
            let app_rect = ui.max_rect();

            let title_bar_height = 32.0;
            let title_bar_rect = {
                let mut rect = app_rect;
                rect.max.y = rect.min.y + title_bar_height;
                rect
            };
            self.titlebar(ui, title_bar_rect);

            let content_rect = {
                let mut rect = app_rect;
                rect.min.y = title_bar_rect.max.y;
                rect
            }.shrink(4.0);

            let mut content_ui = ui.child_ui(content_rect, *ui.layout());
            add_contents(&mut content_ui);
        });
    }

    fn titlebar(&self, ui: &mut egui::Ui, titlebar_rect: egui::epaint::Rect) {
        use egui::*;

        let titlebar_response = ui.interact(titlebar_rect, Id::new("titlebar"), Sense::click());

        let painter = ui.painter();

        painter.text(titlebar_rect.center(), Align2::CENTER_CENTER, &self.title, FontId::proportional(20.0), ui.style().visuals.text_color());

        painter.line_segment([
                titlebar_rect.left_bottom() + vec2(1.0, 0.0),
                titlebar_rect.right_bottom() + vec2(-1.0, 0.0),
            ], ui.visuals().widgets.noninteractive.bg_stroke);

        if titlebar_response.double_clicked() {
            let is_minimized = ui.input(|i| i.viewport().maximized.unwrap_or(false));

            ui.ctx().send_viewport_cmd(ViewportCommand::Maximized(!is_minimized));
        }

        if titlebar_response.is_pointer_button_down_on() {
            ui.ctx().send_viewport_cmd(ViewportCommand::StartDrag);
        }


    }
}
