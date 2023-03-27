use crate::file_ops::FileOps;
use crate::model::FWord;
use crate::table::hhea_table::HheaTable;
use crate::table::maxp_table::MaximumProfileTable;

#[allow(unused)]
#[derive(Debug)]
pub struct LongHorMetric {
    advance_width: u16,
    left_side_bearing: i16,
}

#[allow(unused)]
#[derive(Debug)]
pub struct HmtxTable {
    pub h_metrics: Vec<LongHorMetric>,
    left_side_bearing: Vec<FWord>, // If num_of_long_hor_metrics is less than the total number of glyphs,
                                   // then the h_metrics array is followed by an array for the left side
                                   // bearing values of the remaining glyphs.
}

impl HmtxTable {
    pub fn from_file(
        file_ops: &mut FileOps,
        offset: u32,
        hhea_table: HheaTable,
        maxp_table: &MaximumProfileTable,
    ) -> HmtxTable {
        file_ops.seek_from_start(offset);
        let h_metrics: Vec<LongHorMetric> = (0..hhea_table.num_of_long_hor_metrics)
            .into_iter()
            .map(|_| {
                let advance_width: u16 = file_ops.read_u16();
                let left_side_bearing: i16 = file_ops.read_i16();
                LongHorMetric {
                    advance_width,
                    left_side_bearing,
                }
            })
            .collect();
        let left_side_bearing = (0..maxp_table.num_glyphs - hhea_table.num_of_long_hor_metrics)
            .into_iter()
            .map(|_| file_ops.read_fword())
            .collect();
        HmtxTable {
            h_metrics,
            left_side_bearing,
        }
    }
}
