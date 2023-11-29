#[derive(Hash, PartialEq, Eq, Debug)]
#[repr(u8)]
pub enum PublishFlags {
    TopicTypePredefined = 1 << 0,
    TopicTypeShort = 1 << 1,
    CleanSession = 1 << 2,
    Will = 1 << 3,
    Retain = 1 << 4,
    QoS1 = 1 << 5,
    QoS2 = 1 << 6,
    QoSm1 = 1 << 5 | 1 << 6,
    DUP = 1 << 7,
}

pub enum QoSType {
    AtMostOnce,
    AtLeastOnce,
    ExactlyOnce,
}

pub enum TopicIdType {
    NormalTopic,
    PreDefinedTopic,
    ShortNameTopic,
}
