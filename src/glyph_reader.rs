use crate::contours_reader::ContoursReader;
use crate::font_directory::FontDirectory;
use crate::model::{ArgumentTypes, ComponentData, Glyph, GlyphId};
use crate::reader::{FileOps, Reader, VecOps};
use crate::table::cmap_table::CMapSubtable;
use crate::table::head_table::HeadTable;
use crate::table::hhea_table::HheaTable;
use crate::table::htmx_table::LongHorMetricLookup;
use crate::table::loca_table::GlyphIdOffsetLookup;
use crate::table::maxp_table::MaximumProfileTable;
use crate::table::name_table;
use crate::Segment;

use std::fs::File;

pub struct GlyphReader {
    reader: Box<dyn Reader>,
    glyf_table_offset: u32,
    glyph_id_offset_lookup: GlyphIdOffsetLookup,
    cmap_subtable: CMapSubtable,
    long_hor_metric_lookup: LongHorMetricLookup,
    font_directory: FontDirectory,
    head_table: HeadTable,
}

impl GlyphReader {
    pub fn from_vec(v: Vec<u8>) -> GlyphReader {
        let vec_ops: VecOps = VecOps::from_vec(v);
        let reader: Box<dyn Reader> = Box::new(vec_ops);

        Self::from_reader(reader)
    }

    pub fn from_file(file: File) -> GlyphReader {
        let file_ops: FileOps = FileOps::from_file(file);
        let reader: Box<dyn Reader> = Box::new(file_ops);

        Self::from_reader(reader)
    }

    fn from_reader(mut reader: Box<dyn Reader>) -> GlyphReader {
        let font_directory: FontDirectory = FontDirectory::from_file(&mut reader);

        let loca_table = font_directory.table_directory("loca");
        let glyf_table = font_directory.table_directory("glyf");
        let cmap_table = font_directory.table_directory("cmap");
        let head_table = font_directory.table_directory("head");
        let maxp_table = font_directory.table_directory("maxp");
        let hhea_table = font_directory.table_directory("hhea");
        let htmx_table = font_directory.table_directory("hmtx");

        let head_table = HeadTable::from_file(&mut reader, head_table.offset);

        let maximum_profile_table = MaximumProfileTable::from_file(&mut reader, maxp_table.offset);

        let hhea_table = HheaTable::from_file(&mut reader, hhea_table.offset);

        let long_hor_metric_lookup = LongHorMetricLookup::from_file(
            &mut reader,
            htmx_table.offset,
            hhea_table,
            &maximum_profile_table,
        );

        let glyph_id_offset_lookup: GlyphIdOffsetLookup =
            GlyphIdOffsetLookup::mk_glyph_id_to_offset(
                &mut reader,
                loca_table.offset,
                &head_table,
                &maximum_profile_table,
            );

        let cmap_subtable: CMapSubtable = CMapSubtable::find_cmap_subtable(&mut reader, cmap_table);

        let glyf_table_offset = glyf_table.offset;

        GlyphReader {
            reader,
            glyf_table_offset,
            glyph_id_offset_lookup,
            cmap_subtable,
            long_hor_metric_lookup,
            font_directory,
            head_table,
        }
    }

    pub fn bounding_box(&self) -> (i16, i16, i16, i16) {
        (
            self.head_table.x_min.0,
            self.head_table.y_min.0,
            self.head_table.x_max.0,
            self.head_table.y_max.0,
        )
    }

    pub fn display_font_info(&mut self) {
        let name_table = self.font_directory.table_directory("name");

        name_table::read_name(&mut self.reader, name_table);
    }

    pub fn cmap_table_segments(&mut self) -> Vec<Segment> {
        self.cmap_subtable.segments(&mut self.reader)
    }

    pub fn char_code_to_glyph_id(&mut self, char_code: u16) -> GlyphId {
        self.cmap_subtable
            .find_glyph_id(&mut self.reader, char_code)
    }

    pub fn read_glyph(&mut self, char_code: u16) -> Glyph {
        let glyph_id = self.char_code_to_glyph_id(char_code);

        self.glyph_for_glyph_id(glyph_id)
    }

    pub fn all_char_codes(&mut self) -> Vec<u16> {
        let segments: Vec<Segment> = self.cmap_table_segments();
        segments
            .iter()
            .flat_map(|segment| (segment.start_code..=segment.end_code).collect::<Vec<u16>>())
            .collect::<Vec<u16>>()
    }

