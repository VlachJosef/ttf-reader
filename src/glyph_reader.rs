use crate::contours_reader::ContoursReader;
use crate::file_ops::FileOps;
use crate::font_directory::FontDirectory;
use crate::model::{ArgumentTypes, ComponentData, Glyph, GlyphId};
use crate::table::cmap_table::CMapSubtable;
use crate::table::head_table::HeadTable;
use crate::table::hhea_table::HheaTable;
use crate::table::htmx_table::HmtxTable;
use crate::table::loca_table::IndexToLocTable;
use crate::table::maxp_table::MaximumProfileTable;
use crate::table::name_table;
use crate::Segment;

use std::fs::File;

pub struct GlyphReader {
    file_ops: FileOps,
    glyf_table_offset: u32,
    index_to_loc_table: IndexToLocTable,
    cmap_subtable: CMapSubtable,
}

impl GlyphReader {
    pub fn from_file(file: File) -> GlyphReader {
        let mut file_ops: FileOps = FileOps::from_file(file);

        let font_directory: FontDirectory = FontDirectory::from_file(&mut file_ops);

        let loca_table = font_directory.table_directory("loca");
        let glyf_table = font_directory.table_directory("glyf");
        let cmap_table = font_directory.table_directory("cmap");
        let head_table = font_directory.table_directory("head");
        let maxp_table = font_directory.table_directory("maxp");
        let name_table = font_directory.table_directory("name");
        let hhea_table = font_directory.table_directory("hhea");
        let htmx_table = font_directory.table_directory("hmtx");

        //name_table::read_name(&mut file_ops, name_table);

        let head_table = HeadTable::from_file(&mut file_ops, head_table.offset);

        let maximum_profile_table =
            MaximumProfileTable::from_file(&mut file_ops, maxp_table.offset);

        let hhea_table = HheaTable::from_file(&mut file_ops, hhea_table.offset);

        let htmx_table = HmtxTable::from_file(
            &mut file_ops,
            htmx_table.offset,
            hhea_table,
            &maximum_profile_table,
        );

        let index_to_loc_table: IndexToLocTable = IndexToLocTable::mk_index_to_loc_table(
            &mut file_ops,
            loca_table.offset,
            head_table,
            maximum_profile_table,
        );

        let cmap_subtable: CMapSubtable =
            CMapSubtable::find_cmap_subtable(&mut file_ops, cmap_table);

        let glyf_table_offset = glyf_table.offset;

        GlyphReader {
            file_ops,
            glyf_table_offset,
            index_to_loc_table,
            cmap_subtable,
        }
    }

    pub fn glyph_for_glyph_id(&mut self, glyph_id: GlyphId) -> Glyph {
        self.read(glyph_id)
    }

    pub fn cmap_table_segments(&mut self) -> Vec<Segment> {
        self.cmap_subtable.segments(&mut self.file_ops)
    }

    pub fn char_code_to_glyph_id(&mut self, char_code: u16) -> GlyphId {
        self.cmap_subtable
            .find_glyph_id(&mut self.file_ops, char_code)
    }

    pub fn read_glyph(&mut self, char_code: u16) -> Glyph {
        let glyph_id = self.char_code_to_glyph_id(char_code);

        println!("c_c {} g_id {:?}", char_code, glyph_id);

        self.read(glyph_id)
    }

    pub fn all_char_codes(&mut self) -> Vec<u16> {
        let segments: Vec<Segment> = self.cmap_table_segments();
        segments
            .iter()
            .flat_map(|segment| (segment.start_code..=segment.end_code).collect::<Vec<u16>>())
            .collect::<Vec<u16>>()
    }

    fn read(&mut self, glyph_id: GlyphId) -> Glyph {
        let glyph_offset = self.index_to_loc_table.index_for(&glyph_id);

        if glyph_offset.is_empty() {
            Glyph::Empty { glyph_id }
        } else {
            self.file_ops
                .seek_from_start(self.glyf_table_offset + glyph_offset.offset());

            let number_of_contours = self.file_ops.read_i16();
            let x_min = self.file_ops.read_fword();
            let y_min = self.file_ops.read_fword();
            let x_max = self.file_ops.read_fword();
            let y_max = self.file_ops.read_fword();

            // if >= 0 it is a single glyph; if < 0 the glyph is compound
            if number_of_contours >= 0 {
                let mut contours_reader = ContoursReader::new(&mut self.file_ops);
                let simple_glyph = contours_reader.read_contours(number_of_contours);
                let contours = simple_glyph.contours;
                Glyph::Simple {
                    glyph_id,
                    x_min,
                    x_max,
                    y_min,
                    y_max,
                    contours,
                }
            } else {
                let gc = GlyphComponent::new(&mut self.file_ops);

                let components: Vec<ComponentData> = gc.collect();

                Glyph::Compount {
                    glyph_id,
                    components,
                }
            }
        }
    }
}

struct GlyphComponent<'a> {
    file_ops: &'a mut FileOps,
    has_more: bool,
}

