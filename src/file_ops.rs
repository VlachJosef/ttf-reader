use crate::model::{FWord, Fixed, PlatformId};
use std::fs::File;
use std::io::Read;
use std::io::Seek;
use std::io::SeekFrom;

pub struct FileOps {
    file: File,
}

impl FileOps {
    pub fn from_file(file: File) -> FileOps {
        FileOps { file }
    }

    pub fn seek_from_start(&mut self, seek_from: u32) {
        self.seek(SeekFrom::Start(seek_from as u64));
    }

    pub fn seek_from_current(&mut self, seek_from: i32) {
        self.seek(SeekFrom::Current(seek_from as i64));
    }

    fn seek(&mut self, seek_from: SeekFrom) {
        self.file.seek(seek_from).expect("Expected be able to seek");
    }

    pub fn read_end_code(&mut self, search_range: i32) -> u16 {
        self.read_offset(search_range)
    }

    pub fn read_start_code(&mut self, seg_count_x2: u16) -> u16 {
        self.seek_from_current(2); // Skip reservedPad
        self.read_offset(seg_count_x2 as i32)
    }

    pub fn read_id_delta(&mut self, seg_count_x2: u16) -> u16 {
        self.read_offset(seg_count_x2 as i32)
    }

    pub fn read_id_range_offset(&mut self, seg_count_x2: u16) -> u16 {
        self.read_offset(seg_count_x2 as i32)
    }

    fn read_offset(&mut self, offset: i32) -> u16 {
        self.seek_from_current(offset);

        let result = self.read_u16();

        self.seek_from_current(-2);

        result
    }

    pub fn read_address(&mut self) -> u64 {
        self.file
            .stream_position()
            .expect("Expected to read stream position")
    }

    pub fn read_platform_id(&mut self) -> PlatformId {
        let platform_id: u16 = self.read_u16();
        match platform_id {
            0 => PlatformId::Unicode,
            1 => PlatformId::Macintosh,
            2 => PlatformId::Reserved,
            3 => PlatformId::Microsoft,
            _ => panic!("Unknown PlatformId {:?}", platform_id),
        }
    }

    pub fn read_fword(&mut self) -> FWord {
        let fword = self.read_i16();
        FWord(fword)
    }

    pub fn read_long_date_time(&mut self) -> i64 {
        let mut buffer = [0; 8];
        self.file.read_exact(&mut buffer).expect("Can't read i64");
        i64::from_be_bytes(buffer)
    }

    pub fn read_fixed(&mut self) -> Fixed {
        let major = self.read_u16();
        let minor = self.read_u16();
        Fixed { major, minor }
    }

    pub fn read_u8(&mut self) -> u8 {
        let mut buffer = [0; 1];
        self.file.read_exact(&mut buffer).expect("Can't read u8");
        u8::from_be_bytes(buffer)
    }

    pub fn read_i8(&mut self) -> i8 {
        let mut buffer = [0; 1];
        self.file.read_exact(&mut buffer).expect("Can't read i8");
        i8::from_be_bytes(buffer)
    }

    pub fn read_u16(&mut self) -> u16 {
        let mut buffer = [0; 2];
        self.file.read_exact(&mut buffer).expect("Can't read u16");
        u16::from_be_bytes(buffer)
    }

    pub fn read_i16(&mut self) -> i16 {
        let mut buffer = [0; 2];
        self.file.read_exact(&mut buffer).expect("Can't read i16");
        i16::from_be_bytes(buffer)
    }

    pub fn read_u32(&mut self) -> u32 {
        let mut buffer = [0; 4];
        self.file
            .read_exact(&mut buffer)
            .expect("Exprected to read u32");
        u32::from_be_bytes(buffer)
    }

    pub fn read_table_name(&mut self) -> String {
        let mut tag = [0; 4];
        self.file
            .read_exact(&mut tag)
            .expect("Can't read table name");
        String::from_utf8_lossy(&tag).to_string()
    }
}
