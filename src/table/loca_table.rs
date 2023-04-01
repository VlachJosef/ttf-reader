use crate::model::GlyphId;
use crate::reader::Reader;
use crate::table::head_table::HeadTable;
use crate::table::maxp_table::MaximumProfileTable;
use std::collections::HashMap;

#[derive(Debug)]
pub struct GlyphOffset {
    is_empty: bool,
    offset: u32,
}

impl GlyphOffset {
    fn from(offset: u32, next_offset: u32) -> GlyphOffset {
        let size = next_offset - offset;
        let is_empty = size == 0;

        GlyphOffset { is_empty, offset }
    }

    pub fn offset(&self) -> u32 {
        self.offset
    }

    pub fn is_empty(&self) -> bool {
        self.is_empty
    }
}

#[derive(Debug)]
pub struct GlyphIdOffsetLookup(pub HashMap<GlyphId, GlyphOffset>);

impl GlyphIdOffsetLookup {
    pub fn mk_glyph_id_to_offset(
        reader: &mut Box<dyn Reader>,
        loca_table_offset: u32,
        head_table: &HeadTable,
        maximum_profile_table: &MaximumProfileTable,
    ) -> GlyphIdOffsetLookup {
        let num_glyphs = maximum_profile_table.num_glyphs;
        reader.seek_from_start(loca_table_offset);
        let mut result: HashMap<GlyphId, GlyphOffset> = HashMap::new();
        match head_table.index_to_loc_format {
            0 => {
                (0..num_glyphs + 1)
                    .map(|_| reader.read_u16())
                    .collect::<Vec<u16>>()
                    .windows(2)
                    .enumerate()
                    .for_each(|(index, x)| match x {
                        [cur, next] => {
                            result.insert(
                                GlyphId::new(index as u16),
                                GlyphOffset::from(*cur as u32 * 2, *next as u32 * 2),
                            );
                        }
                        _ => unreachable!("windows are of size 2"),
                    });
            }
            1 => {
                (0..num_glyphs + 1)
                    .map(|_| reader.read_u32())
                    .collect::<Vec<u32>>()
                    .windows(2)
                    .enumerate()
                    .for_each(|(index, x)| match x {
                        [cur, next] => {
                            result
                                .insert(GlyphId::new(index as u16), GlyphOffset::from(*cur, *next));
                        }
                        _ => unreachable!("windows are of size 2"),
                    });
            }
            _ => unreachable!("index_to_loc_format can have only 0 or 1 per specification"),
        }
        GlyphIdOffsetLookup(result)
    }
}
