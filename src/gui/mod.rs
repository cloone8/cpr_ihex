mod main_panel;
mod open_file;

use eframe::{
    egui::{CentralPanel, Context},
    Frame,
};

use crate::ihex_file::IHexFile;

pub struct Gui {
    file: Option<IHexFile>,
}

impl Gui {
    pub fn new(_cc: &eframe::CreationContext, file: Option<IHexFile>) -> Self {
        Gui { file }
    }
}

impl eframe::App for Gui {
    fn update(&mut self, ctx: &Context, frame: &mut Frame) {
        CentralPanel::default().show(ctx, |ui| {
            if self.file.is_none() {
                open_file::gui(self, ctx, frame, ui);
                return;
            }

            main_panel::gui(self, ctx, frame, ui);
        });
    }
}
