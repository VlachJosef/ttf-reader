use crate::file_ops::FileOps;
use crate::font_directory::FontDirectory;
use crate::glyph_index_lookup::GlyphIndexLookup;
use crate::glyph_reader::GlyphReader;
use crate::model::{FWord, Fixed, Glyph, GlyphId};
use std::fs::File;

mod contours_reader;
mod file_ops;
mod font_directory;
mod glyph_index_lookup;
mod glyph_reader;
mod model;
mod name_table;

// https://developer.apple.com/fonts/TrueType-Reference-Manual/RM06/Chap6.html
// https://learn.microsoft.com/en-us/typography/opentype/spec/
// http://formats.kaitai.io/ttf/ttf.svg

#[allow(unused)]
#[derive(Debug)]
struct MaximumProfileTable {
    version: Fixed,                // 0x00010000 (1.0)
    num_glyphs: u16,               // the number of glyphs in the font
    max_points: u16,               // points in non-compound glyph
    max_contours: u16,             // contours in non-compound glyph
    max_component_points: u16,     // points in compound glyph
    max_component_contours: u16,   // contours in compound glyph
    max_zones: u16,                // set to 2
    max_twilight_points: u16,      // points used in Twilight Zone (Z0)
    max_storage: u16,              // number of Storage Area locations
    max_function_defs: u16,        // number of FDEFs
    max_instruction_defs: u16,     // number of IDEFs
    max_stack_elements: u16,       // maximum stack depth
    max_size_of_instructions: u16, // byte count for glyph instructions
    max_component_elements: u16,   // number of glyphs referenced at top level
    max_component_depth: u16,      // levels of recursion, set to 0 if font has only simple glyphs
}

impl MaximumProfileTable {
    fn from_file(file_ops: &mut FileOps, offset: u32) -> MaximumProfileTable {
        file_ops.seek_from_start(offset);
        let version: Fixed = file_ops.read_fixed();
        let num_glyphs: u16 = file_ops.read_u16();
        let max_points: u16 = file_ops.read_u16();
        let max_contours: u16 = file_ops.read_u16();
        let max_component_points: u16 = file_ops.read_u16();
        let max_component_contours: u16 = file_ops.read_u16();
        let max_zones: u16 = file_ops.read_u16();
        let max_twilight_points: u16 = file_ops.read_u16();
        let max_storage: u16 = file_ops.read_u16();
        let max_function_defs: u16 = file_ops.read_u16();
        let max_instruction_defs: u16 = file_ops.read_u16();
        let max_stack_elements: u16 = file_ops.read_u16();
        let max_size_of_instructions: u16 = file_ops.read_u16();
        let max_component_elements: u16 = file_ops.read_u16();
        let max_component_depth: u16 = file_ops.read_u16();
        MaximumProfileTable {
            version,
            num_glyphs,
            max_points,
            max_contours,
            max_component_points,
            max_component_contours,
            max_zones,
            max_twilight_points,
            max_storage,
            max_function_defs,
            max_instruction_defs,
            max_stack_elements,
            max_size_of_instructions,
            max_component_elements,
            max_component_depth,
        }
    }
}

#[derive(Debug)]
struct IndexToLocTable {
    data: Vec<u32>,
}

impl IndexToLocTable {
    fn index_for(&self, glyph_id: &GlyphId) -> u32 {
        self.data[glyph_id.id() as usize]
    }

    fn mk_index_to_loc_table(
        file_ops: &mut FileOps,
        offset: u32,
        head_table: HeadTable,
        maximum_profile_table: MaximumProfileTable,
    ) -> IndexToLocTable {
        file_ops.seek_from_start(offset);
        let data: Vec<u32> = match head_table.index_to_loc_format {
            0 => todo!("                       SHORT"),
            1 => (0..maximum_profile_table.num_glyphs)
                .map(|_| file_ops.read_u32())
                .collect(),
            _ => unreachable!("Only 0 and 1 is supported per specification"),
        };

        IndexToLocTable { data }
    }
}

#[allow(unused)]
#[derive(Debug)]
struct HeadTable {
    version: Fixed,
    font_revision: Fixed,
    flags: u16,
    units_per_em: u16,
    created: i64,
    modified: i64,
    x_min: FWord,
    y_min: FWord,
    x_max: FWord,
    y_max: FWord,
    mac_style: u16,
    lowest_rec_ppem: u16, // smallest readable size in pixels
    font_direction_hint: i16,
    index_to_loc_format: i16, // 0 for short offsets, 1 for long
    glyph_data_format: i16,
}

