use crate::file_ops::FileOps;
use crate::model::GlyphId;
use crate::table::head_table::HeadTable;
use crate::table::maxp_table::MaximumProfileTable;

#[derive(Debug)]
pub enum IndexToLocTable {
    Short { num_glyphs: u16, offsets: Vec<u16> },
    Long { num_glyphs: u16, offsets: Vec<u32> },
}

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

impl IndexToLocTable {
    pub fn mk_index_to_loc_table(
        file_ops: &mut FileOps,
        loca_table_offset: u32,
        head_table: HeadTable,
        maximum_profile_table: MaximumProfileTable,
    ) -> IndexToLocTable {
        let num_glyphs = maximum_profile_table.num_glyphs;
        file_ops.seek_from_start(loca_table_offset);
        match head_table.index_to_loc_format {
            0 => IndexToLocTable::Short {
                num_glyphs,
                offsets: (0..num_glyphs + 1).map(|_| file_ops.read_u16()).collect(),
            },
            1 => IndexToLocTable::Long {
                num_glyphs,
                offsets: (0..num_glyphs + 1).map(|_| file_ops.read_u32()).collect(),
            },
            _ => unreachable!("indexToLocFormat can have only 0 or 1 per specification"),
        }
    }

    fn max_limit(&self) -> u16 {
        match self {
            IndexToLocTable::Short { num_glyphs, .. } => *num_glyphs,
            IndexToLocTable::Long { num_glyphs, .. } => *num_glyphs,
        }
    }

    pub fn index_for(&self, glyph_id: &GlyphId) -> GlyphOffset {
        if glyph_id.id() > self.max_limit() {
            panic!(
                "Cannot ask for glyphId ({}) bigger than max number of glyphs ({})",
                glyph_id.id(),
                self.max_limit()
            )
        }

        let id = glyph_id.id();
        let next_id = id + 1;

        match self {
            IndexToLocTable::Short { offsets, .. } => {
                let offset = offsets[id as usize];
                let next_offset = offsets[next_id as usize];

                GlyphOffset::from(offset as u32 * 2, next_offset as u32 * 2)
            }
            IndexToLocTable::Long { offsets, .. } => {
                let offset = offsets[id as usize];
                let next_offset = offsets[next_id as usize];

                GlyphOffset::from(offset, next_offset)
            }
        }
    }
}
