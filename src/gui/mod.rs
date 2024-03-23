mod main_panel;
mod open_file;

use std::collections::HashMap;

use eframe::{
    egui::{CentralPanel, Context},
    Frame,
};
use strum::EnumIter;

use crate::{ihex_file::IHexFile, ihex_record::DataRecord};

#[derive(EnumIter, PartialEq, Eq, Clone)]
enum DataDisplayMode {
    Bytes,
    Chars,
    Utf8,
    Utf16LE,
    Utf16BE,
}

impl DataDisplayMode {
    const fn as_str(&self) -> &'static str {
        match self {
            DataDisplayMode::Bytes => "Bytes",
            DataDisplayMode::Chars => "ASCII",
            DataDisplayMode::Utf8 => "UTF-8",
            DataDisplayMode::Utf16LE => "UTF-16 (LE)",
            DataDisplayMode::Utf16BE => "UTF-16 (BE)",
        }
    }
}

struct DataRecordDisplayMeta {
    display_mode: DataDisplayMode,
}

impl Default for DataRecordDisplayMeta {
    fn default() -> Self {
        DataRecordDisplayMeta {
            display_mode: DataDisplayMode::Bytes,
        }
    }
}

pub struct Gui {
    file: Option<IHexFile>,
    data_display_meta: HashMap<DataRecord, DataRecordDisplayMeta>,
    set_all_to_mode: DataDisplayMode,
}

impl Gui {
    pub fn new(_cc: &eframe::CreationContext, file: Option<IHexFile>) -> Self {
        Gui {
            file,
            data_display_meta: HashMap::new(),
            set_all_to_mode: DataDisplayMode::Bytes,
        }
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
