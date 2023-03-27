use std::fs::File;
use std::io::prelude::*;
use std::io::SeekFrom;

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
    fn from_file(mut file: &File, offset: u64) -> MaximumProfileTable {
        file.seek(SeekFrom::Start(offset))
            .expect("Expected be able to seek");
        let version: Fixed = read_fixed(file);
        let num_glyphs: u16 = read_u16(file);
        let max_points: u16 = read_u16(file);
        let max_contours: u16 = read_u16(file);
        let max_component_points: u16 = read_u16(file);
        let max_component_contours: u16 = read_u16(file);
        let max_zones: u16 = read_u16(file);
        let max_twilight_points: u16 = read_u16(file);
        let max_storage: u16 = read_u16(file);
        let max_function_defs: u16 = read_u16(file);
        let max_instruction_defs: u16 = read_u16(file);
        let max_stack_elements: u16 = read_u16(file);
        let max_size_of_instructions: u16 = read_u16(file);
        let max_component_elements: u16 = read_u16(file);
        let max_component_depth: u16 = read_u16(file);
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

#[allow(unused)]
#[derive(Debug)]
struct OffsetSubtable {
    scaler_type: u32, // A tag to indicate the OFA scaler to be used to rasterize this font
    num_tables: u16,  // number of tables
    search_range: u16, // (maximum power of 2 <= numTables)*16
    entry_selector: u16, // log2(maximum power of 2 <= numTables)
    range_shift: u16, // numTables*16-searchRang
}

impl OffsetSubtable {
    fn from_file(file: &File) -> OffsetSubtable {
        let scaler_type = read_u32(file);
        let num_tables = read_u16(file);
        let search_range = read_u16(file);
        let entry_selector = read_u16(file);
        let range_shift = read_u16(file);

        OffsetSubtable {
            scaler_type,
            num_tables,
            search_range,
            entry_selector,
            range_shift,
        }
    }
}

#[allow(unused)]
#[derive(Debug)]
struct TableDirectory {
    tag: String,   // 4-byte identifier
    checksum: u32, // checksum for this table
    offset: u32,   // offset from beginning of sfnt
    length: u32,   // length of this table in byte (actual length not padded length)
}

impl TableDirectory {
    fn from_file(file: &File) -> TableDirectory {
        let tag: String = read_table_name(file);
        let checksum = read_u32(file);
        let offset = read_u32(file);
        let length = read_u32(file);

        TableDirectory {
            tag,
            checksum,
            offset,
            length,
        }
    }
}

#[derive(Debug)]
struct IndexToLocTable {
    data: Vec<u32>,
}

impl IndexToLocTable {
    fn index_for(&self, glyph_id: &GlyphId) -> u32 {
        self.data[glyph_id.0 as usize]
    }

    fn mk_index_to_loc_table(
        mut file: &File,
        offset: u64,
        head_table: HeadTable,
        maximum_profile_table: MaximumProfileTable,
    ) -> IndexToLocTable {
        file.seek(SeekFrom::Start(offset))
            .expect("Expected be able to seek");
        let data: Vec<u32> = match head_table.index_to_loc_format {
            0 => todo!("                       SHORT"),
            1 => (0..maximum_profile_table.num_glyphs)
                .map(|_| read_u32(file))
                .collect(),
            _ => unreachable!("Only 0 and 1 is supported per specification"),
        };

        IndexToLocTable { data }
    }
}

#[derive(Debug, PartialEq)]
enum ArgumentTypes {
    XYValue16(i16, i16),
    Point16(u16, u16),
    XYValue8(i8, i8),
    Point8(u8, u8),
}

#[allow(unused)]
#[derive(Debug)]
pub struct ComponentData {
    glyph_index: u16,
    a: i16,
    b: i16,
    c: i16,
    d: i16,
    argument_types: ArgumentTypes, // encapsulates e, f
}

#[derive(Debug)]
struct GlyphTable {}

struct GlyphComponent<'a> {
    file: &'a File,
    has_more: bool,
}

impl<'a> GlyphComponent<'a> {
    fn new(file: &'a File) -> GlyphComponent<'a> {
        GlyphComponent {
            file,
            has_more: true,
        }
    }
}

impl<'a> Iterator for GlyphComponent<'a> {
    type Item = ComponentData;

