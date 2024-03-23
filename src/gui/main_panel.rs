use eframe::{
    egui::{ComboBox, Context, Ui},
    Frame,
};
use egui_extras::{Column, TableBuilder, TableRow};
use itertools::Itertools;
use std::{collections::HashMap, hash::Hash};
use strum::IntoEnumIterator;

use crate::ihex_record::{
    DataRecord, ExtendedLinearAddressRecord, ExtendedSegmentAddressRecord, IHexRecord,
    StartLinearAddressRecord, StartSegmentAddressRecord,
};

use super::{DataDisplayMode, DataRecordDisplayMeta, Gui};

fn display_mode_combobox(id: impl Hash, curr: &mut DataDisplayMode, ui: &mut Ui) {
    ComboBox::from_id_source(id)
        .selected_text(curr.as_str())
        .show_ui(ui, |ui| {
            for mode in DataDisplayMode::iter() {
                ui.selectable_value(curr, mode.clone(), mode.as_str());
            }
        });
}

fn numvec_as_hex_string(data: &[u8]) -> String {
    let str = data
        .iter()
        .map(|byte| format!("0x{:02x}", byte))
        .collect::<Vec<String>>()
        .join(" ");

    format!("[{}]", str)
}

fn numvec_as_char_string(data: &[u8]) -> String {
    // Some slightly weird syntax because intersperse has a future
    // name clash with the stdlib
    let char_str =
        Itertools::intersperse(data.iter().map(|byte| *byte as char), ' ').collect::<String>();

    format!("[{}]", char_str)
}

fn numvec_as_utf8_string(data: &[u8]) -> String {
    format!("[{}]", String::from_utf8_lossy(data))
}

enum Endian {
    Little,
    Big,
}

fn numvec_as_utf16_string(data: &[u8], endian: Endian) -> String {
    let utf16_data = data
        .chunks_exact(2)
        .map(|chunk| match endian {
            Endian::Little => u16::from_le_bytes([chunk[0], chunk[1]]),
            Endian::Big => u16::from_be_bytes([chunk[0], chunk[1]]),
        })
        .collect::<Vec<u16>>();

    format!("[{}]", String::from_utf16_lossy(&utf16_data))
}

fn display_data(i: usize, meta: &mut DataRecordDisplayMeta, record: &DataRecord, ui: &mut Ui) {
    ui.horizontal(|ui| {
        ui.label(format!("Address: 0x{:x}", record.naive_address));
        ui.add_space(5.0);
        ui.label(format!("{} bytes", record.data.len()));
        ui.add_space(5.0);

        display_mode_combobox(i, &mut meta.display_mode, ui);

        let data_str = match meta.display_mode {
            DataDisplayMode::Bytes => numvec_as_hex_string(&record.data),
            DataDisplayMode::Chars => numvec_as_char_string(&record.data),
            DataDisplayMode::Utf8 => numvec_as_utf8_string(&record.data),
            DataDisplayMode::Utf16LE => numvec_as_utf16_string(&record.data, Endian::Little),
            DataDisplayMode::Utf16BE => numvec_as_utf16_string(&record.data, Endian::Big),
        };

        ui.label(data_str);
    });

}

fn display_extended_segment_address(record: &ExtendedSegmentAddressRecord, ui: &mut Ui) {
    ui.label(format!("Segment Base Address: {:x}", record.segment_base));
}

fn display_start_segment_address(record: &StartSegmentAddressRecord, ui: &mut Ui) {
    ui.label(format!(
        "Code Segment: 0x{:x}\nInstruction Pointer: 0x{:x}",
        record.code_segment, record.instruction_pointer
    ));
}

fn display_extended_linear_address(record: &ExtendedLinearAddressRecord, ui: &mut Ui) {
    let base_32: u32 = (record.address_base as u32) << 16;
    ui.label(format!("Linear Base Address: 0x{:x}", base_32));
}

fn display_start_linear_address(record: &StartLinearAddressRecord, ui: &mut Ui) {
    ui.label(format!("0x{:x}", record.entry_point));
}

