use eframe::{
    egui::{Context, ScrollArea, Ui},
    Frame,
};

use crate::ihex_record::{
    DataRecord, ExtendedLinearAddressRecord, ExtendedSegmentAddressRecord, IHexRecord,
    StartLinearAddressRecord, StartSegmentAddressRecord,
};

use super::Gui;

fn numvec_as_hex_string(data: &[u8]) -> String {
    let str = data
        .iter()
        .map(|byte| format!("0x{:02x}", byte))
        .collect::<Vec<String>>()
        .join(" ");

    format!("[{}]", str)
}

fn display_data(record: &DataRecord, ui: &mut Ui) {
    ui.label(format!(
        "Data: Address: 0x{:x} - Length: {}",
        record.naive_address,
        record.data.len()
    ));
    ui.label(format!("Data: {}", numvec_as_hex_string(&record.data)));
}

fn display_end_of_file(ui: &mut Ui) {
    ui.label("End of file");
}

fn display_extended_segment_address(record: &ExtendedSegmentAddressRecord, ui: &mut Ui) {
    ui.label(format!(
        "Extended segment address: Segment Base Address: {:x}",
        record.segment_base
    ));
}

fn display_start_segment_address(record: &StartSegmentAddressRecord, ui: &mut Ui) {
    ui.label(format!(
        "Start segment address: Code Segment: 0x{:x} - Instruction Pointer: 0x{:x}",
        record.code_segment, record.instruction_pointer
    ));
}

fn display_extended_linear_address(record: &ExtendedLinearAddressRecord, ui: &mut Ui) {
    let base_32: u32 = (record.address_base as u32) << 16;
    ui.label(format!(
        "Extended linear address: Linear Base Address: 0x{:x}",
        base_32
    ));
}

fn display_start_linear_address(record: &StartLinearAddressRecord, ui: &mut Ui) {
    ui.label(format!("Start linear address: 0x{:x}", record.entry_point));
}

fn display_record(i: usize, record: &IHexRecord, ui: &mut Ui) {
    ui.horizontal(|ui| {
        ui.label(format!("Record #{:06}", i));
        ui.add_space(2_f32);

        match record {
            IHexRecord::Data(data) => display_data(data, ui),
            IHexRecord::EndOfFile => display_end_of_file(ui),
            IHexRecord::ExtendedSegmentAddress(esa) => display_extended_segment_address(esa, ui),
            IHexRecord::StartSegmentAddress(ssa) => display_start_segment_address(ssa, ui),
            IHexRecord::ExtendedLinearAddress(ela) => display_extended_linear_address(ela, ui),
            IHexRecord::StartLinearAddress(sla) => display_start_linear_address(sla, ui),
        }
    });

    ui.separator();
}

pub(super) fn gui(gui: &mut Gui, _ctx: &Context, _frame: &mut Frame, ui: &mut Ui) {
    let hexfile = gui.file.as_ref().unwrap();

    ScrollArea::vertical()
        .auto_shrink([false, false])
        .show(ui, |ui| {
            hexfile
                .records
                .iter()
                .enumerate()
                .for_each(|(i, record)| display_record(i, record, ui));
        });
}
