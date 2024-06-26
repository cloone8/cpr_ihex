mod main_panel;
mod open_file;

use eframe::{
    egui::{CentralPanel, Context},
    Frame,
};
use strum::EnumIter;

use crate::record::{file::IHexFile, IHexRecord};

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

struct DataDisplayMeta {
    displaymode: DataDisplayMode,
}

enum IHexRecordDisplayMeta {
    Data(DataDisplayMeta),
    EndOfFile,
    ExtendedSegmentAddress,
    StartSegmentAddress,
    ExtendedLinearAddress,
    StartLinearAddress,
}

impl IHexRecordDisplayMeta {
    fn check_matches(&self, record: &IHexRecord) -> bool {
        match record {
            IHexRecord::Data(_) => matches!(self, IHexRecordDisplayMeta::Data { .. }),
            IHexRecord::EndOfFile => matches!(self, IHexRecordDisplayMeta::EndOfFile),
            IHexRecord::ExtendedSegmentAddress(_) => {
                matches!(self, IHexRecordDisplayMeta::ExtendedSegmentAddress)
            }
            IHexRecord::StartSegmentAddress(_) => {
                matches!(self, IHexRecordDisplayMeta::StartSegmentAddress)
            }
            IHexRecord::ExtendedLinearAddress(_) => {
                matches!(self, IHexRecordDisplayMeta::ExtendedLinearAddress)
            }
            IHexRecord::StartLinearAddress(_) => {
                matches!(self, IHexRecordDisplayMeta::StartLinearAddress)
            }
        }
    }

    fn default_for(record: &IHexRecord) -> Self {
        match record {
            IHexRecord::Data(_) => IHexRecordDisplayMeta::Data(DataDisplayMeta {
                displaymode: DataDisplayMode::Bytes,
            }),
            IHexRecord::EndOfFile => IHexRecordDisplayMeta::EndOfFile,
            IHexRecord::ExtendedSegmentAddress(_) => IHexRecordDisplayMeta::ExtendedSegmentAddress,
            IHexRecord::StartSegmentAddress(_) => IHexRecordDisplayMeta::StartSegmentAddress,
            IHexRecord::ExtendedLinearAddress(_) => IHexRecordDisplayMeta::ExtendedLinearAddress,
            IHexRecord::StartLinearAddress(_) => IHexRecordDisplayMeta::StartLinearAddress,
        }
    }
}

struct DataTabMeta {
    record_meta: Vec<IHexRecordDisplayMeta>,
    set_all_to_mode: DataDisplayMode,
}

enum MainPanelTab {
    Data,
}

struct MainPanelMeta {
    data: DataTabMeta,
}

pub struct MainPanel {
    file: IHexFile,
    tab: MainPanelTab,
    meta: MainPanelMeta,
}

pub enum Gui {
    OpenFile,
    MainPanel(MainPanel),
}

impl Gui {
    pub fn new(_cc: &eframe::CreationContext, file: Option<IHexFile>) -> Self {
        let mut gui = Gui::OpenFile;

        if let Some(file) = file {
            gui.file_opened(file)
        }

        gui
    }

    pub fn file_opened(&mut self, file: IHexFile) {
        let record_meta: Vec<_> = file
            .records
            .iter()
            .map(IHexRecordDisplayMeta::default_for)
            .collect();
        *self = Gui::MainPanel(MainPanel {
            file,
            tab: MainPanelTab::Data,
            meta: MainPanelMeta {
                data: DataTabMeta {
                    record_meta,
                    set_all_to_mode: DataDisplayMode::Bytes,
                },
            },
        });
    }
}

impl eframe::App for Gui {
    fn update(&mut self, ctx: &Context, frame: &mut Frame) {
        CentralPanel::default().show(ctx, |ui| match self {
            Gui::OpenFile => open_file::gui(self, ctx, frame, ui),
            Gui::MainPanel(main_panel) => main_panel::gui(main_panel, ctx, frame, ui),
        });
    }
}