impl<'a> GlyphComponent<'a> {
    fn new(file_ops: &'a mut FileOps) -> GlyphComponent<'a> {
        GlyphComponent {
            file_ops,
            has_more: true,
        }
    }
}

impl<'a> Iterator for GlyphComponent<'a> {
    type Item = ComponentData;

    fn next(&mut self) -> Option<Self::Item> {
        if self.has_more {
            //let file_ops = &self.file_ops;
            let component_flag = ComponentFlag::from_file(self.file_ops);
            //component_flag.pretty_print();
            let glyph_index = self.file_ops.read_u16();
            let argument_types = if component_flag.arg1_and_arg2_are_words() {
                if component_flag.args_are_xy_values() {
                    let arg1 = self.file_ops.read_i16();
                    let arg2 = self.file_ops.read_i16();
                    ArgumentTypes::XYValue16(arg1, arg2)
                } else {
                    let arg1 = self.file_ops.read_u16();
                    let arg2 = self.file_ops.read_u16();
                    ArgumentTypes::Point16(arg1, arg2)
                }
            } else {
                if component_flag.args_are_xy_values() {
                    let arg1 = self.file_ops.read_i8();
                    let arg2 = self.file_ops.read_i8();
                    ArgumentTypes::XYValue8(arg1, arg2)
                } else {
                    let arg1 = self.file_ops.read_u8();
                    let arg2 = self.file_ops.read_u8();
                    ArgumentTypes::Point8(arg1, arg2)
                }
            };

            let (a, b, c, d) = if component_flag.we_have_a_scale() {
                let scale = self.file_ops.read_i16();
                (scale, 0, 0, scale)
            } else if component_flag.we_have_an_x_and_y_scale() {
                let x_scale = self.file_ops.read_i16();
                let y_scale = self.file_ops.read_i16();
                (x_scale, 0, 0, y_scale)
            } else if component_flag.we_have_a_two_by_two() {
                let x_scale = self.file_ops.read_i16();
                let scale_01 = self.file_ops.read_i16();
                let scale_10 = self.file_ops.read_i16();
                let y_scale = self.file_ops.read_i16();
                (x_scale, scale_01, scale_10, y_scale)
            } else {
                (1, 0, 0, 1)
            };

            let cd = ComponentData {
                glyph_index,
                a,
                b,
                c,
                d,
                argument_types,
            };

            self.has_more = component_flag.more_components();

            Some(cd)
        } else {
            None
        }
    }
}

#[derive(Debug)]
struct ComponentFlag(u16);

impl ComponentFlag {
    fn from_file(file_ops: &mut FileOps) -> ComponentFlag {
        ComponentFlag(file_ops.read_u16())
    }

    #[allow(unused)]
    #[rustfmt::skip]
    fn pretty_print(&self) {
        println!("arg1_and_arg2_are_words  {}", self.arg1_and_arg2_are_words());
        println!("args_are_xy_values       {}", self.args_are_xy_values());
        println!("round_xy_to_grid         {}", self.round_xy_to_grid());
        println!("we_have_a_scale          {}", self.we_have_a_scale());
        println!("obsolete                 {}", self.obsolete());
        println!("more_components          {}", self.more_components());
        println!("we_have_an_x_and_y_scale {}", self.we_have_an_x_and_y_scale());
        println!("we_have_a_two_by_two     {}", self.we_have_a_two_by_two());
        println!("we_have_instructions     {}", self.we_have_instructions());
        println!("use_my_metrics           {}", self.use_my_metrics());
        println!("overlap_compound         {}", self.overlap_compound());
        println!("scaled_component_offset  {}", self.scaled_component_offset());
        println!("unscaled_component_offset{}", self.unscaled_component_offset());
    }

    fn is_set(&self, bit: u8) -> bool {
        let shift = 1 << bit;
        self.0 & shift == shift
    }

    fn arg1_and_arg2_are_words(&self) -> bool {
        self.is_set(0)
    }

    // This flag must always be set for the first component of a composite glyph.
    fn args_are_xy_values(&self) -> bool {
        self.is_set(1)
    }

    fn round_xy_to_grid(&self) -> bool {
        self.is_set(2)
    }

    fn we_have_a_scale(&self) -> bool {
        self.is_set(3)
    }

    fn obsolete(&self) -> bool {
        self.is_set(4)
    }

    fn more_components(&self) -> bool {
        self.is_set(5)
    }

    fn we_have_an_x_and_y_scale(&self) -> bool {
        self.is_set(6)
    }

    fn we_have_a_two_by_two(&self) -> bool {
        self.is_set(7)
    }

    fn we_have_instructions(&self) -> bool {
        self.is_set(8)
    }

    fn use_my_metrics(&self) -> bool {
        self.is_set(9)
    }

    fn overlap_compound(&self) -> bool {
        self.is_set(10)
    }

    fn scaled_component_offset(&self) -> bool {
        self.is_set(11)
    }

    fn unscaled_component_offset(&self) -> bool {
        self.is_set(12)
    }
}
