use crate::model::{FWord, Fixed, UFWord};
use crate::reader::Reader;

#[allow(unused)]
#[derive(Debug)]
pub struct HheaTable {
    version: Fixed,
    ascent: FWord,
    descent: FWord,
    line_gap: FWord,
    advance_width_mac: UFWord,
    min_left_side_bearing: FWord,
    min_right_side_bearing: FWord,
    x_max_extent: FWord,
    caret_slope_rise: i16,
    caret_slope_run: i16,
    caret_offset: FWord,
    metric_data_format: i16,
    pub num_of_long_hor_metrics: u16,
}

impl HheaTable {
    pub fn from_file(reader: &mut Box<dyn Reader>, offset: u32) -> HheaTable {
        reader.seek_from_start(offset);
        let version: Fixed = reader.read_fixed();
        let ascent: FWord = reader.read_fword();
        let descent: FWord = reader.read_fword();
        let line_gap: FWord = reader.read_fword();
        let advance_width_mac: UFWord = reader.read_ufword();
        let min_left_side_bearing: FWord = reader.read_fword();
        let min_right_side_bearing: FWord = reader.read_fword();
        let x_max_extent: FWord = reader.read_fword();
        let caret_slope_rise: i16 = reader.read_i16();
        let caret_slope_run: i16 = reader.read_i16();
        let caret_offset: FWord = reader.read_fword();

        let _reserved = reader.read_i16();
        let _reserved = reader.read_i16();
        let _reserved = reader.read_i16();
        let _reserved = reader.read_i16();

        let metric_data_format: i16 = reader.read_i16();
        let num_of_long_hor_metrics: u16 = reader.read_u16();
        HheaTable {
            version,
            ascent,
            descent,
            line_gap,
            advance_width_mac,
            min_left_side_bearing,
            min_right_side_bearing,
            x_max_extent,
            caret_slope_rise,
            caret_slope_run,
            caret_offset,
            metric_data_format,
            num_of_long_hor_metrics,
        }
    }
}
