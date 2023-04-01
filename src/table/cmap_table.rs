use crate::font_directory::TableDirectory;
use crate::glyph_index_lookup::GlyphIndexLookup;
use crate::model::{GlyphId, PlatformId};
use crate::reader::Reader;

#[allow(unused)]
#[derive(Debug)]
pub struct CMapSubtable {
    platform_id: PlatformId,
    platform_specific_id: u16,
    offset: u32,
}

impl CMapSubtable {
    pub fn find_cmap_subtable(
        reader: &mut Box<dyn Reader>,
        cmap_table: &TableDirectory,
    ) -> CMapSubtable {
        reader.seek_from_start(cmap_table.offset);
        let _version = reader.read_u16();
        let number_subtables = reader.read_u16();

        let subtables = (0..number_subtables).into_iter().map(|_| {
            let platform_id = reader.read_platform_id();
            let platform_specific_id = reader.read_u16();
            let offset = reader.read_u32();

            let offset = cmap_table.offset + offset;

            CMapSubtable {
                platform_id,
                platform_specific_id,
                offset,
            }
        });

        subtables
            .into_iter()
            .find(|subtable| subtable.platform_id == PlatformId::Unicode)
            .unwrap()
    }

    pub fn segments(&self, reader: &mut Box<dyn Reader>) -> Vec<Segment> {
        reader.seek_from_start(self.offset);
        let subtable_format = reader.read_u16();
        if subtable_format != 4 {
            panic!("Unsuported cmap subtable format: {}", subtable_format);
        }
        let _length = reader.read_u16();
        let _version = reader.read_u16();

        let seg_count_x2 = reader.read_u16(); // The segCount is the number of contiguous code ranges in the font
        let _search_range = reader.read_u16();
        let _entry_selector = reader.read_u16();
        let _range_shift = reader.read_u16();

        self.read_whole_subtable(reader, seg_count_x2)
    }

    pub fn find_glyph_id(&self, reader: &mut Box<dyn Reader>, char_code: u16) -> GlyphId {
        reader.seek_from_start(self.offset);
        let subtable_format = reader.read_u16();
        if subtable_format != 4 {
            panic!("Unsuported cmap subtable format: {}", subtable_format);
        }
        let _length = reader.read_u16();
        let _version = reader.read_u16();

        let seg_count_x2 = reader.read_u16(); // The segCount is the number of contiguous code ranges in the font
        let search_range = reader.read_u16(); // TODO compute this from seg_count_x2
        let entry_selector = reader.read_u16(); // TODO compute this from seg_count_x2
        let _range_shift = reader.read_u16(); // Do not use

        let mut index_lookup = GlyphIndexLookup::new(reader, seg_count_x2, char_code);

        index_lookup.seek_glyph_id(search_range, entry_selector)
    }

    fn read_array(&self, reader: &mut Box<dyn Reader>, seg_count: u16) -> Vec<u16> {
        (0..seg_count)
            .into_iter()
            .map(|_| reader.read_u16())
            .collect()
    }

    fn read_whole_subtable(&self, reader: &mut Box<dyn Reader>, seg_count_x2: u16) -> Vec<Segment> {
        let seg_count = seg_count_x2 / 2;

        let end_codes: Vec<u16> = self.read_array(reader, seg_count);

        let reserved_pad = reader.read_u16();

        assert_eq!(reserved_pad, 0);

        let start_codes: Vec<u16> = self.read_array(reader, seg_count);
        let id_deltas: Vec<u16> = self.read_array(reader, seg_count);
        let id_range_offsets: Vec<u16> = self.read_array(reader, seg_count);

        let segments: Vec<Segment> = start_codes
            .into_iter()
            .enumerate()
            .zip(end_codes)
            .zip(id_deltas)
            .zip(id_range_offsets)
            .map(
                |((((index, start_code), end_code), id_delta), id_range_offset)| Segment {
                    index,
                    start_code,
                    end_code,
                    id_delta,
                    id_range_offset,
                },
            )
            .collect::<Vec<Segment>>();

        segments
    }
}

#[derive(Debug, PartialEq)]
pub struct Segment {
    index: usize,
    pub start_code: u16,
    pub end_code: u16,
    id_delta: u16,
    id_range_offset: u16,
}

impl Segment {
    pub fn new(
        index: usize,
        start_code: u16,
        end_code: u16,
        id_delta: u16,
        id_range_offset: u16,
    ) -> Segment {
        Segment {
            index,
            start_code,
            end_code,
            id_delta,
            id_range_offset,
        }
    }
}