    fn next(&mut self) -> Option<Self::Item> {
        if self.has_more {
            let file = &self.file;
            let component_flag = ComponentFlag::from_file(file);
            //component_flag.pretty_print();
            let glyph_index = read_u16(file);
            let argument_types = if component_flag.arg1_and_arg2_are_words() {
                if component_flag.args_are_xy_values() {
                    let arg1 = read_i16(file);
                    let arg2 = read_i16(file);
                    ArgumentTypes::XYValue16(arg1, arg2)
                } else {
                    let arg1 = read_u16(file);
                    let arg2 = read_u16(file);
                    ArgumentTypes::Point16(arg1, arg2)
                }
            } else {
                if component_flag.args_are_xy_values() {
                    let arg1 = read_i8(file);
                    let arg2 = read_i8(file);
                    ArgumentTypes::XYValue8(arg1, arg2)
                } else {
                    let arg1 = read_u8(file);
                    let arg2 = read_u8(file);
                    ArgumentTypes::Point8(arg1, arg2)
                }
            };

            let (a, b, c, d) = if component_flag.we_have_a_scale() {
                let scale = read_i16(file);
                (scale, 0, 0, scale)
            } else if component_flag.we_have_an_x_and_y_scale() {
                let x_scale = read_i16(file);
                let y_scale = read_i16(file);
                (x_scale, 0, 0, y_scale)
            } else if component_flag.we_have_a_two_by_two() {
                let x_scale = read_i16(file);
                let scale_01 = read_i16(file);
                let scale_10 = read_i16(file);
                let y_scale = read_i16(file);
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

impl GlyphTable {
    fn read_glyph(file: &File, glyph_id: GlyphId) -> GlyphData {
        let number_of_contours = read_i16(file);
        let x_min = read_fword(file);
        let y_min = read_fword(file);
        let x_max = read_fword(file);
        let y_max = read_fword(file);

        // if >= 0 it is a single glyph; if < 0 the glyph is compound
        println!("number_of_contours >>> {}", number_of_contours);
        if number_of_contours >= 0 {
            let simple_glyph = SimpleGlyph::mk_simple_glyph(file, number_of_contours);
            let contours = simple_glyph.contours;
            GlyphData::SimpleGlyph {
                glyph_id,
                x_min,
                x_max,
                y_min,
                y_max,
                contours,
            }
        } else {
            let gc = GlyphComponent::new(file);

            let aaa: Vec<ComponentData> = gc.collect();

            GlyphData::CompountGlyph { components: aaa }

            //println!("-------------------- a: {:?}", aaa);

            //todo!("Implement compount glyph ")
        }
    }

    fn mk_glyph_table(
        mut file: &File,
        offset: u64,
        glyph_offset: u32,
        glyph_id: GlyphId,
    ) -> GlyphData {
        file.seek(SeekFrom::Start(offset + glyph_offset as u64))
            .expect("Expected be able to seek");

        Self::read_glyph(file, glyph_id)
    }
}

#[allow(unused)]
#[derive(Debug)]
struct SimpleGlyph {
    instruction_length: u16,
    instructions: Vec<u8>,
    contours: Vec<Contour>,
}

#[allow(unused)]
#[derive(Debug)]
struct ContourFlags {
    contour_flags: Vec<ControlPointsFlags>,
}

impl ContourFlags {
    fn mk_contour_flags(file: &File, end_pts_of_contours: Vec<u16>) -> ContourFlags {
        let last = end_pts_of_contours.last().unwrap();
        println!("last                {:?}", last);

        let contour_flags_total: Vec<ControlPointsFlags> = Self::_mk_contour_flags(file, *last + 1);

        ContourFlags {
            contour_flags: contour_flags_total,
        }
    }
    fn _mk_contour_flags(file: &File, mut number_of_points: u16) -> Vec<ControlPointsFlags> {
        let mut contour_flags: Vec<ControlPointsFlags> =
            Vec::with_capacity((number_of_points + 1) as usize);
        while number_of_points > 0 {
            let control_points = ControlPointsFlags::from_file(file);
            if control_points.repeat() {
                // If repeat is set, the next byte specifies the number of additional times this set of flags is to be repeated.
                let mut repeat_times = read_u8(file);

                while repeat_times > 0 {
                    contour_flags.push(control_points);
                    repeat_times -= 1;
                    number_of_points -= 1;
                }
            }
            contour_flags.push(control_points);
            number_of_points -= 1;
        }
        contour_flags
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum PointType {
    OnCurve,
    Control,
}

impl PointType {
    fn from(flags: ControlPointsFlags) -> PointType {
        if flags.on_curve() {
            PointType::OnCurve
        } else {
            PointType::Control
        }
    }
}

#[derive(Debug, Copy, Clone)]
struct ComponentFlag(u16);

impl ComponentFlag {
    fn from_file(file: &File) -> ComponentFlag {
        ComponentFlag(read_u16(file))
    }

    #[allow(unused)]
    fn pretty_print(&self) {
        println!(
            "arg1_and_arg2_are_words  {}",
            self.arg1_and_arg2_are_words()
        );
        println!("args_are_xy_values       {}", self.args_are_xy_values());
        println!("round_xy_to_grid         {}", self.round_xy_to_grid());
        println!("we_have_a_scale          {}", self.we_have_a_scale());
        println!("obsolete                 {}", self.obsolete());
        println!("more_components          {}", self.more_components());
        println!(
            "we_have_an_x_and_y_scale {}",
            self.we_have_an_x_and_y_scale()
        );
        println!("we_have_a_two_by_two     {}", self.we_have_a_two_by_two());
        println!("we_have_instructions     {}", self.we_have_instructions());
        println!("use_my_metrics           {}", self.use_my_metrics());
        println!("overlap_compound         {}", self.overlap_compound());
        println!(
            "scaled_component_offset  {}",
            self.scaled_component_offset()
        );
        println!(
            "unscaled_component_offset{}",
            self.unscaled_component_offset()
        );
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

#[derive(Debug, Copy, Clone)]
struct ControlPointsFlags(u8);

impl ControlPointsFlags {
    fn from_file(file: &File) -> ControlPointsFlags {
        ControlPointsFlags(read_u8(file))
    }

    fn is_set(&self, bit: u8) -> bool {
        let shift = 1 << bit;
        self.0 & shift == shift
    }

    fn on_curve(&self) -> bool {
        self.is_set(0)
    }

    fn x_short_vector(&self) -> bool {
        self.is_set(1)
    }

    fn y_short_vector(&self) -> bool {
        self.is_set(2)
    }

    fn repeat(&self) -> bool {
        self.is_set(3)
    }

    fn x_is_same(&self) -> bool {
        self.is_set(4)
    }

    fn y_is_same(&self) -> bool {
        self.is_set(5)
    }

    #[allow(unused)]
    fn pretty_print(&self, id: &str) {
        println!("[{}], on_curve      : {:?}", id, self.on_curve());
        println!("[{}], x_short_vector: {:?}", id, self.x_short_vector());
        println!("[{}], y_short_vector: {:?}", id, self.y_short_vector());
        println!("[{}], repeat        : {:?}", id, self.repeat());
        println!("[{}], x_is_same     : {:?}", id, self.x_is_same());
        println!("[{}], y_is_same     : {:?}", id, self.y_is_same());
    }
}

#[derive(Debug)]
pub enum GlyphData {
    SimpleGlyph {
        glyph_id: GlyphId,
        x_min: FWord, // Minimum x for coordinate data
        y_min: FWord, // Minimum y for coordinate data
        x_max: FWord, // Maximum x for coordinate data
        y_max: FWord, // Maximum y for coordinate data
        contours: Vec<Contour>,
    },
    CompountGlyph {
        components: Vec<ComponentData>,
    },
}

#[derive(Debug, PartialEq)]
pub struct Contour {
    points: Vec<Point>,
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct Point {
    x: i16,
    y: i16,
    tpe: PointType,
}

impl Point {
    fn new(x: i16, y: i16, tpe: PointType) -> Point {
        Point { x, y, tpe }
    }
}

impl SimpleGlyph {
    fn read_coordinates(
        file: &File,
        flags: &ContourFlags,
        is_short_vector: fn(&ControlPointsFlags) -> bool,
        is_same: fn(&ControlPointsFlags) -> bool,
    ) -> Vec<i16> {
        let mut coordinates = vec![];
        let mut last_elem = 0;

        flags.contour_flags.iter().for_each(|contour_flag| {
            let coordinate = if is_short_vector(contour_flag) {
                let coor = read_u8(file) as i16;
                if is_same(contour_flag) {
                    coor
                } else {
                    -coor
                }
            } else {
                if is_same(contour_flag) {
                    0
                } else {
                    read_i16(file)
                }
            };
            let coordinate = last_elem + coordinate;
            coordinates.push(coordinate);
            last_elem = coordinate;
        });

        coordinates
    }

    fn mk_simple_glyph(file: &File, n: i16) -> SimpleGlyph {
        let mut end_pts_of_contours: Vec<u16> =
            (0..n).into_iter().map(|_| read_u16(file)).collect();
        let instruction_length: u16 = read_u16(file);

        println!("SOURCE end_pts_of_contours {:?}", end_pts_of_contours);

        let instructions = (0..instruction_length)
            .into_iter()
            .map(|_| read_u8(file))
            .collect();

        end_pts_of_contours.insert(0, 0);

        // The number of points is determined by the last entry in the end_pts_of_contours array.

        let flags: ContourFlags = ContourFlags::mk_contour_flags(file, end_pts_of_contours.clone());
        println!("HERE flags {:?}", flags);

        let x_coordinates = SimpleGlyph::read_coordinates(
            file,
            &flags,
            |cf| cf.x_short_vector(),
            |cf| cf.x_is_same(),
        );
        let y_coordinates = SimpleGlyph::read_coordinates(
            file,
            &flags,
            |cf| cf.y_short_vector(),
            |cf| cf.y_is_same(),
        );

        println!("x_coordinates {:?}", x_coordinates);
        println!("y_coordinates {:?}", y_coordinates);
        //println!("        flags {:?}", flags.len());
        let mut points: Vec<Point> = x_coordinates
            .into_iter()
            .zip(y_coordinates)
            .zip(flags.contour_flags)
            .map(|((x, y), flags)| Point::new(x, y, PointType::from(flags)))
            .collect();

        println!("             points {:?}", points);

        let wins: Vec<&[u16]> = end_pts_of_contours.windows(2).collect();
        let points_per_contours: Vec<u16> = wins
            .into_iter()
            .map(|window| {
                let s = window[0];
                let e = window[1];

                if s == 0 {
                    e + 1
                } else {
                    e - s
                }
            })
            .collect();

        println!("points_per_contours {:?}", points_per_contours);

        let contours: Vec<Contour> = points_per_contours
            .iter()
            .map(|ppc| {
                let size = *ppc as usize;
                //let taken = points.split_off(size);
                let taken = points.splice(0..size, []).collect();
                Contour { points: taken }
            })
            .collect();

        SimpleGlyph {
            instruction_length,
            instructions,
            contours,
        }
    }
}

#[allow(unused)]
#[derive(Debug)]
struct Fixed {
    major: u16,
    minor: u16,
}

#[derive(Debug)]
pub struct FWord(i16);

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
    fn from_file(mut file: &File, offset: u64) -> HeadTable {
        file.seek(SeekFrom::Start(offset))
            .expect("Expected be able to seek");

        let version = read_fixed(file);
        let font_revision = read_fixed(file);

        let _checksum = read_u32(file);
        let _magic_number = read_u32(file); // Must be 0x5F0F3CF5
        let flags = read_u16(file);
        let units_per_em = read_u16(file);

        let created = read_long_date_time(file);
        let modified = read_long_date_time(file);

        let x_min = read_fword(file);
        let y_min = read_fword(file);
        let x_max = read_fword(file);
        let y_max = read_fword(file);

        let mac_style = read_u16(file);
        let lowest_rec_ppem = read_u16(file);
        let font_direction_hint = read_i16(file);
        let index_to_loc_format = read_i16(file);
        let glyph_data_format = read_i16(file);

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

#[derive(Debug)]
enum PlatformId {
    Unicode,
    Macintosh,
    Reserved,
    Microsoft,
}

#[derive(Debug)]
pub struct GlyphId(u16);

impl GlyphId {
    const MISSING_CHARACTER_GLYPH: GlyphId = GlyphId(0);
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

struct FileOps<'a> {
    file: &'a File,
}

impl<'a> FileOps<'a> {
    fn seek(&mut self, seek_from: SeekFrom) {
        self.file.seek(seek_from).expect("Expected be able to seek");
    }

    fn read_end_code(&mut self, current_search_range: i64) -> u16 {
        self.seek(SeekFrom::Current(current_search_range));
        let end_code = read_u16(self.file);

        // Reset last read offset shift
        self.seek(SeekFrom::Current(-2));

        end_code
    }
    fn read_start_code(&mut self, seg_count_x2: u16) -> u16 {
        self.seek(SeekFrom::Current(2)); // Skip reservedPad
        self.seek(SeekFrom::Current(seg_count_x2 as i64));

        let start_code = read_u16(self.file);

        // Reset last read offset shift
        self.seek(SeekFrom::Current(-2));

        start_code
    }

    fn read_id_delta(&mut self, seg_count_x2: u16) -> u16 {
        self.seek(SeekFrom::Current(seg_count_x2 as i64));

        let id_delta = read_u16(self.file);

        self.seek(SeekFrom::Current(-2));

        id_delta
    }

    fn read_id_range_offset(&mut self, seg_count_x2: u16) -> u16 {
        self.seek(SeekFrom::Current(seg_count_x2 as i64));

        let id_range_offset = read_u16(self.file);

        self.seek(SeekFrom::Current(-2));

        id_range_offset
    }

    fn read_address(&mut self) -> u64 {
        self.file
            .stream_position()
            .expect("Expected to read stream position")
    }
}

impl CMapSubtable {
    fn find_char_code_segment(file: &File, char_code: u16) -> GlyphId {
        let subtable_format = read_u16(file);
        let length = read_u16(file);
        let version = read_u16(file);

        let seg_count_x2 = read_u16(file); // The segCount is the number of contiguous code ranges in the font
        let search_range = read_u16(file); // TODO compute this from seg_count_x2
        let entry_selector = read_u16(file); // TODO compute this from seg_count_x2
        let range_shift = read_u16(file); // Do not use

        println!("[CMapSubtable] subtable_format {:?}", subtable_format);
        println!("[CMapSubtable] length {:?}", length);
        println!("[CMapSubtable] version {:?}", version);
        println!("[CMapSubtable] seg_count_x2 {:?}", seg_count_x2);
        //println!("[CMapSubtable] seg_count    {:?}", seg_count);
        println!("[CMapSubtable] search_range {:?}", search_range);
        println!("[CMapSubtable] entry_selector {:?}", entry_selector);
        println!("[CMapSubtable] range_shift {:?}", range_shift);

        let file_ops = FileOps { file };

        let mut index_lookup = GlyphIndexLookup {
            file_ops,
            seg_count_x2,
            char_code,
        };

        index_lookup.seek_glyph_id(search_range, entry_selector)
    }
}

struct GlyphIndexLookup<'a> {
    file_ops: FileOps<'a>,
    seg_count_x2: u16,
    char_code: u16,
}

impl<'a> GlyphIndexLookup<'a> {
    fn read_start_code(&mut self) -> u16 {
        self.file_ops.read_start_code(self.seg_count_x2)
    }

    fn read_id_delta(&mut self) -> u16 {
        self.file_ops.read_id_delta(self.seg_count_x2)
    }

    fn read_id_range_offset(&mut self) -> u16 {
        self.file_ops.read_id_range_offset(self.seg_count_x2)
    }

    fn read_u16(&mut self) -> u16 {
        read_u16(self.file_ops.file)
    }

    fn seek_glyph_id(&mut self, search_range: u16, entry_selector: u16) -> GlyphId {
        let end_code = self.file_ops.read_end_code(search_range as i64);

        if self.char_code > end_code {
            self.sequential_search()
        } else {
            self.binary_search(end_code, search_range, entry_selector)
        }
    }

    fn sequential_search(&mut self) -> GlyphId {
        self.file_ops.seek(SeekFrom::Current(2));

        let next_end_code = self.read_u16();

        if next_end_code >= self.char_code {
            self.file_ops.seek(SeekFrom::Current(-2));
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

        if self.char_code <= end_code && self.char_code > start_code {
            let id_delta = self.read_id_delta();
            let id_range_offset = self.read_id_range_offset();

            self.compute_glyp_id(start_code, id_delta, id_range_offset)
        } else {
            if entry_selector == 0 {
                GlyphId::MISSING_CHARACTER_GLYPH
            } else {
                let search_range = search_range >> 1;

                self.file_ops
                    .seek(SeekFrom::Current(-2 - self.seg_count_x2 as i64)); // Go from start_code array (+reservedPad) back to end_code array

                let end_code = if self.char_code < end_code {
                    self.file_ops.read_end_code(-(search_range as i64))
                } else {
                    self.file_ops.read_end_code(search_range as i64)
                };

                self.binary_search(end_code, search_range, entry_selector - 1)
            }
        }
    }

    fn compute_glyp_id(&mut self, start_code: u16, id_delta: u16, id_range_offset: u16) -> GlyphId {
        let glyph_id = if id_range_offset > 0 {
            let address = self.file_ops.read_address();

            let glyph_index_address = id_range_offset as u32
                + 2 * ((self.char_code - start_code) as u32)
                + address as u32;

            self.file_ops
                .seek(SeekFrom::Start(glyph_index_address as u64));
            self.read_u16() as u32
        } else {
            // If the id_range_offset is 0, the id_delta value is added directly to the character code to get the corresponding glyph index
            id_delta as u32 + self.char_code as u32
        };

        // NOTE: All id_delta[i] arithmetic is modulo 65536.
        let glyph_id = (glyph_id % (u16::MAX as u32 + 1)) as u16;

        GlyphId(glyph_id)
    }
}

// Character to glyph index mapping
struct CMapTable {
    //glyph_index_lookup: GlyphIndexLookup,
}

impl CMapTable {
    fn mk_cmap_table(mut file: &File, offset: u64, char_code: u16) -> GlyphId {
        file.seek(SeekFrom::Start(offset))
            .expect("Expected be able to seek");

        let version = read_u16(file);
        let number_subtables = read_u16(file);
        println!("[CMapTable] version {}", version);
        println!("[CMapTable] number_subtables {}", number_subtables);

        for i in 0..number_subtables {
            let platform_id = read_platform_id(file);
            let platform_specific_id = read_u16(file);
            let offset = read_u32(file);

            println!("[CMapTable] platform_id_{} {:?}", i, platform_id);
            println!(
                "[CMapTable] platform_specific_id_{} {}",
                i, platform_specific_id
            );
            println!("[CMapTable] offset_{} {}", i, offset);
        }

        // TODO select the best table. (Is it just a coincidence that Unicode table follows first?)

        CMapSubtable::find_char_code_segment(file, char_code)
    }
}

fn read_platform_id(file: &File) -> PlatformId {
    let platform_id: u16 = read_u16(file);
    match platform_id {
        0 => PlatformId::Unicode,
        1 => PlatformId::Macintosh,
        2 => PlatformId::Reserved,
        3 => PlatformId::Microsoft,
        _ => panic!("Unknown PlatformId {:?}", platform_id),
    }
}

fn read_fword(file: &File) -> FWord {
    let fword = read_i16(file);
    FWord(fword)
}

fn read_long_date_time(mut file: &File) -> i64 {
    let mut buffer = [0; 8];
    file.read_exact(&mut buffer).expect("Can't read i64");
    i64::from_be_bytes(buffer)
}

fn read_fixed(file: &File) -> Fixed {
    let major = read_u16(file);
    let minor = read_u16(file);
    Fixed { major, minor }
}

fn read_u8(mut file: &File) -> u8 {
    let mut buffer = [0; 1];
    file.read_exact(&mut buffer).expect("Can't read u8");
    u8::from_be_bytes(buffer)
}

fn read_i8(mut file: &File) -> i8 {
    let mut buffer = [0; 1];
    file.read_exact(&mut buffer).expect("Can't read i8");
    i8::from_be_bytes(buffer)
}

fn read_u16(mut file: &File) -> u16 {
    let mut buffer = [0; 2];
    file.read_exact(&mut buffer).expect("Can't read u16");
    u16::from_be_bytes(buffer)
}

fn read_i16(mut file: &File) -> i16 {
    let mut buffer = [0; 2];
    file.read_exact(&mut buffer).expect("Can't read i16");
    i16::from_be_bytes(buffer)
}

fn read_u32(mut file: &File) -> u32 {
    let mut buffer = [0; 4];
    file.read_exact(&mut buffer).expect("Exprected to read u32");
    u32::from_be_bytes(buffer)
}

fn read_table_name(mut file: &File) -> String {
    let mut tag = [0; 4];
    file.read_exact(&mut tag).expect("Can't read table name");
    String::from_utf8_lossy(&tag).to_string()
}

pub fn read_font_file(char_code: u16) -> GlyphData {
    let file_path = "fonts/Monaco.ttf";

    let file: File = File::open(file_path).expect("Should been able to open the file");

    let os: OffsetSubtable = OffsetSubtable::from_file(&file);

    println!("Read {:?} os", os);

    let table_dictionary: Vec<TableDirectory> = (0..os.num_tables)
        .into_iter()
        .map(|_| TableDirectory::from_file(&file))
        .collect();

    // table_dictionary
    //     .iter()
    //     .for_each(|td| println!("TABLE {}", td.tag));

    let maybe_head_table: Option<&TableDirectory> =
        table_dictionary.iter().find(|td| td.tag == "head");
    let maybe_loca_table: Option<&TableDirectory> =
        table_dictionary.iter().find(|td| td.tag == "loca");
    let maybe_glyf_table: Option<&TableDirectory> =
        table_dictionary.iter().find(|td| td.tag == "glyf");
    let maybe_cmap_table: Option<&TableDirectory> =
        table_dictionary.iter().find(|td| td.tag == "cmap");
    let maybe_maxp_table: Option<&TableDirectory> =
        table_dictionary.iter().find(|td| td.tag == "maxp");

    let head_table = maybe_head_table.expect("'head' table not found");
    let maxp_table = maybe_maxp_table.expect("'maxp' table not found");
    let loca_table = maybe_loca_table.expect("'loca' table not found");
    let glyf_table = maybe_glyf_table.expect("'glyf' table not found");
    let cmap_table = maybe_cmap_table.expect("'cmap' table not found");

    let head_table_content = HeadTable::from_file(&file, head_table.offset as u64);
    println!("Head table {:?} ", head_table_content);

    let maximum_profile_table = MaximumProfileTable::from_file(&file, maxp_table.offset as u64);
    println!("Maximum profile table {:?} ", maximum_profile_table);

    println!("LOCA TABLE {:?} ", loca_table);
    let lolll: IndexToLocTable = IndexToLocTable::mk_index_to_loc_table(
        &file,
        loca_table.offset as u64,
        head_table_content,
        maximum_profile_table,
    );

    println!("Read {:?} os", glyf_table);

    let glyph_id = CMapTable::mk_cmap_table(&file, cmap_table.offset as u64, char_code);

    let glyph_offset = lolll.index_for(&glyph_id);

    println!("glyph_id {:?} glyph_offset {} ", glyph_id, glyph_offset);
    let glyph_table_content =
        GlyphTable::mk_glyph_table(&file, glyf_table.offset as u64, glyph_offset, glyph_id);
    println!("glyph_offset {:?} ", glyph_offset);
    println!("glyph_table_content {:?}", glyph_table_content);

    glyph_table_content
}

#[cfg(test)]
mod tests {
    use super::*;

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
            GlyphData::CompountGlyph { .. } => panic!("Unexpected glyph type"),
            GlyphData::SimpleGlyph {
                glyph_id,
                x_min,
                x_max,
                y_min,
                y_max,
                contours,
            } => {
                assert_eq!(glyph_id.0, 4);
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
            GlyphData::CompountGlyph { .. } => panic!("Unexpected glyph type"),
            GlyphData::SimpleGlyph {
                glyph_id,
                x_min,
                x_max,
                y_min,
                y_max,
                contours,
            } => {
                assert_eq!(glyph_id.0, 68);
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

    fn abc(c: char) -> u16 {
        if let Ok(abc) = u16::try_from(c as u32) {
            abc
        } else {
            0
        }
    }

    #[test]
    fn test_char_aacute() {
        println!("a      {:?}", b'a');
        //println!("accute {:?}", abc('á' as u32));
        println!("accute {:?}", abc('á'));
        println!("checkerboard {:?}", abc('▒'));

        let result = read_font_file('á' as u16);

        match result {
            GlyphData::CompountGlyph { components } => {
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
                //println!("{:?}", components);
                //panic!("Unexpected glyph type")
            }

            GlyphData::SimpleGlyph { .. } => panic!("Unexpected glyph type"),
        }
    }

    #[test]
    fn test_char_medium_shade() {
        let result = read_font_file('▒' as u16);

        match result {
            GlyphData::CompountGlyph { .. } => panic!("Unexpected glyph type"),
            GlyphData::SimpleGlyph {
                glyph_id,
                x_min,
                x_max,
                y_min,
                y_max,
                contours,
            } => {
                assert_eq!(glyph_id.0, 1676);
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