    pub fn glyph_for_glyph_id(&mut self, glyph_id: GlyphId) -> Glyph {
        let glyph_offset = self
            .glyph_id_offset_lookup
            .0
            .get(&glyph_id)
            .unwrap_or_else(|| panic!("{:?} not found in lookup map", glyph_id));

        let long_hor_metric = self
            .long_hor_metric_lookup
            .0
            .get(&glyph_id)
            .unwrap_or_else(|| {
                panic!("long_hor_metric for {:?} not found in lookup map", glyph_id)
            });

        let advance_width = long_hor_metric.advance_width;
        let left_side_bearing = long_hor_metric.left_side_bearing;

        if glyph_offset.is_empty() {
            Glyph::Empty {
                glyph_id,
                advance_width,
                left_side_bearing,
            }
        } else {
            self.reader
                .seek_from_start(self.glyf_table_offset + glyph_offset.offset());

            let number_of_contours = self.reader.read_i16();
            let x_min = self.reader.read_fword();
            let y_min = self.reader.read_fword();
            let x_max = self.reader.read_fword();
            let y_max = self.reader.read_fword();

            // if >= 0 it is a single glyph; if < 0 the glyph is compound
            if number_of_contours >= 0 {
                let mut contours_reader = ContoursReader::new(&mut self.reader);
                let simple_glyph = contours_reader.read_contours(number_of_contours);
                let contours = simple_glyph.contours;
                Glyph::Simple {
                    glyph_id,
                    x_min,
                    x_max,
                    y_min,
                    y_max,
                    advance_width,
                    left_side_bearing,
                    contours,
                }
            } else {
                let gc = GlyphComponent::new(&mut self.reader);

                let components: Vec<ComponentData> = gc.collect();

                Glyph::Compound {
                    glyph_id,
                    x_min,
                    x_max,
                    y_min,
                    y_max,
                    advance_width,
                    left_side_bearing,
                    components,
                }
            }
        }
    }
}

struct GlyphComponent<'a> {
    reader: &'a mut Box<dyn Reader>,
    has_more: bool,
}

impl<'a> GlyphComponent<'a> {
    fn new(reader: &'a mut Box<dyn Reader>) -> GlyphComponent<'a> {
        GlyphComponent {
            reader,
            has_more: true,
        }
    }
}

impl<'a> Iterator for GlyphComponent<'a> {
    type Item = ComponentData;

    fn next(&mut self) -> Option<Self::Item> {
        if self.has_more {
            let component_flag = ComponentFlag::from_file(self.reader);

            let glyph_index = self.reader.read_u16();
            let argument_types = if component_flag.arg1_and_arg2_are_words() {
                if component_flag.args_are_xy_values() {
                    let arg1 = self.reader.read_i16();
                    let arg2 = self.reader.read_i16();
                    ArgumentTypes::XYValue16(arg1, arg2)
                } else {
                    let arg1 = self.reader.read_u16();
                    let arg2 = self.reader.read_u16();
                    ArgumentTypes::Point16(arg1, arg2)
                }
            } else if component_flag.args_are_xy_values() {
                let arg1 = self.reader.read_i8();
                let arg2 = self.reader.read_i8();
                ArgumentTypes::XYValue8(arg1, arg2)
            } else {
                let arg1 = self.reader.read_u8();
                let arg2 = self.reader.read_u8();
                ArgumentTypes::Point8(arg1, arg2)
            };

            let (a, b, c, d) = if component_flag.we_have_a_scale() {
                let scale = self.reader.read_i16();
                (scale, 0, 0, scale)
            } else if component_flag.we_have_an_x_and_y_scale() {
                let x_scale = self.reader.read_i16();
                let y_scale = self.reader.read_i16();
                (x_scale, 0, 0, y_scale)
            } else if component_flag.we_have_a_two_by_two() {
                let x_scale = self.reader.read_i16();
                let scale_01 = self.reader.read_i16();
                let scale_10 = self.reader.read_i16();
                let y_scale = self.reader.read_i16();
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
    fn from_file(reader: &mut Box<dyn Reader>) -> ComponentFlag {
        ComponentFlag(reader.read_u16())
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
