use ttf_reader::{all_glyphs, cmap_table_segments, display_font_info};

fn main() {
    //let font = "fonts/Zeyada_1.ttf";
    let font = "fonts/GolosText-Regular.ttf";
    display_font_info(font);

    let segments = cmap_table_segments(font);

    for segment in segments {
        println!("{:?}", segment);
    }

    all_glyphs(font);
}
