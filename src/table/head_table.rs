use crate::file_ops::FileOps;
use crate::model::{FWord, Fixed};

#[allow(unused)]
#[derive(Debug)]
pub struct HeadTable {
    version: Fixed,
    font_revision: Fixed,
    flags: u16,
    units_per_em: u16,
    created: i64,
    modified: i64,
    x_min: FWord,
    y_min: FWord,
    x_max: FWord,
    y_max: FWord,
    mac_style: u16,
    lowest_rec_ppem: u16, // smallest readable size in pixels
    font_direction_hint: i16,
    pub index_to_loc_format: i16, // 0 for short offsets, 1 for long
    glyph_data_format: i16,
}

impl HeadTable {
    pub fn from_file(file_ops: &mut FileOps, offset: u32) -> HeadTable {
        file_ops.seek_from_start(offset);

        let version = file_ops.read_fixed();
        let font_revision = file_ops.read_fixed();

        let _checksum = file_ops.read_u32();
        let _magic_number = file_ops.read_u32(); // Must be 0x5F0F3CF5
        let flags = file_ops.read_u16();
        let units_per_em = file_ops.read_u16();

        let created = file_ops.read_long_date_time();
        let modified = file_ops.read_long_date_time();

        let x_min = file_ops.read_fword();
        let y_min = file_ops.read_fword();
        let x_max = file_ops.read_fword();
        let y_max = file_ops.read_fword();

        let mac_style = file_ops.read_u16();
        let lowest_rec_ppem = file_ops.read_u16();
        let font_direction_hint = file_ops.read_i16();
        let index_to_loc_format = file_ops.read_i16();
        let glyph_data_format = file_ops.read_i16();

        HeadTable {
            version,
            font_revision,
            flags,
            units_per_em,
            created,
            modified,
            x_min,
            y_min,
            x_max,
            y_max,
            mac_style,
            lowest_rec_ppem, // smallest readable size in pixels
            font_direction_hint,
            index_to_loc_format, // 0 for short offsets, 1 for long
            glyph_data_format,
        }
    }
}
