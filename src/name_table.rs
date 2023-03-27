use crate::file_ops::FileOps;
use crate::font_directory::TableDirectory;
use crate::model::PlatformId;

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
            _ => "other",
        }
    }
}

pub fn read_name(file_ops: &mut FileOps, td: &TableDirectory) {
    file_ops.seek_from_start(td.offset);

    let _format = file_ops.read_u16();
    let count = file_ops.read_u16();
    let _string_offset = file_ops.read_u16();

    let name_records: Vec<NameRecord> = (0..count)
        .into_iter()
        .map(|_| {
            let platform_id = file_ops.read_platform_id();
            let platform_specific_id = file_ops.read_u16();
            let language_id = file_ops.read_u16();
            let name_id = file_ops.read_u16();
            let name_id = NameId(name_id);
            let length = file_ops.read_u16();
            let offset = file_ops.read_u16();

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
            let str_value = file_ops.read_string(nr.length);
            let info = nr.name_id.info();
            println!("{}: {}", info, str_value);
        }
    });
}
