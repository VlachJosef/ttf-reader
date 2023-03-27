use crate::file_ops::FileOps;
use crate::model::{FWord, GlyphId};
use crate::table::hhea_table::HheaTable;
use crate::table::maxp_table::MaximumProfileTable;
use std::collections::HashMap;

#[allow(unused)]
#[derive(Debug)]
pub struct LongHorMetric {
    pub advance_width: u16,
    pub left_side_bearing: i16,
}

pub struct LongHorMetricLookup(pub HashMap<GlyphId, LongHorMetric>);

impl LongHorMetricLookup {
    pub fn from_file(
        file_ops: &mut FileOps,
        offset: u32,
        hhea_table: HheaTable,
        maxp_table: &MaximumProfileTable,
    ) -> LongHorMetricLookup {
        file_ops.seek_from_start(offset);
        let mut result: HashMap<GlyphId, LongHorMetric> = HashMap::new();
        let mut last_advance_width = 0;

        (0..hhea_table.num_of_long_hor_metrics)
            .into_iter()
            .for_each(|index| {
                let advance_width: u16 = file_ops.read_u16();
                let left_side_bearing: i16 = file_ops.read_i16();
                let long_hor_matrics = LongHorMetric {
                    advance_width,
                    left_side_bearing,
                };
                last_advance_width = advance_width;
                result.insert(GlyphId::new(index), long_hor_matrics);
            });

        // If num_of_long_hor_metrics is less than the total number of glyphs,
        // then the h_metrics array is followed by an array for the left side
        // bearing values of the remaining glyphs.
        (0..maxp_table.num_glyphs - hhea_table.num_of_long_hor_metrics)
            .into_iter()
            .for_each(|index| {
                let left_side_bearing: i16 = file_ops.read_i16();
                let long_hor_matrics = LongHorMetric {
                    advance_width: last_advance_width,
                    left_side_bearing,
                };
                let index = hhea_table.num_of_long_hor_metrics + index;
                result.insert(GlyphId::new(index), long_hor_matrics);
            });

        LongHorMetricLookup(result)
    }
}
