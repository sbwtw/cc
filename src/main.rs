#![windows_subsystem = "windows"]

use eframe::egui::{self, Align2, Label, Sense};

mod addr;
use addr::*;

mod addr_input;
use addr_input::AddrInput;

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
    start_addr: AddrString,
    offset_addr: AddrString,
    memory_addr: AddrString,
    direction: CalcDirection,
}

impl LocData {
    pub fn toggle_direction(&mut self) {
        match self.direction {
            CalcDirection::StartAddressToMemory => {
                self.direction = CalcDirection::MemoryToStartAddress
            }
            CalcDirection::MemoryToStartAddress => {
                self.direction = CalcDirection::StartAddressToMemory
            }
        }
    }

    pub fn direction_text(&self) -> &'static str {
        match self.direction {
            CalcDirection::StartAddressToMemory => "->",
            CalcDirection::MemoryToStartAddress => "<-",
        }
    }

    pub fn update_data(&mut self, base_addr: u64) {
        match self.direction {
            CalcDirection::MemoryToStartAddress => self.update_function_data(base_addr),
            CalcDirection::StartAddressToMemory => self.update_memory_data(base_addr),
        };
    }

    fn update_memory_data(&mut self, base_addr: u64) {
        let start_addr = match self.start_addr.get_addr() {
            Some(v) => v,
            _ => return,
        };
        let offset_addr = self.offset_addr.get_addr().unwrap_or(0);
        let mem_addr = match base_addr.checked_add(start_addr + offset_addr) {
            Some(v) => v,
            _ => return,
        };

        self.memory_addr.update_data(mem_addr);
    }

    fn update_function_data(&mut self, base_addr: u64) {
        let mem_addr = match self.memory_addr.get_addr() {
            Some(v) => v,
            _ => return,
        };
        let offset_addr = self.offset_addr.get_addr().unwrap_or(0);
        let start_addr = match mem_addr.checked_sub(base_addr + offset_addr) {
            Some(v) => v,
            _ => return,
        };

        self.start_addr.update_data(start_addr);
    }
}

struct AreaData {
    base_addr: AddrString,
    locations: Vec<LocData>,
}

impl AreaData {
    pub fn update_data(&mut self) {
        if let Some(base_addr) = self.base_addr.get_addr() {
            for loc in &mut self.locations {
                loc.update_data(base_addr);
            }
        }
    }
}

impl Default for AreaData {
    fn default() -> Self {
        Self {
            base_addr: Default::default(),
            locations: vec![
                LocData {
                    start_addr: AddrString::default_decimal(),
                    ..Default::default()
                },
                LocData {
                    start_addr: AddrString::default_decimal(),
                    direction: CalcDirection::MemoryToStartAddress,
                    memory_addr: AddrString::default_decimal(),
                    ..Default::default()
                },
                LocData {
                    start_addr: AddrString::default_decimal(),
                    ..Default::default()
                },
                LocData {
                    start_addr: AddrString::default_decimal(),
                    direction: CalcDirection::MemoryToStartAddress,
                    memory_addr: AddrString::default_decimal(),
                    ..Default::default()
                },
            ],
        }
    }
}

struct CompilerCalc {
    info_loading_window_show: bool,
    info_data: String,
    current_area_tab: usize,
    area_data: Vec<AreaData>,
}

impl Default for CompilerCalc {
    fn default() -> Self {
        Self {
            info_data: String::new(),
            info_loading_window_show: false,
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
        if self.info_loading_window_show {
            egui::Window::new("Import area base address")
                .anchor(Align2::CENTER_CENTER, [0., -50.])
                .default_size([500., 400.])
                .show(ctx, |ui| {
                    ui.vertical(|ui| {
                        ui.horizontal(|ui| {
                            ui.label("Input area addresses:");
                            if ui.button("Recognize").clicked() {
                                self.info_loading_window_show = false;
                            }
                        });

                        ui.text_edit_multiline(&mut self.info_data);
                    });
                });
        }

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
                // Load button
                if ui.button("Load areas...").clicked() {
                    self.info_loading_window_show = true;
                }
            });

            let current_area = self.area_data.get_mut(self.current_area_tab).unwrap();
            // TODO: Shouldn't update every times, but who cares :-)
            current_area.update_data();
            ui.separator();

            // Area base address
            ui.horizontal(|ui| {
                ui.label("Base Address:");
                let mut input = AddrInput::new(&mut current_area.base_addr);
                if ui.add(&mut input).clicked() {
                    input.toggle_mode();
                }
            });

            ui.add_space(10.);

            // Contents
            for (index, loc_data) in &mut current_area.locations.iter_mut().enumerate() {
                if index != 0 {
                    ui.add_space(5.);
                }

                ui.horizontal(|ui| {
                    ui.label("Tips:");
                    ui.text_edit_singleline(&mut loc_data.comment);
                });
                ui.horizontal(|ui| {
                    ui.label("FunAddr:");
                    let mut start_input =
                        AddrInput::new(&mut loc_data.start_addr).desired_width(80.);
                    if ui.add(&mut start_input).clicked() {
                        start_input.toggle_mode();
                    }
                    ui.label("Offset:");
                    let mut offset_input =
                        AddrInput::new(&mut loc_data.offset_addr).desired_width(50.);
                    if ui.add(&mut offset_input).clicked() {
                        offset_input.toggle_mode();
                    }

                    let dir_text = loc_data.direction_text();
                    if ui
                        .add(Label::new(dir_text).selectable(false).sense(Sense::click()))
                        .clicked()
                    {
                        loc_data.toggle_direction();
                    }

                    ui.label("MemAddr:");
                    let mut mem_input =
                        AddrInput::new(&mut loc_data.memory_addr).desired_width(80.);
                    if ui.add(&mut mem_input).clicked() {
                        mem_input.toggle_mode();
                    }
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
        viewport: egui::ViewportBuilder::default()
            .with_inner_size(prefer_size)
            .with_min_inner_size(min_size),
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
