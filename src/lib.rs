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

fn read_file(file_path: &str) -> File {
    File::open(file_path).expect("Should been able to open the file")
}

fn mk_glyph_reader(file_path: &str) -> GlyphReader {
    let file: File = read_file(file_path);

    GlyphReader::from_file(file)
}

pub fn read_glyph(char_code: u16, file_path: &str) -> Glyph {
    let mut glyph_reader = mk_glyph_reader(file_path);

    glyph_reader.read_glyph(char_code)
}

pub fn read_glyph_id(glyph_id: GlyphId, file_path: &str) -> Glyph {
    let mut glyph_reader = mk_glyph_reader(file_path);

    glyph_reader.glyph_for_glyph_id(glyph_id)
}

pub fn cmap_table_segments(file_path: &str) -> Vec<Segment> {
    let mut glyph_reader = mk_glyph_reader(file_path);
    glyph_reader.cmap_table_segments()
}

pub fn all_glyphs(file_path: &str) -> Vec<Glyph> {
    let mut glyph_reader = mk_glyph_reader(file_path);

    let all_chars: Vec<u16> = glyph_reader.all_char_codes();

    let all_glyphs: Vec<Glyph> = all_chars
        .iter()
        .map(|char_code| glyph_reader.read_glyph(*char_code))
        .collect();

    all_glyphs
        .iter()
        .for_each(|glyph| println!("Glyph: {:?}", glyph));

    all_glyphs
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

        //panic!("");
    }

    #[test]
    fn all_char_codes() {
        let mut glyph_reader = mk_glyph_reader("fonts/Zeyada_1");

        let char_codes = glyph_reader.all_char_codes();

        #[rustfmt::skip]
        let expected_char_codes = vec![
            32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50,
            51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64, 65, 66, 67, 68, 69,
            70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 80, 81, 82, 83, 84, 85, 86, 87, 88,
            89, 90, 91, 92, 93, 94, 95, 96, 97, 98, 99, 100, 101, 102, 103, 104, 105, 106,
            107, 108, 109, 110, 111, 112, 113, 114, 115, 116, 117, 118, 119, 120, 121, 122,
            123, 124, 125, 126, 160, 161, 162, 163, 164, 165, 166, 167, 168, 169, 170, 171,
            172, 173, 174, 175, 176, 177, 178, 179, 180, 181, 182, 183, 184, 185, 186, 187,
            188, 189, 190, 191, 192, 193, 194, 195, 196, 197, 198, 199, 200, 201, 202, 203,
            204, 205, 206, 207, 208, 209, 210, 211, 212, 213, 214, 215, 216, 217, 218, 219,
            220, 221, 222, 223, 224, 225, 226, 227, 228, 229, 230, 231, 232, 233, 234, 235,
            236, 237, 238, 239, 240, 241, 242, 243, 244, 245, 246, 247, 248, 249, 250, 251,
            252, 253, 254, 255, 256, 257, 258, 259, 260, 261, 262, 263, 264, 265, 266, 267,
            268, 269, 270, 274, 275, 276, 277, 278, 279, 280, 281, 282, 283, 284, 285, 286,
            287, 288, 289, 290, 292, 293, 296, 297, 299, 300, 301, 302, 303, 304, 305, 308,
            309, 310, 311, 313, 314, 315, 316, 319, 320, 321, 322, 323, 324, 325, 326, 327,
            328, 332, 333, 334, 335, 336, 337, 338, 339, 340, 341, 342, 343, 344, 345, 346,
            347, 348, 349, 350, 351, 352, 353, 354, 355, 356, 360, 361, 362, 363, 364, 365,
            366, 367, 368, 369, 370, 371, 372, 373, 374, 375, 376, 377, 378, 379, 380, 381,
            382, 402, 508, 509, 510, 511, 537, 710, 711, 728, 729, 730, 731, 732, 733, 7808,
            7809, 7810, 7811, 7812, 7813, 7922, 7923, 8211, 8212, 8216, 8217, 8218, 8220, 8221,
            8222, 8224, 8225, 8226, 8230, 8249, 8250, 8260, 8364, 8482, 8722, 63171, 64257, 64258, 65535
        ];

        assert_eq!(char_codes, expected_char_codes);
        //panic!("");
    }

    #[test]
    fn test_char_code_to_glyph_id() {
        let mut glyph_reader = mk_glyph_reader("fonts/Zeyada_1");

        let char_codes: Vec<u16> = glyph_reader.all_char_codes();

        let mapping: Vec<(u16, GlyphId)> = char_codes
            .into_iter()
            .filter(|char_code| *char_code < u16::MAX)
            .map(|char_code| (char_code, glyph_reader.char_code_to_glyph_id(char_code)))
            .collect();

        let expected_mapping: Vec<(u16, GlyphId)> = vec![
            (32, GlyphId::new(3)),
            (33, GlyphId::new(4)),
            (34, GlyphId::new(5)),
            (35, GlyphId::new(6)),
            (36, GlyphId::new(7)),
            (37, GlyphId::new(8)),
            (38, GlyphId::new(9)),
            (39, GlyphId::new(10)),
            (40, GlyphId::new(11)),
            (41, GlyphId::new(12)),
            (42, GlyphId::new(13)),
            (43, GlyphId::new(14)),
            (44, GlyphId::new(15)),
            (45, GlyphId::new(16)),
            (46, GlyphId::new(17)),
            (47, GlyphId::new(18)),
            (48, GlyphId::new(19)),
            (49, GlyphId::new(20)),
            (50, GlyphId::new(21)),
            (51, GlyphId::new(22)),
            (52, GlyphId::new(23)),
            (53, GlyphId::new(24)),
            (54, GlyphId::new(25)),
            (55, GlyphId::new(26)),
            (56, GlyphId::new(27)),
            (57, GlyphId::new(28)),
            (58, GlyphId::new(29)),
            (59, GlyphId::new(30)),
            (60, GlyphId::new(31)),
            (61, GlyphId::new(32)),
            (62, GlyphId::new(33)),
            (63, GlyphId::new(34)),
            (64, GlyphId::new(35)),
            (65, GlyphId::new(36)),
            (66, GlyphId::new(37)),
            (67, GlyphId::new(38)),
            (68, GlyphId::new(39)),
            (69, GlyphId::new(40)),
            (70, GlyphId::new(41)),
            (71, GlyphId::new(42)),
            (72, GlyphId::new(43)),
            (73, GlyphId::new(44)),
            (74, GlyphId::new(45)),
            (75, GlyphId::new(46)),
            (76, GlyphId::new(47)),
            (77, GlyphId::new(48)),
            (78, GlyphId::new(49)),
            (79, GlyphId::new(50)),
            (80, GlyphId::new(51)),
            (81, GlyphId::new(52)),
            (82, GlyphId::new(53)),
            (83, GlyphId::new(54)),
            (84, GlyphId::new(55)),
            (85, GlyphId::new(56)),
            (86, GlyphId::new(57)),
            (87, GlyphId::new(58)),
            (88, GlyphId::new(59)),
            (89, GlyphId::new(60)),
            (90, GlyphId::new(61)),
            (91, GlyphId::new(62)),
            (92, GlyphId::new(63)),
            (93, GlyphId::new(64)),
            (94, GlyphId::new(65)),
            (95, GlyphId::new(66)),
            (96, GlyphId::new(67)),
            (97, GlyphId::new(68)),
            (98, GlyphId::new(69)),
            (99, GlyphId::new(70)),
            (100, GlyphId::new(71)),
            (101, GlyphId::new(72)),
            (102, GlyphId::new(73)),
            (103, GlyphId::new(74)),
            (104, GlyphId::new(75)),
            (105, GlyphId::new(76)),
            (106, GlyphId::new(77)),
            (107, GlyphId::new(78)),
            (108, GlyphId::new(79)),
            (109, GlyphId::new(80)),
            (110, GlyphId::new(81)),
            (111, GlyphId::new(82)),
            (112, GlyphId::new(83)),
            (113, GlyphId::new(84)),
            (114, GlyphId::new(85)),
            (115, GlyphId::new(86)),
            (116, GlyphId::new(87)),
            (117, GlyphId::new(88)),
            (118, GlyphId::new(89)),
            (119, GlyphId::new(90)),
            (120, GlyphId::new(91)),
            (121, GlyphId::new(92)),
            (122, GlyphId::new(93)),
            (123, GlyphId::new(94)),
            (124, GlyphId::new(95)),
            (125, GlyphId::new(96)),
            (126, GlyphId::new(97)),
            (160, GlyphId::new(98)),
            (161, GlyphId::new(99)),
            (162, GlyphId::new(100)),
            (163, GlyphId::new(101)),
            (164, GlyphId::new(102)),
            (165, GlyphId::new(103)),
            (166, GlyphId::new(104)),
            (167, GlyphId::new(105)),
            (168, GlyphId::new(106)),
            (169, GlyphId::new(107)),
            (170, GlyphId::new(108)),
            (171, GlyphId::new(109)),
            (172, GlyphId::new(110)),
            (173, GlyphId::new(111)),
            (174, GlyphId::new(112)),
            (175, GlyphId::new(113)),
            (176, GlyphId::new(114)),
            (177, GlyphId::new(115)),
            (178, GlyphId::new(116)),
            (179, GlyphId::new(117)),
            (180, GlyphId::new(118)),
            (181, GlyphId::new(119)),
            (182, GlyphId::new(120)),
            (183, GlyphId::new(121)),
            (184, GlyphId::new(122)),
            (185, GlyphId::new(123)),
            (186, GlyphId::new(124)),
            (187, GlyphId::new(125)),
            (188, GlyphId::new(126)),
            (189, GlyphId::new(127)),
            (190, GlyphId::new(128)),
            (191, GlyphId::new(129)),
            (192, GlyphId::new(130)),
            (193, GlyphId::new(131)),
            (194, GlyphId::new(132)),
            (195, GlyphId::new(133)),
            (196, GlyphId::new(134)),
            (197, GlyphId::new(135)),
            (198, GlyphId::new(136)),
            (199, GlyphId::new(137)),
            (200, GlyphId::new(138)),
            (201, GlyphId::new(139)),
            (202, GlyphId::new(140)),
            (203, GlyphId::new(141)),
            (204, GlyphId::new(142)),
            (205, GlyphId::new(143)),
            (206, GlyphId::new(144)),
            (207, GlyphId::new(145)),
            (208, GlyphId::new(146)),
            (209, GlyphId::new(147)),
            (210, GlyphId::new(148)),
            (211, GlyphId::new(149)),
            (212, GlyphId::new(150)),
            (213, GlyphId::new(151)),
            (214, GlyphId::new(152)),
            (215, GlyphId::new(153)),
            (216, GlyphId::new(154)),
            (217, GlyphId::new(155)),
            (218, GlyphId::new(156)),
            (219, GlyphId::new(157)),
            (220, GlyphId::new(158)),
            (221, GlyphId::new(159)),
            (222, GlyphId::new(160)),
            (223, GlyphId::new(161)),
            (224, GlyphId::new(162)),
            (225, GlyphId::new(163)),
            (226, GlyphId::new(164)),
            (227, GlyphId::new(165)),
            (228, GlyphId::new(166)),
            (229, GlyphId::new(167)),
            (230, GlyphId::new(168)),
            (231, GlyphId::new(169)),
            (232, GlyphId::new(170)),
            (233, GlyphId::new(171)),
            (234, GlyphId::new(172)),
            (235, GlyphId::new(173)),
            (236, GlyphId::new(174)),
            (237, GlyphId::new(175)),
            (238, GlyphId::new(176)),
            (239, GlyphId::new(177)),
            (240, GlyphId::new(178)),
            (241, GlyphId::new(179)),
            (242, GlyphId::new(180)),
            (243, GlyphId::new(181)),
            (244, GlyphId::new(182)),
            (245, GlyphId::new(183)),
            (246, GlyphId::new(184)),
            (247, GlyphId::new(185)),
            (248, GlyphId::new(186)),
            (249, GlyphId::new(187)),
            (250, GlyphId::new(188)),
            (251, GlyphId::new(189)),
            (252, GlyphId::new(190)),
            (253, GlyphId::new(191)),
            (254, GlyphId::new(192)),
            (255, GlyphId::new(193)),
            (256, GlyphId::new(194)),
            (257, GlyphId::new(195)),
            (258, GlyphId::new(196)),
            (259, GlyphId::new(197)),
            (260, GlyphId::new(198)),
            (261, GlyphId::new(199)),
            (262, GlyphId::new(200)),
            (263, GlyphId::new(201)),
            (264, GlyphId::new(202)),
            (265, GlyphId::new(203)),
            (266, GlyphId::new(204)),
            (267, GlyphId::new(205)),
            (268, GlyphId::new(206)),
            (269, GlyphId::new(207)),
            (270, GlyphId::new(208)),
            (274, GlyphId::new(209)),
            (275, GlyphId::new(210)),
            (276, GlyphId::new(211)),
            (277, GlyphId::new(212)),
            (278, GlyphId::new(213)),
            (279, GlyphId::new(214)),
            (280, GlyphId::new(215)),
            (281, GlyphId::new(216)),
            (282, GlyphId::new(217)),
            (283, GlyphId::new(218)),
            (284, GlyphId::new(219)),
            (285, GlyphId::new(220)),
            (286, GlyphId::new(221)),
            (287, GlyphId::new(222)),
            (288, GlyphId::new(223)),
            (289, GlyphId::new(224)),
            (290, GlyphId::new(225)),
            (292, GlyphId::new(226)),
            (293, GlyphId::new(227)),
            (296, GlyphId::new(228)),
            (297, GlyphId::new(229)),
            (299, GlyphId::new(230)),
            (300, GlyphId::new(231)),
            (301, GlyphId::new(232)),
            (302, GlyphId::new(233)),
            (303, GlyphId::new(234)),
            (304, GlyphId::new(235)),
            (305, GlyphId::new(236)),
            (308, GlyphId::new(237)),
            (309, GlyphId::new(238)),
            (310, GlyphId::new(239)),
            (311, GlyphId::new(240)),
            (313, GlyphId::new(241)),
            (314, GlyphId::new(242)),
            (315, GlyphId::new(243)),
            (316, GlyphId::new(244)),
            (319, GlyphId::new(245)),
            (320, GlyphId::new(246)),
            (321, GlyphId::new(247)),
            (322, GlyphId::new(248)),
            (323, GlyphId::new(249)),
            (324, GlyphId::new(250)),
            (325, GlyphId::new(251)),
            (326, GlyphId::new(252)),
            (327, GlyphId::new(253)),
            (328, GlyphId::new(254)),
            (332, GlyphId::new(255)),
            (333, GlyphId::new(256)),
            (334, GlyphId::new(257)),
            (335, GlyphId::new(258)),
            (336, GlyphId::new(259)),
            (337, GlyphId::new(260)),
            (338, GlyphId::new(261)),
            (339, GlyphId::new(262)),
            (340, GlyphId::new(263)),
            (341, GlyphId::new(264)),
            (342, GlyphId::new(265)),
            (343, GlyphId::new(266)),
            (344, GlyphId::new(267)),
            (345, GlyphId::new(268)),
            (346, GlyphId::new(269)),
            (347, GlyphId::new(270)),
            (348, GlyphId::new(271)),
            (349, GlyphId::new(272)),
            (350, GlyphId::new(273)),
            (351, GlyphId::new(274)),
            (352, GlyphId::new(275)),
            (353, GlyphId::new(276)),
            (354, GlyphId::new(277)),
            (355, GlyphId::new(278)),
            (356, GlyphId::new(279)),
            (360, GlyphId::new(280)),
            (361, GlyphId::new(281)),
            (362, GlyphId::new(282)),
            (363, GlyphId::new(283)),
            (364, GlyphId::new(284)),
            (365, GlyphId::new(285)),
            (366, GlyphId::new(286)),
            (367, GlyphId::new(287)),
            (368, GlyphId::new(288)),
            (369, GlyphId::new(289)),
            (370, GlyphId::new(290)),
            (371, GlyphId::new(291)),
            (372, GlyphId::new(292)),
            (373, GlyphId::new(293)),
            (374, GlyphId::new(294)),
            (375, GlyphId::new(295)),
            (376, GlyphId::new(296)),
            (377, GlyphId::new(297)),
            (378, GlyphId::new(298)),
            (379, GlyphId::new(299)),
            (380, GlyphId::new(300)),
            (381, GlyphId::new(301)),
            (382, GlyphId::new(302)),
            (402, GlyphId::new(303)),
            (508, GlyphId::new(304)),
            (509, GlyphId::new(305)),
            (510, GlyphId::new(306)),
            (511, GlyphId::new(307)),
            (537, GlyphId::new(308)),
            (710, GlyphId::new(309)),
            (711, GlyphId::new(310)),
            (728, GlyphId::new(311)),
            (729, GlyphId::new(312)),
            (730, GlyphId::new(313)),
            (731, GlyphId::new(314)),
            (732, GlyphId::new(315)),
            (733, GlyphId::new(316)),
            (7808, GlyphId::new(317)),
            (7809, GlyphId::new(318)),
            (7810, GlyphId::new(319)),
            (7811, GlyphId::new(320)),
            (7812, GlyphId::new(321)),
            (7813, GlyphId::new(322)),
            (7922, GlyphId::new(323)),
            (7923, GlyphId::new(324)),
            (8211, GlyphId::new(325)),
            (8212, GlyphId::new(326)),
            (8216, GlyphId::new(327)),
            (8217, GlyphId::new(328)),
            (8218, GlyphId::new(329)),
            (8220, GlyphId::new(330)),
            (8221, GlyphId::new(331)),
            (8222, GlyphId::new(332)),
            (8224, GlyphId::new(333)),
            (8225, GlyphId::new(334)),
            (8226, GlyphId::new(335)),
            (8230, GlyphId::new(336)),
            (8249, GlyphId::new(337)),
            (8250, GlyphId::new(338)),
            (8260, GlyphId::new(339)),
            (8364, GlyphId::new(340)),
            (8482, GlyphId::new(341)),
            (8722, GlyphId::new(342)),
            (63171, GlyphId::new(343)),
            (64257, GlyphId::new(344)),
            (64258, GlyphId::new(345)),
        ];

        assert_eq!(mapping, expected_mapping);
    }

    #[test]
    fn notdef_0() {
        let result = read_glyph_id(GlyphId::new(0), "fonts/Zeyada_1");

        match result {
            Glyph::Empty { .. } => assert!(true),
            Glyph::Compount { .. } => panic!("Expected Empty glyph"),
            Glyph::Simple { .. } => panic!("Expected Empty glyph"),
        }
    }

    #[test]
    fn glyph_98() {
        let result = read_glyph_id(GlyphId::new(98), "fonts/Zeyada_1");

        match result {
            Glyph::Empty { .. } => assert!(true),
            Glyph::Compount { .. } => panic!("Expected Empty glyph"),
            Glyph::Simple { .. } => panic!("Expected Empty glyph"),
        }
    }

    #[test]
    fn notdef_4() {
        let result = read_glyph_id(GlyphId::new(4), "fonts/Zeyada_1");

        match result {
            Glyph::Empty { .. } => panic!("Expected Simple glyph"),
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
            Glyph::Empty { .. } => panic!("Expected Simple glyph"),
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
            Glyph::Empty { .. } => panic!("Expected Simple glyph"),
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
            Glyph::Empty { .. } => panic!("Expected Simple glyph"),
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
            Glyph::Empty { .. } => panic!("Expected Empty glyph"),
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
            Glyph::Empty { .. } => panic!("Expected Compount glyph"),
            Glyph::Compount {
                glyph_id,
                components,
            } => {
                assert_eq!(glyph_id.id(), 105);
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
            Glyph::Empty { .. } => panic!("expected Simple glyph"),
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
