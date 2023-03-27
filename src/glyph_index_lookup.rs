use crate::file_ops::FileOps;
use crate::model::GlyphId;

pub struct GlyphIndexLookup<'a> {
    file_ops: &'a mut FileOps,
    seg_count_x2: u16,
    char_code: u16,
}

impl<'a> GlyphIndexLookup<'a> {
    pub fn new(file_ops: &'a mut FileOps, seg_count_x2: u16, char_code: u16) -> GlyphIndexLookup {
        GlyphIndexLookup {
            file_ops,
            seg_count_x2,
            char_code,
        }
    }

    fn reset_to_end_code_read(&mut self) {
        // Go from start_code array (+reservedPad) back to end_code array
        self.file_ops
            .seek_from_current(-2 - self.seg_count_x2 as i32);
    }

    // First segment is not covered by binary search, so it needs to be checked explicitly
    pub fn check_first_segment(&mut self) -> Option<GlyphId> {
        let end_code = self.file_ops.read_end_code(0);
        let start_code = self.read_start_code();
        if self.char_code <= end_code && self.char_code > start_code {
            let id_delta = self.read_id_delta();
            let id_range_offset = self.read_id_range_offset();

            Some(self.compute_glyp_id(start_code, id_delta, id_range_offset))
        } else {
            self.reset_to_end_code_read();
            None
        }
    }

    pub fn seek_glyph_id(&mut self, search_range: u16, entry_selector: u16) -> GlyphId {
        let maybe_glyph_id = self.check_first_segment();

        match maybe_glyph_id {
            Some(glyph_id) => glyph_id,
            None => {
                let end_code = self.file_ops.read_end_code(search_range as i32);

                if self.char_code > end_code {
                    self.sequential_search()
                } else {
                    self.binary_search(end_code, search_range, entry_selector)
                }
            }
        }
    }

    fn sequential_search(&mut self) -> GlyphId {
        let next_end_code = self.file_ops.read_u16();

        if next_end_code >= self.char_code {
            self.file_ops.seek_from_current(-2);
            let start_code = self.read_start_code();
            let id_delta = self.read_id_delta();
            let id_range_offset = self.read_id_range_offset();

            self.compute_glyp_id(start_code, id_delta, id_range_offset)
        } else {
            if next_end_code == 0xFFFF {
                GlyphId::MISSING_CHARACTER_GLYPH
            } else {
                self.sequential_search()
            }
        }
    }

    fn binary_search(&mut self, end_code: u16, search_range: u16, entry_selector: u16) -> GlyphId {
        let start_code = self.read_start_code();

        if self.char_code <= end_code && self.char_code >= start_code {
            let id_delta = self.read_id_delta();
            let id_range_offset = self.read_id_range_offset();

            self.compute_glyp_id(start_code, id_delta, id_range_offset)
        } else {
            if entry_selector == 0 {
                GlyphId::MISSING_CHARACTER_GLYPH
            } else {
                let search_range = search_range >> 1;

                let end_code_range = if self.char_code < end_code {
                    -(search_range as i32)
                } else {
                    search_range as i32
                };

                self.reset_to_end_code_read();
                let end_code = self.file_ops.read_end_code(end_code_range);
                self.binary_search(end_code, search_range, entry_selector - 1)
            }
        }
    }

    fn compute_glyp_id(&mut self, start_code: u16, id_delta: u16, id_range_offset: u16) -> GlyphId {
        let glyph_id = if id_range_offset == 0 {
            println!(
                "char_code {} start_code {} id_delta {}",
                self.char_code, start_code, id_delta
            );
            // If the id_range_offset is 0, the id_delta value is added directly to the character code to get the corresponding glyph index
            id_delta as u32 + self.char_code as u32
        } else {
            let address = self.file_ops.read_address();

            let glyph_index_address = id_range_offset as u32
                + 2 * ((self.char_code - start_code) as u32)
                + address as u32;

            self.file_ops.seek_from_start(glyph_index_address);
            self.file_ops.read_u16() as u32
        };

        // NOTE: All id_delta[i] arithmetic is modulo 65536.
        let glyph_id = (glyph_id % (u16::MAX as u32 + 1)) as u16;

        GlyphId::new(glyph_id)
    }

    fn read_start_code(&mut self) -> u16 {
        self.file_ops.read_start_code(self.seg_count_x2)
    }

    fn read_id_delta(&mut self) -> u16 {
        self.file_ops.read_id_delta(self.seg_count_x2)
    }

    fn read_id_range_offset(&mut self) -> u16 {
        self.file_ops.read_id_range_offset(self.seg_count_x2)
    }
}
