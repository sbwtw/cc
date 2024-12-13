use eframe::egui::{self, TextEdit};

struct CompilerCalc {
    area_count: usize,
    current_area_tab: usize,
    offset_data_1: String,
}

impl Default for CompilerCalc {
    fn default() -> Self {
        Self {
            area_count: 6,
            current_area_tab: 0,
            offset_data_1: String::new(),
        }
    }
}

impl eframe::App for CompilerCalc {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            // Areas
            ui.horizontal(|ui| {
                for i in 0..self.area_count {
                    ui.selectable_value(&mut self.current_area_tab, i, format!("Area {i}"));
                }

                if ui.small_button("-").clicked() {
                    self.area_count -= 1;
                }
                if ui.small_button("+").clicked() {
                    self.area_count += 1;
                }
            });

            ui.separator();

            // Area base address
            ui.horizontal(|ui| {
                ui.label("Base Address:");
                ui.add(TextEdit::singleline(&mut self.offset_data_1).desired_width(100.));
            });

            ui.add_space(10.);

            // Contents
            ui.text_edit_singleline(&mut self.offset_data_1);
            ui.horizontal(|ui| {
                ui.label("Offset1");
                ui.add(TextEdit::singleline(&mut self.offset_data_1).desired_width(60.));
                ui.label("Offset2");
                ui.add(TextEdit::singleline(&mut self.offset_data_1).desired_width(40.));
                ui.label("Offset3");
                ui.add(TextEdit::singleline(&mut self.offset_data_1).desired_width(30.));
            });
        });
    }
}

fn main() -> eframe::Result {
    env_logger::init();
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([450., 320.]),
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