const fn record_type_name(record: &IHexRecord) -> &'static str {
    match record {
        IHexRecord::Data(_) => "Data",
        IHexRecord::EndOfFile => "End of File",
        IHexRecord::ExtendedSegmentAddress(_) => "Extended Segment Address",
        IHexRecord::StartSegmentAddress(_) => "Start Segment Address",
        IHexRecord::ExtendedLinearAddress(_) => "Extended Linear Address",
        IHexRecord::StartLinearAddress(_) => "Start Linear Address",
    }
}

fn display_record(
    meta: &mut HashMap<DataRecord, DataRecordDisplayMeta>,
    i: usize,
    record: &IHexRecord,
    row: &mut TableRow,
) {
    row.col(|ui| {
        ui.label(format!("#{:06}", i));
    });

    row.col(|ui| {
        ui.label(record_type_name(record));
    });

    row.col(|ui| match record {
        IHexRecord::Data(data) => {
            let found_val = meta.get_mut(data);

            let meta_for_rec = match found_val {
                Some(meta) => meta,
                None => {
                    meta.insert(data.clone(), DataRecordDisplayMeta::default());
                    meta.get_mut(data).unwrap()
                }
            };

            display_data(i, meta_for_rec, data, ui)
        }
        IHexRecord::EndOfFile => (),
        IHexRecord::ExtendedSegmentAddress(esa) => display_extended_segment_address(esa, ui),
        IHexRecord::StartSegmentAddress(ssa) => display_start_segment_address(ssa, ui),
        IHexRecord::ExtendedLinearAddress(ela) => display_extended_linear_address(ela, ui),
        IHexRecord::StartLinearAddress(sla) => display_start_linear_address(sla, ui),
    });
}

const TABLE_ROW_HEIGHT: f32 = 20.0;

fn get_record_height(record: &IHexRecord) -> f32 {
    match record {
        IHexRecord::Data(_) => TABLE_ROW_HEIGHT,
        IHexRecord::EndOfFile => TABLE_ROW_HEIGHT,
        IHexRecord::ExtendedSegmentAddress(_) => TABLE_ROW_HEIGHT * 3.0,
        IHexRecord::StartSegmentAddress(_) => TABLE_ROW_HEIGHT * 3.0,
        IHexRecord::ExtendedLinearAddress(_) => TABLE_ROW_HEIGHT * 3.0,
        IHexRecord::StartLinearAddress(_) => TABLE_ROW_HEIGHT,
    }
}

pub fn gui(gui: &mut Gui, _ctx: &Context, _frame: &mut Frame, ui: &mut Ui) {
    let hexfile = gui.file.as_ref().unwrap();

    ui.spacing_mut().item_spacing.y += 3.0;

    ui.horizontal(|ui| {
        display_mode_combobox("set_all_mode_box", &mut gui.set_all_to_mode, ui);

        if ui.button("Set all").clicked() {
            for (_, meta) in gui.data_display_meta.iter_mut() {
                meta.display_mode = gui.set_all_to_mode.clone();
            }
        }
    });

    let max_scroll_height = ui.available_height() - TABLE_ROW_HEIGHT;

    TableBuilder::new(ui)
        .striped(true)
        .resizable(true)
        .auto_shrink([false, false])
        .max_scroll_height(max_scroll_height)
        .column(Column::auto().at_least(75.0))
        .column(Column::auto().at_least(60.0))
        .column(Column::remainder())
        .header(TABLE_ROW_HEIGHT, |mut header| {
            header.col(|ui| {
                ui.heading("Index");
            });
            header.col(|ui| {
                ui.heading("Type");
            });
            header.col(|ui| {
                ui.heading("Data");
            });
        })
        .body(|body| {
            let height_iter = hexfile.records.iter().map(get_record_height);

            body.heterogeneous_rows(height_iter, |mut row| {
                let record = hexfile.records.get(row.index()).unwrap();
                display_record(&mut gui.data_display_meta, row.index(), record, &mut row);
            });
        });
}