impl HeadTable {
    fn from_file(file_ops: &mut FileOps, offset: u32) -> HeadTable {
        file_ops.seek_from_start(offset);

        let version = file_ops.read_fixed();
        let font_revision = file_ops.read_fixed();

        let _checksum = file_ops.read_u32();
        let _magic_number = file_ops.read_u32(); // Must be 0x5F0F3CF5
        let flags = file_ops.read_u16();
        let units_per_em = file_ops.read_u16();

        let created = file_ops.read_long_date_time();
        let modified = file_ops.read_long_date_time();

        let x_min = file_ops.read_fword();
        let y_min = file_ops.read_fword();
        let x_max = file_ops.read_fword();
        let y_max = file_ops.read_fword();

        let mac_style = file_ops.read_u16();
        let lowest_rec_ppem = file_ops.read_u16();
        let font_direction_hint = file_ops.read_i16();
        let index_to_loc_format = file_ops.read_i16();
        let glyph_data_format = file_ops.read_i16();

        HeadTable {
            version,
            font_revision,
            flags,
            units_per_em,
            created,
            modified,
            x_min,
            y_min,
            x_max,
            y_max,
            mac_style,
            lowest_rec_ppem, // smallest readable size in pixels
            font_direction_hint,
            index_to_loc_format, // 0 for short offsets, 1 for long
            glyph_data_format,
        }
    }
}

struct CMapSubtable {}

#[allow(unused)]
#[derive(Debug)]
struct Segment {
    glyph_id: GlyphId,
    start_code: u16,
    end_code: u16,
    id_delta: u16,
    id_range_offset: u16,
}

impl CMapSubtable {
    fn find_char_code_segment(file_ops: &mut FileOps, char_code: u16) -> GlyphId {
        let subtable_format = file_ops.read_u16();
        let length = file_ops.read_u16();
        let version = file_ops.read_u16();

        let seg_count_x2 = file_ops.read_u16(); // The segCount is the number of contiguous code ranges in the font
        let search_range = file_ops.read_u16(); // TODO compute this from seg_count_x2
        let entry_selector = file_ops.read_u16(); // TODO compute this from seg_count_x2
        let range_shift = file_ops.read_u16(); // Do not use

        println!("[CMapSubtable] subtable_format {:?}", subtable_format);
        println!("[CMapSubtable] length {:?}", length);
        println!("[CMapSubtable] version {:?}", version);
        println!("[CMapSubtable] seg_count_x2 {:?}", seg_count_x2);
        //println!("[CMapSubtable] seg_count    {:?}", seg_count);
        println!("[CMapSubtable] search_range {:?}", search_range);
        println!("[CMapSubtable] entry_selector {:?}", entry_selector);
        println!("[CMapSubtable] range_shift {:?}", range_shift);

        let mut index_lookup = GlyphIndexLookup::new(file_ops, seg_count_x2, char_code);

        index_lookup.seek_glyph_id(search_range, entry_selector)
    }
}

// Character to glyph index mapping
struct CMapTable {}

impl CMapTable {
    fn mk_cmap_table(file_ops: &mut FileOps, offset: u32, char_code: u16) -> GlyphId {
        file_ops.seek_from_start(offset);
        let version = file_ops.read_u16();
        let number_subtables = file_ops.read_u16();
        println!("[CMapTable] version {}", version);
        println!("[CMapTable] number_subtables {}", number_subtables);

        for i in 0..number_subtables {
            let platform_id = file_ops.read_platform_id();
            let platform_specific_id = file_ops.read_u16();
            let offset = file_ops.read_u32();

            println!("[CMapTable] platform_id_{} {:?}", i, platform_id);
            println!(
                "[CMapTable] platform_specific_id_{} {}",
                i, platform_specific_id
            );
            println!("[CMapTable] offset_{} {}", i, offset);
        }

        // TODO select the best table. (Is it just a coincidence that Unicode table follows first?)

        CMapSubtable::find_char_code_segment(file_ops, char_code)
    }
}

