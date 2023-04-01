use crate::model::{FWord, Fixed, PlatformId, UFWord};
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};

pub trait Reader {
    fn seek_from_start(&mut self, seek_from: u32);
    fn seek_from_current(&mut self, seek_from: i32);
    fn read_end_code(&mut self, search_range: i32) -> u16;
    fn read_start_code(&mut self, seg_count_x2: u16) -> u16;
    fn read_id_delta(&mut self, seg_count_x2: u16) -> u16;
    fn read_id_range_offset(&mut self, seg_count_x2: u16) -> u16;
    fn read_address(&mut self) -> u64;
    fn read_platform_id(&mut self) -> PlatformId;
    fn read_fword(&mut self) -> FWord;
    fn read_ufword(&mut self) -> UFWord;
    fn read_long_date_time(&mut self) -> i64;
    fn read_fixed(&mut self) -> Fixed;
    fn read_u8(&mut self) -> u8;
    fn read_i8(&mut self) -> i8;
    fn read_u16(&mut self) -> u16;
    fn read_i16(&mut self) -> i16;
    fn read_u32(&mut self) -> u32;
    fn read_table_name(&mut self) -> String;
    fn read_string(&mut self, length: u16) -> String;
    fn read_utf_16be(&mut self, length: u16) -> String;

    fn read_offset(&mut self, offset: i32) -> u16 {
        self.seek_from_current(offset);

        let result = self.read_u16();

        self.seek_from_current(-2);

        result
    }
}

impl Reader for FileOps {
    fn seek_from_start(&mut self, seek_from: u32) {
        self.seek(SeekFrom::Start(seek_from as u64));
    }

    fn seek_from_current(&mut self, seek_from: i32) {
        self.seek(SeekFrom::Current(seek_from as i64));
    }

    fn read_end_code(&mut self, search_range: i32) -> u16 {
        self.read_offset(search_range)
    }

    fn read_start_code(&mut self, seg_count_x2: u16) -> u16 {
        self.seek_from_current(2); // Skip reservedPad
        self.read_offset(seg_count_x2 as i32)
    }

    fn read_id_delta(&mut self, seg_count_x2: u16) -> u16 {
        self.read_offset(seg_count_x2 as i32)
    }

    fn read_id_range_offset(&mut self, seg_count_x2: u16) -> u16 {
        self.read_offset(seg_count_x2 as i32)
    }

    fn read_address(&mut self) -> u64 {
        self.file
            .stream_position()
            .expect("Expected to read stream position")
    }

    fn read_platform_id(&mut self) -> PlatformId {
        let platform_id: u16 = self.read_u16();
        match platform_id {
            0 => PlatformId::Unicode,
            1 => PlatformId::Macintosh,
            2 => PlatformId::Reserved,
            3 => PlatformId::Microsoft,
            _ => panic!("Unknown PlatformId {:?}", platform_id),
        }
    }

    fn read_fword(&mut self) -> FWord {
        let i16 = self.read_i16();
        FWord(i16)
    }

    fn read_ufword(&mut self) -> UFWord {
        let u16 = self.read_u16();
        UFWord(u16)
    }

    fn read_long_date_time(&mut self) -> i64 {
        let mut buffer = [0; 8];
        self.file.read_exact(&mut buffer).expect("Can't read i64");
        i64::from_be_bytes(buffer)
    }

    fn read_fixed(&mut self) -> Fixed {
        let major = self.read_u16();
        let minor = self.read_u16();
        Fixed { major, minor }
    }

    fn read_u8(&mut self) -> u8 {
        let mut buffer = [0; 1];
        self.file.read_exact(&mut buffer).expect("Can't read u8");
        u8::from_be_bytes(buffer)
    }

    fn read_i8(&mut self) -> i8 {
        let mut buffer = [0; 1];
        self.file.read_exact(&mut buffer).expect("Can't read i8");
        i8::from_be_bytes(buffer)
    }

    fn read_u16(&mut self) -> u16 {
        let mut buffer = [0; 2];
        self.file.read_exact(&mut buffer).expect("Can't read u16");
        u16::from_be_bytes(buffer)
    }

    fn read_i16(&mut self) -> i16 {
        let mut buffer = [0; 2];
        self.file.read_exact(&mut buffer).expect("Can't read i16");
        i16::from_be_bytes(buffer)
    }

    fn read_u32(&mut self) -> u32 {
        let mut buffer = [0; 4];
        self.file
            .read_exact(&mut buffer)
            .expect("Exprected to read u32");
        u32::from_be_bytes(buffer)
    }

    fn read_table_name(&mut self) -> String {
        let mut buffer = [0; 4];
        self.file
            .read_exact(&mut buffer)
            .expect("Can't read table name");
        String::from_utf8_lossy(&buffer).to_string()
    }

    fn read_string(&mut self, length: u16) -> String {
        let bytes: Vec<u8> = (0..length).into_iter().map(|_| self.read_u8()).collect();

        String::from_iter(bytes.iter().map(|ch| *ch as char))
    }

    fn read_utf_16be(&mut self, length: u16) -> String {
        let bytes: Vec<u16> = (0..(length / 2))
            .into_iter()
            .map(|_| self.read_u16())
            .collect();

        String::from_utf16(&bytes).unwrap()
    }
}

