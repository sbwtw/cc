use eframe::egui;

struct CompilerCalc {}

impl Default for CompilerCalc {
    fn default() -> Self {
        Self {}
    }
}

impl eframe::App for CompilerCalc {
    fn update(&mut self, ctx: &eframe::egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| ui.label("text"));
    }
}

fn main() -> eframe::Result {
    env_logger::init();
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([800., 600.]),
        ..Default::default()
    };

    eframe::run_native(
        "Compiler Calculator",
        options,
        Box::new(|ctx| {
            ctx.egui_ctx.set_theme(egui::Theme::Light);
            Ok(Box::<CompilerCalc>::default())
        }),
    )
}
