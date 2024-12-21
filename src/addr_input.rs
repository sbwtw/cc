use eframe::egui::{Label, Sense, TextEdit, Widget};

use crate::AddrString;

pub enum AddrMode {
    Hex,
    Dec,
}

impl From<&AddrMode> for &'static str {
    fn from(value: &AddrMode) -> Self {
        match *value {
            AddrMode::Hex => "16",
            AddrMode::Dec => "10",
        }
    }
}

pub struct AddrInput<'a> {
    input: &'a mut AddrString,
    desired_width: f32,
}

impl<'a> AddrInput<'a> {
    pub fn new(text: &'a mut AddrString) -> Self {
        Self {
            input: text,
            desired_width: 80.,
        }
    }

    pub fn toggle_mode(&mut self) {
        self.input.toggle_mode();
    }

    pub fn desired_width(mut self, w: f32) -> Self {
        self.desired_width = w;
        self
    }
}

impl Widget for &mut AddrInput<'_> {
    fn ui(self, ui: &mut eframe::egui::Ui) -> eframe::egui::Response {
        let mut response = ui.response();

        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing = [0., 0.].into();

            let input = TextEdit::singleline(self.input.buffer()).desired_width(self.desired_width);
            ui.add(input);
            let label = Label::new(Into::<&'static str>::into(&self.input.mode()))
                .selectable(false)
                .sense(Sense::click());
            response = ui.add(label);
        });

        response
    }
}
