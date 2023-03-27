use crate::contours_reader::ContoursReader;
use crate::file_ops::FileOps;
use crate::model::{ArgumentTypes, ComponentData, GlyphId, GlyphType};

pub struct GlyphReader<'a> {
    file_ops: &'a mut FileOps,
    offset: u32,
}

impl<'a> GlyphReader<'a> {
    pub fn new(file_ops: &mut FileOps, offset: u32, glyph_offset: u32) -> GlyphReader {
        let offset = offset + glyph_offset;

        GlyphReader { file_ops, offset }
    }

    pub fn read_glyph(&mut self, glyph_id: GlyphId) -> GlyphType {
        self.file_ops.seek_from_start(self.offset);

        let number_of_contours = self.file_ops.read_i16();
        let x_min = self.file_ops.read_fword();
        let y_min = self.file_ops.read_fword();
        let x_max = self.file_ops.read_fword();
        let y_max = self.file_ops.read_fword();

        // if >= 0 it is a single glyph; if < 0 the glyph is compound
        println!("number_of_contours >>> {}", number_of_contours);
        if number_of_contours >= 0 {
            let mut contours_reader = ContoursReader::new(self.file_ops);
            let simple_glyph = contours_reader.read_contours(number_of_contours);
            let contours = simple_glyph.contours;
            GlyphType::SimpleGlyph {
                glyph_id,
                x_min,
                x_max,
                y_min,
                y_max,
                contours,
            }
        } else {
            let gc = GlyphComponent::new(self.file_ops);

            let components: Vec<ComponentData> = gc.collect();

            GlyphType::CompountGlyph { components }
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
