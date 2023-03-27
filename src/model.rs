#[derive(Debug, PartialEq)]
pub enum PlatformId {
    Unicode,
    Macintosh,
    Reserved,
    Microsoft,
}

#[derive(Debug)]
pub struct FWord(pub i16);

#[derive(Debug)]
pub struct UFWord(pub u16);

#[allow(unused)]
#[derive(Debug)]
pub struct Fixed {
    pub major: u16,
    pub minor: u16,
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct GlyphId(u16);

impl GlyphId {
    pub const MISSING_CHARACTER_GLYPH: GlyphId = GlyphId(0);

    pub fn new(id: u16) -> GlyphId {
        GlyphId(id)
    }

    pub fn id(&self) -> u16 {
        self.0
    }
}

#[derive(Debug)]
pub enum Glyph {
    Empty {
        glyph_id: GlyphId,
        advance_width: u16,
        left_side_bearing: i16,
    },
    Simple {
        glyph_id: GlyphId,
        x_min: FWord, // Minimum x for coordinate data
        y_min: FWord, // Minimum y for coordinate data
        x_max: FWord, // Maximum x for coordinate data
        y_max: FWord, // Maximum y for coordinate data
        advance_width: u16,
        left_side_bearing: i16,
        contours: Vec<Contour>,
    },
    Compount {
        glyph_id: GlyphId,
        advance_width: u16,
        left_side_bearing: i16,
        components: Vec<ComponentData>,
    },
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum PointType {
    OnCurve,
    Control,
}

impl PointType {
    pub fn from(on_curve: bool) -> PointType {
        if on_curve {
            PointType::OnCurve
        } else {
            PointType::Control
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Contour {
    pub points: Vec<Point>,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Point {
    pub x: i16,
    pub y: i16,
    pub tpe: PointType,
}

impl Point {
    pub fn new(x: i16, y: i16, tpe: PointType) -> Point {
        Point { x, y, tpe }
    }
}

#[derive(Debug)]
pub struct Contours {
    pub contours: Vec<Contour>,
}

#[derive(Debug, PartialEq)]
pub enum ArgumentTypes {
    XYValue16(i16, i16),
    Point16(u16, u16),
    XYValue8(i8, i8),
    Point8(u8, u8),
}

#[allow(unused)]
#[derive(Debug)]
pub struct ComponentData {
    pub glyph_index: u16,
    pub a: i16,
    pub b: i16,
    pub c: i16,
    pub d: i16,
    pub argument_types: ArgumentTypes, // encapsulates e, f
}
