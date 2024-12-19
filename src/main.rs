#![windows_subsystem = "windows"]

use eframe::egui::{self, Align, Label, Layout, TextEdit};
use egui_extras::{StripBuilder, Size};

enum CalcDirection {
    StartAddressToMemory,
    MemoryToStartAddress,
}

impl Default for CalcDirection {
    fn default() -> Self {
        Self::StartAddressToMemory
    }
}

#[derive(Default)]
struct AreaData {
    comment: String,
    base_addr: String,
    start_addr: String,
    offset_addr: String,
    memory_addr: String,
    direction: CalcDirection,
}

struct CompilerCalc {
    current_area_tab: usize,
    area_data: Vec<AreaData>,
}

impl Default for CompilerCalc {
    fn default() -> Self {
        Self {
            current_area_tab: 0,
            area_data: vec![
                Default::default(),
                Default::default(),
                Default::default(),
                Default::default(),
                Default::default(),
            ],
        }
    }
}

impl eframe::App for CompilerCalc {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            // Areas
            ui.horizontal(|ui| {
                for i in 0..self.area_data.len() {
                    ui.selectable_value(&mut self.current_area_tab, i, format!("Area {i}"));
                }

                if ui.small_button("-").clicked() && !self.area_data.is_empty() {
                    self.area_data.pop();
                }
                if ui.small_button("+").clicked() {
                    self.area_data.push(Default::default());
                }
            });

            let current_area = self.area_data.get_mut(self.current_area_tab).unwrap();
            ui.separator();

            // Area base address
            ui.horizontal(|ui| {
                ui.label("Base Address:");
                ui.add(TextEdit::singleline(&mut current_area.base_addr).desired_width(100.));
            });

            ui.add_space(10.);

            // Contents
            ui.text_edit_singleline(&mut current_area.comment);
            ui.horizontal(|ui| {
                ui.vertical(|ui| {
                    StripBuilder::new(ui).size(Size::exact(60.)).size(Size::remainder().at_least(80.))
                    .horizontal(|mut strip| {
                        strip.cell(|ui| {
                            ui.label("FunAddr:");
                        });

                        strip.cell(|ui| {
                            ui.add(TextEdit::singleline(&mut current_area.start_addr).desired_width(80.));
                        });
                    });

                    StripBuilder::new(ui).size(Size::exact(60.)).size(Size::remainder().at_least(80.))
                    .horizontal(|mut strip| {
                        strip.cell(|ui| {
                            ui.label("Offset:");
                        });

                        strip.cell(|ui| {
                            ui.add(TextEdit::singleline(&mut current_area.offset_addr).desired_width(80.));
                        });
                    });
                });

                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    ui.add(TextEdit::singleline(&mut current_area.memory_addr).desired_width(80.));
                    ui.label("MemAddr:");
                });
            });
        });
    }
}

fn main() -> eframe::Result {
    env_logger::init();

    let min_size = [450., 320.];
    let prefer_size = [min_size[0] * 1.2, min_size[1] * 1.2];
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size(prefer_size).with_min_inner_size(min_size),
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
