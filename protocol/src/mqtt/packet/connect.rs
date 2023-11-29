#[derive(Clone, Copy)]
pub enum WillFlags {
    None = 0,
    WillFlag = 1 << 2,
    WillQos1 = 1 << 3,
    WillQos2 = 1 << 4,
    WillRetain = 1 << 5,
}
impl WillFlags {
    pub fn value(&self) -> u8 {
        *self as u8
    }
}

#[derive(Debug)]
pub struct Will<'w> {
    pub flags: u8,
    pub properties: Option<&'w [u8]>,
    pub topic: Option<&'w [u8]>,
    pub payload: Option<&'w [u8]>,
}
