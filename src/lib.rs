use crate::glyph_reader::GlyphReader;
use crate::model::{Glyph, GlyphId};
use crate::table::cmap_table::Segment;
use std::fs::File;

mod contours_reader;
mod file_ops;
mod font_directory;
mod glyph_index_lookup;
mod glyph_reader;
mod model;
mod table;

// https://developer.apple.com/fonts/TrueType-Reference-Manual/RM06/Chap6.html
// https://learn.microsoft.com/en-us/typography/opentype/spec/
// http://formats.kaitai.io/ttf/ttf.svg

pub fn read_glyph(char_code: u16, file_path: &str) -> Glyph {
    let file: File = File::open(file_path).expect("Should been able to open the file");

    let mut glyph_reader = GlyphReader::from_file(file);

    glyph_reader.read_glyph(char_code)
}

pub fn read_glyph_id(glyph_id: GlyphId, file_path: &str) -> Glyph {
    let file: File = File::open(file_path).expect("Should been able to open the file");

    let mut glyph_reader = GlyphReader::from_file(file);

    glyph_reader.glyph_for_glyph_id(glyph_id)
}

pub fn cmap_table_segments(file_path: &str) -> Vec<Segment> {
    let file: File = File::open(file_path).expect("Should been able to open the file");
    let mut glyph_reader = GlyphReader::from_file(file);
    glyph_reader.cmap_table_segments()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{ArgumentTypes, Contour, Point, PointType};

    macro_rules! mk_segments {
        ($index:literal, $start_code:literal, $end_code:literal, $id_delta:literal, $id_range_offset:literal) => {{
            Segment::new($index, $start_code, $end_code, $id_delta, $id_range_offset)
        }};
    }

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
    fn zeyada_cmap_segments() {
        let segments = cmap_table_segments("fonts/Zeyada_1");

        let expected_segments = vec![
            mk_segments!(0, 32, 126, 65507, 0),
            mk_segments!(1, 160, 270, 65474, 0),
            mk_segments!(2, 274, 290, 65471, 0),
            mk_segments!(3, 292, 293, 65470, 0),
            mk_segments!(4, 296, 297, 65468, 0),
            mk_segments!(5, 299, 305, 65467, 0),
            mk_segments!(6, 308, 311, 65465, 0),
            mk_segments!(7, 313, 316, 65464, 0),
            mk_segments!(8, 319, 328, 65462, 0),
            mk_segments!(9, 332, 356, 65459, 0),
            mk_segments!(10, 360, 382, 65456, 0),
            mk_segments!(11, 402, 402, 65437, 0),
            mk_segments!(12, 508, 511, 65332, 0),
            mk_segments!(13, 537, 537, 65307, 0),
            mk_segments!(14, 710, 711, 65135, 0),
            mk_segments!(15, 728, 733, 65119, 0),
            mk_segments!(16, 7808, 7813, 58045, 0),
            mk_segments!(17, 7922, 7923, 57937, 0),
            mk_segments!(18, 8211, 8212, 57650, 0),
            mk_segments!(19, 8216, 8218, 57647, 0),
            mk_segments!(20, 8220, 8222, 57646, 0),
            mk_segments!(21, 8224, 8226, 57645, 0),
            mk_segments!(22, 8230, 8230, 57642, 0),
            mk_segments!(23, 8249, 8250, 57624, 0),
            mk_segments!(24, 8260, 8260, 57615, 0),
            mk_segments!(25, 8364, 8364, 57512, 0),
            mk_segments!(26, 8482, 8482, 57395, 0),
            mk_segments!(27, 8722, 8722, 57156, 0),
            mk_segments!(28, 63171, 63171, 2708, 0),
            mk_segments!(29, 64257, 64258, 1623, 0),
            mk_segments!(30, 65535, 65535, 1, 0),
        ];

        assert_eq!(segments, expected_segments);
    }

    #[test]
    fn notdef_0() {
        let result = read_glyph_id(GlyphId::new(0), "fonts/Zeyada_1");

        match result {
            Glyph::Empty => assert!(true),
            Glyph::Compount { .. } => panic!("Expected Empty glyph"),
            Glyph::Simple { .. } => panic!("Expected Empty glyph"),
        }
    }

    #[test]
    fn notdef_4() {
        let result = read_glyph_id(GlyphId::new(4), "fonts/Zeyada_1");

        match result {
            Glyph::Empty => panic!("Expected Simple glyph"),
            Glyph::Compount { .. } => panic!("Expected Simple glyph"),
            Glyph::Simple {
                glyph_id,
                x_min,
                x_max,
                y_min,
                y_max,
                contours,
            } => {
                assert_eq!(glyph_id.id(), 4);
                assert_eq!(x_min.0, 84);
                assert_eq!(x_max.0, 172);
                assert_eq!(y_min.0, 0);
                assert_eq!(y_max.0, 732);

                #[rustfmt::skip]
                let expected_contours = vec![
                    mk_contour!(
                        84, 574 - OnCurve,
                        84, 604 - OnCurve,
                        84, 625 - Control,
                        85, 672 - Control,
                        85, 693  - OnCurve,
                        85, 722  - OnCurve,
                        88, 730  - Control,
                        97, 732  - Control,
                        102, 732  - OnCurve,
                        107, 732  - Control,
                        117, 730  - Control,
                        119, 722  - OnCurve,
                        119, 713  - Control,
                        118, 675  - Control,
                        118, 650  - OnCurve,
                        118, 549  - OnCurve,
                        118, 525  - Control,
                        119, 487  - Control,
                        119, 478  - OnCurve,
                        120, 439  - Control,
                        126, 397  - OnCurve,
                        138, 315  - OnCurve,
                        144, 274  - Control,
                        154, 193  - Control,
                        154, 154  - OnCurve,
                        154, 149  - Control,
                        153, 140  - Control,
                        146, 137  - OnCurve,
                        139, 141  - Control,
                        130, 157  - Control,
                        128, 163  - OnCurve,
                        100, 267  - Control,
                        84, 468  - Control
                    ),
                    mk_contour!(
                        119, 26  - OnCurve,
                        119, 40  - Control,
                        138, 54  - Control,
                        151, 54  - OnCurve,
                        172, 54  - Control,
                        172, 31  - OnCurve,
                        172, 27  - Control,
                        169, 16  - Control,
                        163, 7   - Control,
                        155, 0   - Control,
                        151, 0   - OnCurve,
                        139, 0   - Control,
                        119, 15  - Control
                    )];

                assert_eq!(contours, expected_contours);

                //panic!("");
            }
        }
    }

    #[test]
    fn notdef_monaco() {
        let result = read_glyph_id(GlyphId::new(0), "fonts/Monaco.ttf");

        match result {
            Glyph::Empty => panic!("Expected Simple glyph"),
            Glyph::Compount { .. } => panic!("Expected Simple glyph"),
            Glyph::Simple {
                glyph_id,
                x_min,
                x_max,
                y_min,
                y_max,
                contours,
            } => {
                assert_eq!(glyph_id.id(), 0);
                assert_eq!(x_min.0, 147);
                assert_eq!(x_max.0, 877);
                assert_eq!(y_min.0, 0);
                assert_eq!(y_max.0, 1552);

                #[rustfmt::skip]
                let expected_contours = vec![
                    mk_contour!(
                        877,    0 - OnCurve,
                        147,    0 - OnCurve,
                        147, 1552 - OnCurve,
                        877, 1552 - OnCurve
                    ),
                    mk_contour!(
                        727, 1402 - OnCurve,
                        297, 1402 - OnCurve,
                        297, 150 - OnCurve,
                        727, 150 - OnCurve
                    )];

                assert_eq!(contours, expected_contours);

                //panic!("");
            }
        }
    }

    #[test]
    fn test_char_exclamation_mark_zeyada() {
        let result = read_glyph(b'!' as u16, "fonts/Zeyada_1");

        match result {
            Glyph::Empty => panic!("Expected Simple glyph"),
            Glyph::Compount { .. } => panic!("Expected Simple glyph"),
            Glyph::Simple {
                glyph_id,
                x_min,
                x_max,
                y_min,
                y_max,
                contours,
            } => {
                assert_eq!(glyph_id.id(), 4);
                assert_eq!(x_min.0, 84);
                assert_eq!(x_max.0, 172);
                assert_eq!(y_min.0, 0);
                assert_eq!(y_max.0, 732);

                #[rustfmt::skip]
                let expected_contours = vec![
                    mk_contour!(
                         84, 574 - OnCurve,
                         84, 604 - OnCurve,
                         84, 625 - Control,
                         85, 672 - Control,
                         85, 693 - OnCurve,
                         85, 722 - OnCurve,
                         88, 730 - Control,
                         97, 732 - Control,
                        102, 732 - OnCurve,
                        107, 732 - Control,
                        117, 730 - Control,
                        119, 722 - OnCurve,
                        119, 713 - Control,
                        118, 675 - Control,
                        118, 650 - OnCurve,
                        118, 549 - OnCurve,
                        118, 525 - Control,
                        119, 487 - Control,
                        119, 478 - OnCurve,
                        120, 439 - Control,
                        126, 397 - OnCurve,
                        138, 315 - OnCurve,
                        144, 274 - Control,
                        154, 193 - Control,
                        154, 154 - OnCurve,
                        154, 149 - Control,
                        153, 140 - Control,
                        146, 137 - OnCurve,
                        139, 141 - Control,
                        130, 157 - Control,
                        128, 163 - OnCurve,
                        100, 267 - Control,
                         84, 468 - Control

                    ),
                    mk_contour!(
                        119, 26 - OnCurve,
                        119, 40 - Control,
                        138, 54 - Control,
                        151, 54 - OnCurve,
                        172, 54 - Control,
                        172, 31 - OnCurve,
                        172, 27 - Control,
                        169, 16 - Control,
                        163,  7 - Control,
                        155,  0 - Control,
                        151,  0 - OnCurve,
                        139,  0 - Control,
                        119, 15 - Control
                    )];

                assert_eq!(contours, expected_contours);
            }
        }
    }

    #[test]
    fn test_char_exclamation_mark_monaco() {
        let result = read_glyph(b'!' as u16, "fonts/Monaco.ttf");

        match result {
            Glyph::Empty => panic!("Expected Simple glyph"),
            Glyph::Compount { .. } => panic!("Expected Simple glyph"),
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
    fn test_monaco_char_a() {
        let result = read_glyph(b'a' as u16, "fonts/Monaco.ttf");

        match result {
            Glyph::Empty => panic!("Expected Empty glyph"),
            Glyph::Compount { .. } => panic!("Expected Simpe glyph"),
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
        let result = read_glyph('á' as u16, "fonts/Monaco.ttf");

        match result {
            Glyph::Empty => panic!("Expected Compount glyph"),
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

            Glyph::Simple { .. } => panic!("Expected Compount glyph"),
        }
    }

    #[test]
    fn test_char_medium_shade() {
        let result = read_glyph('▒' as u16, "fonts/Monaco.ttf");

        match result {
            Glyph::Empty => panic!("expected Simple glyph"),
            Glyph::Compount { .. } => panic!("Expected Simpe glyph"),
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
