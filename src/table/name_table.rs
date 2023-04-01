use crate::font_directory::TableDirectory;
use crate::model::PlatformId;
use crate::reader::Reader;

#[allow(unused)]
#[derive(Debug)]
struct NameRecord {
    platform_id: PlatformId,
    platform_specific_id: u16,
    language_id: u16,
    name_id: NameId,
    length: u16,
    offset: u16,
}

#[derive(Debug)]
struct NameId(u16);

impl NameId {
    fn info(&self) -> &str {
        match self.0 {
            0 => "Copyright",
            1 => "Font Family",
            2 => "Font Subfamily",
            3 => "Unique subfamily identification",
            4 => "Full name of the font",
            5 => "Version of the name table",
            6 => "PostScript name",
            7 => "Trademark notice",
            8 => "Manufacturer name",
            9 => "Designer",
            10 => "Description",
            11 => "URL of the font vendor",
            12 => "URL of the font designer",
            13 => "License description",
            14 => "License information",
            _ => "other",
        }
    }
}

pub fn read_name(reader: &mut Box<dyn Reader>, name_table: &TableDirectory) {
    reader.seek_from_start(name_table.offset);

    let format = reader.read_u16();
    let count = reader.read_u16();
    let string_offset = reader.read_u16();

    assert!(0 == format, "format of name table should be 0");

    let name_records: Vec<NameRecord> = (0..count)
        .into_iter()
        .map(|_| {
            let platform_id = reader.read_platform_id();
            let platform_specific_id = reader.read_u16();
            let language_id = reader.read_u16();
            let name_id = reader.read_u16();
            let name_id = NameId(name_id);
            let length = reader.read_u16();
            let offset = reader.read_u16();

            NameRecord {
                platform_id,
                platform_specific_id,
                language_id,
                name_id,
                length,
                offset,
            }
        })
        .collect();

    name_records.iter().for_each(|nr| {
        if nr.platform_id == PlatformId::Macintosh && nr.language_id == 0 {
            reader.seek_from_start(name_table.offset);
            reader.seek_from_current((string_offset + nr.offset) as i32);

            let str_value = reader.read_string(nr.length);
            let info = nr.name_id.info();

            println!("[Macintosh] {}: {}", info, str_value);
        } else if nr.platform_id == PlatformId::Microsoft && nr.language_id == 1033 {
            reader.seek_from_start(name_table.offset);
            reader.seek_from_current((string_offset + nr.offset) as i32);

            let str_value = reader.read_utf_16be(nr.length);
            let info = nr.name_id.info();
            println!("[Microsolf] {}: {}", info, str_value);
        }
    });
}
