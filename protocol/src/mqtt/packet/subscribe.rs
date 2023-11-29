#[derive(Clone, Copy)]
pub enum SubFlags {
    QoS1 = 1,
    QoS2 = 1 << 1,
    NoLocal = 1 << 2,
    RetainAsPlublished = 1 << 3,
    SendRetainedIfNotExists = 1 << 4,
    DontSendRetain = 1 << 5,
}
impl SubFlags {
    pub fn value(&self) -> u8 {
        *self as u8
    }
}