pub fn read_font_file(char_code: u16) -> Glyph {
    let file_path = "fonts/Monaco.ttf";

    let file: File = File::open(file_path).expect("Should been able to open the file");

    let mut file_ops: FileOps = FileOps::from_file(file);

    let font_directory: FontDirectory = FontDirectory::from_file(&mut file_ops);

    let loca_table = font_directory.table_directory("loca");
    let glyf_table = font_directory.table_directory("glyf");
    let cmap_table = font_directory.table_directory("cmap");
    let head_table = font_directory.table_directory("head");
    let maxp_table = font_directory.table_directory("maxp");
    let name_table = font_directory.table_directory("name");

    name_table::read_name(&mut file_ops, name_table);

    let head_table = HeadTable::from_file(&mut file_ops, head_table.offset);

    let maximum_profile_table = MaximumProfileTable::from_file(&mut file_ops, maxp_table.offset);

    let index_to_loc_table: IndexToLocTable = IndexToLocTable::mk_index_to_loc_table(
        &mut file_ops,
        loca_table.offset,
        head_table,
        maximum_profile_table,
    );

    let glyph_id = CMapTable::mk_cmap_table(&mut file_ops, cmap_table.offset, char_code);

    let glyph_offset = index_to_loc_table.index_for(&glyph_id);

    println!("glyph_id {:?} glyph_offset {} ", glyph_id, glyph_offset);
    let mut glyph_reader = GlyphReader::new(&mut file_ops, glyf_table.offset, glyph_offset);

    let glyph_table_content = glyph_reader.read_glyph(glyph_id);
    println!("glyph_offset {:?} ", glyph_offset);
    println!("glyph_table_content {:?}", glyph_table_content);

    glyph_table_content
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{ArgumentTypes, Contour, Point, PointType};

    macro_rules! mk_contour {
        ($($x:literal, $y:literal - $tpe:ident),*) => {{
            Contour {
                points: vec![
                    $(
                        Point {
                            x: $x,
                            y: $y,
                            tpe: PointType::$tpe,
                        },
                    )*
                ],
            }
        }};
    }

    #[test]
    fn test_char_exclamation_mark() {
        let result = read_font_file(b'!' as u16);

        match result {
            Glyph::Compount { .. } => panic!("Unexpected glyph type"),
            Glyph::Simple {
                glyph_id,
                x_min,
                x_max,
                y_min,
                y_max,
                contours,
            } => {
                assert_eq!(glyph_id.id(), 4);
                assert_eq!(x_min.0, 471);
                assert_eq!(x_max.0, 758);
                assert_eq!(y_min.0, -47);
                assert_eq!(y_max.0, 1552);

                #[rustfmt::skip]
                let expected_contours = vec![
                    mk_contour!(
                        529,  442 - OnCurve,
                        483, 1552 - OnCurve,
                        731, 1552 - OnCurve,
                        684,  442 - OnCurve
                    ),
                    mk_contour!(
                        615, -47 - OnCurve,
                        556, -47 - Control,
                        471,  38 - Control,
                        471,  97 - OnCurve,
                        471, 156 - Control,
                        556, 240 - Control,
                        615, 240 - OnCurve,
                        673, 240 - Control,
                        758, 157 - Control,
                        758,  97 - OnCurve,
                        758,  37 - Control,
                        672, -47 - Control
                    )];

                assert_eq!(contours, expected_contours);
            }
        }

        //panic!("");
    }

    #[test]
    fn test_char_a() {
        let result = read_font_file(b'a' as u16);

        match result {
            Glyph::Compount { .. } => panic!("Unexpected glyph type"),
            Glyph::Simple {
                glyph_id,
                x_min,
                x_max,
                y_min,
                y_max,
                contours,
            } => {
                assert_eq!(glyph_id.id(), 68);
                assert_eq!(x_min.0, 102);
                assert_eq!(x_max.0, 1118);
                assert_eq!(y_min.0, -31);
                assert_eq!(y_max.0, 1133);

                #[rustfmt::skip]
                let expected_contours = vec![
                    mk_contour!(
                        889,  251 - OnCurve,
                        804,  111 - Control,
                        565,  -31 - Control,
                        465,  -31 - OnCurve,
                        325,  -31 - Control,
                        102,  211 - Control,
                        102,  457 - OnCurve,
                        102,  651 - Control,
                        250,  956 - Control,
                        498, 1133 - Control,
                        681, 1133 - OnCurve,
                        734, 1133 - Control,
                        841, 1122 - OnCurve,
                        857, 1120 - Control,
                        889, 1117 - OnCurve,
                        1076, 1117 - OnCurve,
                        1076,  328 - OnCurve,
                        1076,  126 - Control,
                        1118,    0 - OnCurve,
                        924,    0 - OnCurve,
                        902,   92 - Control
                    ),
                    mk_contour!(
                        889, 472 - OnCurve,
                        889, 951 - OnCurve,
                        790, 977 - Control,
                        705, 977 - OnCurve,
                        533, 977 - Control,
                        304, 711 - Control,
                        304, 482 - OnCurve,
                        304, 316 - Control,
                        429, 147 - Control,
                        509, 147 - OnCurve,
                        596, 147 - Control,
                        806, 323 - Control
                    )];
                assert_eq!(contours, expected_contours);
            }
        }
    }

    #[test]
    fn test_char_aacute() {
        let result = read_font_file('á' as u16);

        match result {
            Glyph::Compount { components } => {
                assert_eq!(components.len(), 2);

                let c1 = &components[0];
                let c2 = &components[1];

                assert_eq!(c1.glyph_index, 68);
                assert_eq!(c1.a, 1);
                assert_eq!(c1.b, 0);
                assert_eq!(c1.c, 0);
                assert_eq!(c1.d, 1);
                assert_eq!(c1.argument_types, ArgumentTypes::XYValue8(0, 0));
                assert_eq!(c2.glyph_index, 141);
                assert_eq!(c2.a, 1);
                assert_eq!(c2.b, 0);
                assert_eq!(c2.c, 0);
                assert_eq!(c2.d, 1);
                assert_eq!(c2.argument_types, ArgumentTypes::XYValue16(159, 0));
            }

            Glyph::Simple { .. } => panic!("Unexpected glyph type"),
        }
    }

    #[test]
    fn test_char_medium_shade() {
        let result = read_font_file('▒' as u16);

        match result {
            Glyph::Compount { .. } => panic!("Unexpected glyph type"),
            Glyph::Simple {
                glyph_id,
                x_min,
                x_max,
                y_min,
                y_max,
                contours,
            } => {
                assert_eq!(glyph_id.id(), 1676);
                assert_eq!(x_min.0, 1);
                assert_eq!(x_max.0, 1024);
                assert_eq!(y_min.0, -202);
                assert_eq!(y_max.0, 1555);

                #[rustfmt::skip]
                let expected_contours = vec![
                    mk_contour!(
                        773, 1261 - OnCurve,
                        520, 1262 - OnCurve,
                        521, 1553 - OnCurve,
                        772, 1555 - OnCurve
                    ),
                    mk_contour!(
                        252, 1262 - OnCurve,
                        1, 1262 - OnCurve,
                        1, 1555 - OnCurve,
                        254, 1553 - OnCurve
                    ),
                    mk_contour!(
                        515, 969 - OnCurve,
                        264, 969 - OnCurve,
                        264, 1262 - OnCurve,
                        516, 1261 - OnCurve
                    ),
                    mk_contour!(
                        1024, 968 - OnCurve,
                        771, 969 - OnCurve,
                        772, 1261 - OnCurve,
                        1022, 1262 - OnCurve
                    ),
                    mk_contour!(
                        773, 676 - OnCurve,
                        520, 677 - OnCurve,
                        521, 968 - OnCurve,
                        772, 969 - OnCurve
                    ),
                    mk_contour!(
                        252, 677 - OnCurve,
                        1, 677 - OnCurve,
                        1, 969 - OnCurve,
                        254, 968 - OnCurve
                    ),
                    mk_contour!(
                        515, 385 - OnCurve,
                        264, 385 - OnCurve,
                        264, 677 - OnCurve,
                        516, 676 - OnCurve
                    ),
                    mk_contour!(
                        1024, 384 - OnCurve,
                        771, 385 - OnCurve,
                        772, 676 - OnCurve,
                        1022, 677 - OnCurve
                    ),
                    mk_contour!(
                        773, 91 - OnCurve,
                        520, 92 - OnCurve,
                        521, 384 - OnCurve,
                        772, 385 - OnCurve
                    ),
                    mk_contour!(
                        252, 92 - OnCurve,
                        1, 92 - OnCurve,
                        1, 385 - OnCurve,
                        254, 384 - OnCurve
                    ),
                    mk_contour!(
                        515, -200 - OnCurve,
                        264, -200 - OnCurve,
                        264, 92 - OnCurve,
                        516, 91 - OnCurve
                    ),
                    mk_contour!(
                        1024, -202 - OnCurve,
                        771, -200 - OnCurve,
                        772, 91 - OnCurve,
                        1022, 92 - OnCurve
                    )
                ];

                assert_eq!(contours, expected_contours);
            }
        }
    }
}
