use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use crate::ihex_file::IHexFile;
use eframe::{
    egui::{Context, Ui},
    Frame,
};
use rfd::FileDialog;

use super::Gui;

pub(super) fn gui(gui: &mut Gui, _ctx: &Context, _frame: &mut Frame, ui: &mut Ui) {
    ui.vertical_centered(|ui| {
        ui.heading("Select file");
        if ui.button("Open").clicked() {
            let picked_path = FileDialog::new()
                .add_filter("Intel HEX", &["hex"])
                .add_filter("Others", &["*"])
                .pick_file();

            if let Some(path) = picked_path {
                log::debug!("Opening file: {}", path.display());

                let file = File::open(path);

                match file {
                    Ok(file) => {
                        log::info!("File opened successfully");
                        log::debug!("Parsing file");
                        let parsed = IHexFile::read(BufReader::new(file).lines());

                        match parsed {
                            Ok(parsed) => {
                                log::info!("File parsed successfully");
                                gui.file = Some(parsed);
                            }
                            Err(e) => log::error!("Could not parse file: {}", e),
                        }
                    }
                    Err(e) => log::error!("Could not open file: {}", e),
                }
            }
        }
    });
}
