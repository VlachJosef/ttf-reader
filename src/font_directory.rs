use crate::file_ops::FileOps;

#[allow(unused)]
pub struct FontDirectory {
    offset_subtable: OffsetSubtable,
    table_dictionary: Vec<TableDirectory>,
}

impl FontDirectory {
    pub fn from_file(file_ops: &mut FileOps) -> FontDirectory {
        let offset_subtable: OffsetSubtable = OffsetSubtable::from_file(file_ops);

        let table_dictionary: Vec<TableDirectory> = (0..offset_subtable.num_tables)
            .into_iter()
            .map(|_| TableDirectory::from_file(file_ops))
            .collect();

        FontDirectory {
            offset_subtable,
            table_dictionary,
        }
    }

    pub fn table_directory(&self, name: &str) -> &TableDirectory {
        let maybe_loca_table: Option<&TableDirectory> =
            self.table_dictionary.iter().find(|td| td.tag == name);
        maybe_loca_table.expect(&format!("'{name}' table not found"))
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
    fn from_file(file_ops: &mut FileOps) -> OffsetSubtable {
        let scaler_type = file_ops.read_u32();
        let num_tables = file_ops.read_u16();
        let search_range = file_ops.read_u16();
        let entry_selector = file_ops.read_u16();
        let range_shift = file_ops.read_u16();

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
    fn from_file(file_ops: &mut FileOps) -> TableDirectory {
        let tag: String = file_ops.read_table_name();
        let checksum = file_ops.read_u32();
        let offset = file_ops.read_u32();
        let length = file_ops.read_u32();

        TableDirectory {
            tag,
            checksum,
            offset,
            length,
        }
    }
}
