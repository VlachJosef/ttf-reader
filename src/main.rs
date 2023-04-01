use std::fs::File;
use ttf_reader::all_glyphs;
use ttf_reader::GlyphReader;

fn read_file(file_path: &str) -> File {
    File::open(file_path).expect("Should been able to open the file")
}

fn main() {
    let file_path = "fonts/GolosText-Regular.ttf";

    let file: File = read_file(file_path);

    let mut glyph_reader = GlyphReader::from_file(file);

    glyph_reader.display_font_info();

    let segments = glyph_reader.cmap_table_segments();

    for segment in segments {
        println!("{:?}", segment);
    }

    all_glyphs(glyph_reader);
}
