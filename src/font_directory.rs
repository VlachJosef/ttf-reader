use crate::reader::Reader;

#[allow(unused)]
pub struct FontDirectory {
    offset_subtable: OffsetSubtable,
    table_dictionary: Vec<TableDirectory>,
}

impl FontDirectory {
    pub fn from_file(reader: &mut Box<dyn Reader>) -> FontDirectory {
        let offset_subtable: OffsetSubtable = OffsetSubtable::from_file(reader);

        let table_dictionary: Vec<TableDirectory> = (0..offset_subtable.num_tables)
            .into_iter()
            .map(|_| TableDirectory::from_file(reader))
            .collect();

        FontDirectory {
            offset_subtable,
            table_dictionary,
        }
    }

    pub fn table_directory(&self, name: &str) -> &TableDirectory {
        let maybe_loca_table: Option<&TableDirectory> =
            self.table_dictionary.iter().find(|td| td.tag == name);
        maybe_loca_table.unwrap_or_else(|| panic!("'{name}' table not found"))
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
    fn from_file(reader: &mut Box<dyn Reader>) -> OffsetSubtable {
        let scaler_type = reader.read_u32();
        let num_tables = reader.read_u16();
        let search_range = reader.read_u16();
        let entry_selector = reader.read_u16();
        let range_shift = reader.read_u16();

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
pub struct TableDirectory {
    tag: String,     // 4-byte identifier
    checksum: u32,   // checksum for this table
    pub offset: u32, // offset from beginning of sfnt
    length: u32,     // length of this table in byte (actual length not padded length)
}

impl TableDirectory {
    fn from_file(reader: &mut Box<dyn Reader>) -> TableDirectory {
        let tag: String = reader.read_table_name();
        let checksum = reader.read_u32();
        let offset = reader.read_u32();
        let length = reader.read_u32();

        TableDirectory {
            tag,
            checksum,
            offset,
            length,
        }
    }
}
