use crate::state::AppState;
use crate::theme::{apply_fonts, Theme};
use crate::ui::{Body, Footer, Header, Window};

use eframe::egui;

pub struct FundingApp {
    state: AppState,
}

impl FundingApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        egui_extras::install_image_loaders(&cc.egui_ctx);
        apply_fonts(&cc.egui_ctx);

        let mut state = AppState::new();
        state.theme = Theme::default();

        Self { state }
    }
}

impl eframe::App for FundingApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.state.process_events();
        self.state.fps = 1.0 / ctx.input(|i| i.stable_dt.max(0.0001));
        self.state.theme.apply(ctx);

        Header::show(ctx, &mut self.state);
        Footer::show(ctx, &mut self.state);
        Body::show(ctx, &mut self.state);
        Window::show(ctx, &mut self.state);
        ctx.request_repaint_after_secs(1.0)
    }
}
