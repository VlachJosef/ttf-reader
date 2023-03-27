use crate::file_ops::FileOps;
use crate::model::{FWord, Fixed, UFWord};

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
    pub fn from_file(file_ops: &mut FileOps, offset: u32) -> HheaTable {
        file_ops.seek_from_start(offset);
        let version: Fixed = file_ops.read_fixed();
        let ascent: FWord = file_ops.read_fword();
        let descent: FWord = file_ops.read_fword();
        let line_gap: FWord = file_ops.read_fword();
        let advance_width_mac: UFWord = file_ops.read_ufword();
        let min_left_side_bearing: FWord = file_ops.read_fword();
        let min_right_side_bearing: FWord = file_ops.read_fword();
        let x_max_extent: FWord = file_ops.read_fword();
        let caret_slope_rise: i16 = file_ops.read_i16();
        let caret_slope_run: i16 = file_ops.read_i16();
        let caret_offset: FWord = file_ops.read_fword();

        let _reserved = file_ops.read_i16();
        let _reserved = file_ops.read_i16();
        let _reserved = file_ops.read_i16();
        let _reserved = file_ops.read_i16();

        let metric_data_format: i16 = file_ops.read_i16();
        let num_of_long_hor_metrics: u16 = file_ops.read_u16();
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
