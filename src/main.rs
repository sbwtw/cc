#![windows_subsystem = "windows"]

use eframe::egui::{self, Label, Sense, TextEdit};

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
struct LocData {
    comment: String,
    start_addr: String,
    offset_addr: String,
    memory_addr: String,
    direction: CalcDirection,
}

struct AreaData {
    base_addr: String,
    locations: Vec<LocData>,
}

impl Default for AreaData {
    fn default() -> Self {
        Self { base_addr: Default::default(), locations: vec![
            Default::default(),
            LocData {
                direction: CalcDirection::MemoryToStartAddress,
                ..Default::default()
            }
        ] }
    }
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
            for loc_data in &mut current_area.locations {
                ui.text_edit_singleline(&mut loc_data.comment);
                ui.horizontal(|ui| {
                    ui.label("FunAddr:");
                    ui.add(TextEdit::singleline(&mut loc_data.start_addr).desired_width(80.));
                    ui.label("Offset:");
                    ui.add(TextEdit::singleline(&mut loc_data.offset_addr).desired_width(80.));

                    let dir_text = match loc_data.direction {
                        CalcDirection::StartAddressToMemory => "->",
                        CalcDirection::MemoryToStartAddress => "<-",
                    };
                    if ui.add(Label::new(dir_text).selectable(false).sense(Sense::click())).clicked() {
                        match loc_data.direction {
                            CalcDirection::StartAddressToMemory => loc_data.direction = CalcDirection::MemoryToStartAddress,
                            CalcDirection::MemoryToStartAddress => loc_data.direction = CalcDirection::StartAddressToMemory,
                        }
                    }

                    ui.label("MemAddr:");
                    ui.add(TextEdit::singleline(&mut loc_data.memory_addr).desired_width(80.));
                });
            }
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
