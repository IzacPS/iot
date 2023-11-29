#[derive(Clone, Copy)]
pub enum PublishFlags {
    Retain = 1,
    QoS1 = 1 << 1,
    QoS2 = 1 << 2,
    DUP = 1 << 3,
}

impl PublishFlags {
    pub fn value(&self) -> u8 {
        *self as u8
    }
}
