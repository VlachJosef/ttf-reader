use crate::file_ops::FileOps;
use crate::model::GlyphId;
use crate::table::head_table::HeadTable;
use crate::table::maxp_table::MaximumProfileTable;

#[derive(Debug)]
pub struct IndexToLocTable {
    data: Vec<u32>,
}

impl IndexToLocTable {
    pub fn mk_index_to_loc_table(
        file_ops: &mut FileOps,
        loca_table_offset: u32,
        head_table: HeadTable,
        maximum_profile_table: MaximumProfileTable,
    ) -> IndexToLocTable {
        file_ops.seek_from_start(loca_table_offset);
        let data: Vec<u32> = match head_table.index_to_loc_format {
            0 => todo!("                       SHORT"),
            1 => (0..maximum_profile_table.num_glyphs)
                .map(|_| file_ops.read_u32())
                .collect(),
            _ => unreachable!("Only 0 and 1 is supported per specification"),
        };

        IndexToLocTable { data }
    }

    pub fn index_for(&self, glyph_id: &GlyphId) -> u32 {
        self.data[glyph_id.id() as usize]
    }
}
