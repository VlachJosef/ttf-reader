#[derive(Debug)]
pub enum PlatformId {
    Unicode,
    Macintosh,
    Reserved,
    Microsoft,
}

#[derive(Debug)]
pub struct FWord(pub i16);

#[allow(unused)]
#[derive(Debug)]
pub struct Fixed {
    pub major: u16,
    pub minor: u16,
}