impl Reader for VecOps {
    fn seek_from_start(&mut self, seek_from: u32) {
        self.offset = seek_from as usize
    }
    fn seek_from_current(&mut self, seek_from: i32) {
        self.offset = (self.offset as i64 + seek_from as i64) as usize
    }
    fn read_end_code(&mut self, search_range: i32) -> u16 {
        self.read_offset(search_range)
    }
    fn read_start_code(&mut self, seg_count_x2: u16) -> u16 {
        self.seek_from_current(2); // Skip reservedPad
        self.read_offset(seg_count_x2 as i32)
    }
    fn read_id_delta(&mut self, seg_count_x2: u16) -> u16 {
        self.read_offset(seg_count_x2 as i32)
    }
    fn read_id_range_offset(&mut self, seg_count_x2: u16) -> u16 {
        self.read_offset(seg_count_x2 as i32)
    }
    fn read_address(&mut self) -> u64 {
        self.offset as u64
    }
    fn read_platform_id(&mut self) -> PlatformId {
        let platform_id: u16 = self.read_u16();
        match platform_id {
            0 => PlatformId::Unicode,
            1 => PlatformId::Macintosh,
            2 => PlatformId::Reserved,
            3 => PlatformId::Microsoft,
            _ => panic!("Unknown PlatformId {:?}", platform_id),
        }
    }
    fn read_fword(&mut self) -> FWord {
        let i16 = self.read_i16();
        FWord(i16)
    }
    fn read_ufword(&mut self) -> UFWord {
        let u16 = self.read_u16();
        UFWord(u16)
    }
    fn read_long_date_time(&mut self) -> i64 {
        let mut buffer = [0; 8];
        buffer[0] = self.data[self.offset];
        buffer[1] = self.data[self.offset + 1];
        buffer[2] = self.data[self.offset + 2];
        buffer[3] = self.data[self.offset + 3];
        buffer[4] = self.data[self.offset + 4];
        buffer[5] = self.data[self.offset + 5];
        buffer[6] = self.data[self.offset + 6];
        buffer[7] = self.data[self.offset + 7];
        self.offset = self.offset + 8;
        i64::from_be_bytes(buffer)
    }
    fn read_fixed(&mut self) -> Fixed {
        let major = self.read_u16();
        let minor = self.read_u16();
        Fixed { major, minor }
    }
    fn read_u8(&mut self) -> u8 {
        let mut buffer = [0; 1];
        buffer[0] = self.data[self.offset];
        self.offset = self.offset + 1;
        u8::from_be_bytes(buffer)
    }
    fn read_i8(&mut self) -> i8 {
        let mut buffer = [0; 1];
        buffer[0] = self.data[self.offset];
        self.offset = self.offset + 1;
        i8::from_be_bytes(buffer)
    }
    fn read_u16(&mut self) -> u16 {
        let mut buffer = [0; 2];
        buffer[0] = self.data[self.offset];
        buffer[1] = self.data[self.offset + 1];
        self.offset = self.offset + 2;
        u16::from_be_bytes(buffer)
    }
    fn read_i16(&mut self) -> i16 {
        let mut buffer = [0; 2];
        buffer[0] = self.data[self.offset];
        buffer[1] = self.data[self.offset + 1];
        self.offset = self.offset + 2;
        i16::from_be_bytes(buffer)
    }
    fn read_u32(&mut self) -> u32 {
        let mut buffer = [0; 4];
        buffer[0] = self.data[self.offset];
        buffer[1] = self.data[self.offset + 1];
        buffer[2] = self.data[self.offset + 2];
        buffer[3] = self.data[self.offset + 3];
        self.offset = self.offset + 4;
        u32::from_be_bytes(buffer)
    }
    fn read_table_name(&mut self) -> String {
        let mut buffer = [0; 4];
        buffer[0] = self.data[self.offset];
        buffer[1] = self.data[self.offset + 1];
        buffer[2] = self.data[self.offset + 2];
        buffer[3] = self.data[self.offset + 3];
        self.offset = self.offset + 4;
        String::from_utf8_lossy(&buffer).to_string()
    }
    fn read_string(&mut self, length: u16) -> String {
        let bytes: Vec<u8> = (0..length).into_iter().map(|_| self.read_u8()).collect();
        self.offset = self.offset + length as usize;
        String::from_iter(bytes.iter().map(|ch| *ch as char))
    }
    fn read_utf_16be(&mut self, length: u16) -> String {
        let bytes: Vec<u16> = (0..(length / 2))
            .into_iter()
            .map(|_| self.read_u16())
            .collect();

        self.offset = self.offset + length as usize;
        String::from_utf16(&bytes).unwrap()
    }
}

pub struct VecOps {
    data: Vec<u8>,
    offset: usize,
}

impl VecOps {
    pub fn from_vec(data: Vec<u8>) -> VecOps {
        let offset = 0;
        VecOps { data, offset }
    }
}

pub struct FileOps {
    file: File,
}

impl FileOps {
    pub fn from_file(file: File) -> FileOps {
        FileOps { file }
    }

    fn seek(&mut self, seek_from: SeekFrom) {
        self.file.seek(seek_from).expect("Expected be able to seek");
    }
}
