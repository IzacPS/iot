use crate::PacketSerializer;

// use super::utils::TopicType;

#[repr(u8)]
pub enum Flags {
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

impl Flags {
    pub fn value(self) -> u8 {
        self as u8
    }
    pub fn mqtt_value(self) -> u8 {
        match self {
            Flags::Retain => 1,
            Flags::QoS1 => 1 << 1,
            Flags::QoS2 => 1 << 2,
            Flags::DUP => 1 << 3,
            _ => 0,
        }
    }
}

#[derive(Debug)]
pub enum PacketLengthType {
    Short,
    Long,
}

#[derive(Debug)]
pub enum PacketType<'a> {
    Advertise {
        len_type: PacketLengthType,
        gwid: u8,
        duration: u16,
    },
    Connack {
        len_type: PacketLengthType,
        return_code: u8,
    },

    Connect {
        len_type: PacketLengthType,
        flags: u8,
        protocolid: u8,
        duration: u16,
        client_id: Option<&'a [u8]>,
    },

    Disconnect {
        len_type: PacketLengthType,
        duration: u16,
    },

    ForwarderCapsule {
        len_type: PacketLengthType,
        wnode_id: Option<&'a [u8]>,
        mqttsn_msg: Option<&'a [u8]>,
        control: u8,
    },

    GwInfo {
        len_type: PacketLengthType,
        gwid: u8,
        gwaddr: Option<&'a [u8]>,
    },

    PingReq {
        len_type: PacketLengthType,
        client_id: Option<&'a [u8]>,
    },

    PingResp {
        len_type: PacketLengthType,
    },
    PubAck {
        len_type: PacketLengthType,
        topicid: u16,
        msgid: u16,
        return_code: u8,
    },

    PubComp {
        len_type: PacketLengthType,
        msgid: u16,
    },

    Publish {
        len_type: PacketLengthType,
        flags: u8,
        topicid: u16,
        msgid: u16,
        data: Option<&'a [u8]>,
    },

    PubRec {
        len_type: PacketLengthType,
        msgid: u16,
    },

    PubRel {
        len_type: PacketLengthType,
        msgid: u16,
    },

    RegAck {
        len_type: PacketLengthType,
        topicid: u16,
        msgid: u16,
        return_code: u8,
    },

    Register {
        len_type: PacketLengthType,
        topicid: u16,
        msgid: u16,
        topic_name: Option<&'a [u8]>,
    },

    SearchGw {
        len_type: PacketLengthType,
        radius: u8,
    },
    SubAck {
        len_type: PacketLengthType,
        flags: u8,
        topicid: u16,
        msgid: u16,
        return_code: u8,
    },

    Subscribe {
        len_type: PacketLengthType,
        flags: u8,
        msgid: u16,
        topic: &'a [u8],
    },

    UnsubAck {
        len_type: PacketLengthType,
        msgid: u16,
    },

    Unsubscribe {
        len_type: PacketLengthType,
        flags: u8,
        msgid: u16,
        topic: &'a [u8],
    },
    WillMsg {
        len_type: PacketLengthType,
        will_msg: Option<&'a [u8]>,
    },
    WillMsgReq {
        len_type: PacketLengthType,
    },
    WillMsgResp {
        len_type: PacketLengthType,
        return_code: u8,
    },
    WillMsgUpd {
        len_type: PacketLengthType,
        will_msg: Option<&'a [u8]>,
    },
    WillTopic {
        len_type: PacketLengthType,
        flags: u8,
        will_topic: Option<&'a [u8]>,
    },
    WillTopicReq {
        len_type: PacketLengthType,
    },
    WillTopicResp {
        len_type: PacketLengthType,
        return_code: u8,
    },
    WillTopicUpd {
        len_type: PacketLengthType,
        flags: u8,
        will_topic: Option<&'a [u8]>,
    },
    None,
}

impl<'a> PacketSerializer for PacketType<'a> {
    fn serialize(&self, buf: &mut [u8]) -> usize {
        let mut index: usize = 0;
        match self {
            PacketType::None => 0,
            PacketType::Advertise {
                len_type,
                gwid,
                duration,
            } => {
                let mut index: usize = 0;
                match len_type {
                    PacketLengthType::Short => {
                        buf[index] = 1 + 4;
                        index += 1;
                    }
                    PacketLengthType::Long => {
                        let len: u16 = 3 + 4;

                        buf[index] = 0x01;
                        index += 1;
                        #[cfg(target_endian = "big")]
                        {
                            buf[index] = len as u8;
                            index += 1;
                            buf[index] = (len >> 8) as u8;
                            index += 1;
                        }
                        #[cfg(target_endian = "little")]
                        {
                            buf[index] = (len >> 8) as u8;
                            index += 1;
                            buf[index] = len as u8;
                            index += 1;
                        }
                    }
                }
                buf[index] = 0x00;
                index += 1;
                buf[index] = *gwid;
                index += 1;
                #[cfg(target_endian = "big")]
                {
                    buf[index] = *duration as u8;
                    index += 1;
                    buf[index] = (*duration >> 8) as u8;
                    index += 1;
                }
                #[cfg(target_endian = "little")]
                {
                    buf[index] = (*duration >> 8) as u8;
                    index += 1;
                    buf[index] = *duration as u8;
                    index += 1;
                }
                index
            }
            PacketType::Connack {
                len_type,
                return_code,
            } => {
                match len_type {
                    PacketLengthType::Short => {
                        buf[index] = 1 + 2;
                        index += 1;
                    }
                    PacketLengthType::Long => {
                        let len: u16 = 3 + 2;
                        buf[index] = 0x01;
                        index += 1;
                        #[cfg(target_endian = "big")]
                        {
                            buf[index] = len as u8;
                            index += 1;
                            buf[index] = (len >> 8) as u8;
                            index += 1;
                        }
                        #[cfg(target_endian = "little")]
                        {
                            buf[index] = (len >> 8) as u8;
                            index += 1;
                            buf[index] = len as u8;
                            index += 1;
                        }
                    }
                }
                buf[index] = 0x05;
                index += 1;
                buf[index] = *return_code;
                index += 1;
                index
            }
            PacketType::Connect {
                len_type,
                flags,
                protocolid,
                duration,
                client_id,
            } => {
                match len_type {
                    PacketLengthType::Short => match *client_id {
                        Some(client_id) => {
                            buf[index] = 1 + 5 + client_id.len() as u8;
                            index += 1;
                        }
                        None => {
                            buf[index] = 1 + 5;
                            index += 1;
                        }
                    },
                    PacketLengthType::Long => {
                        let len = match *client_id {
                            Some(client_id) => 3 + 5 + client_id.len() as u16,
                            None => 3 + 5,
                        };
                        buf[index] = 0x01;
                        index += 1;
                        #[cfg(target_endian = "big")]
                        {
                            buf[index] = len as u8;
                            index += 1;
                            buf[index] = (len >> 8) as u8;
                            index += 1;
                        }
                        #[cfg(target_endian = "little")]
                        {
                            buf[index] = (len >> 8) as u8;
                            index += 1;
                            buf[index] = len as u8;
                            index += 1;
                        }
                    }
                }
                buf[index] = 0x04;
                index += 1;
                buf[index] = *flags;
                index += 1;
                buf[index] = *protocolid;
                index += 1;
                #[cfg(target_endian = "big")]
                {
                    buf[index] = *duration as u8;
                    index += 1;
                    buf[index] = (*duration >> 8) as u8;
                    index += 1;
                }
                #[cfg(target_endian = "little")]
                {
                    buf[index] = (*duration >> 8) as u8;
                    index += 1;
                    buf[index] = *duration as u8;
                    index += 1;
                }

                if let Some(client_id) = *client_id {
                    for c in client_id {
                        buf[index] = *c;
                        index += 1;
                    }
                }
                index
            }
            PacketType::Disconnect { len_type, duration } => {
                match len_type {
                    PacketLengthType::Short => {
                        buf[index] = 1 + 3;
                        index += 1;
                    }
                    PacketLengthType::Long => {
                        let len: u16 = 3 + 3;
                        buf[index] = 0x01;
                        index += 1;
                        #[cfg(target_endian = "big")]
                        {
                            buf[index] = len as u8;
                            index += 1;
                            buf[index] = (len >> 8) as u8;
                            index += 1;
                        }
                        #[cfg(target_endian = "little")]
                        {
                            buf[index] = (len >> 8) as u8;
                            index += 1;
                            buf[index] = len as u8;
                            index += 1;
                        }
                    }
                }
                buf[index] = 0x18;
                index += 1;
                #[cfg(target_endian = "big")]
                {
                    buf[index] = *duration as u8;
                    index += 1;
                    buf[index] = (*duration >> 8) as u8;
                    index += 1;
                }
                #[cfg(target_endian = "little")]
                {
                    buf[index] = (*duration >> 8) as u8;
                    index += 1;
                    buf[index] = *duration as u8;
                    index += 1;
                }
                index
            }
            PacketType::ForwarderCapsule {
                len_type,
                wnode_id,
                mqttsn_msg,
                control,
            } => {
                match len_type {
                    PacketLengthType::Short => {
                        let wnode_len = match *wnode_id {
                            Some(wnodeid) => wnodeid.len() as u8 + 1,
                            None => 0,
                        };
                        let msg_len = match *mqttsn_msg {
                            Some(msg) => msg.len() as u8,
                            None => 0,
                        };
                        buf[index] = 1 + 2 + wnode_len + msg_len;
                        index += 1;
                    }
                    PacketLengthType::Long => {
                        let wnode_len = match *wnode_id {
                            Some(wnodeid) => wnodeid.len() as u16 + 1,
                            None => 0,
                        };
                        let msg_len = match *mqttsn_msg {
                            Some(msg) => msg.len() as u16,
                            None => 0,
                        };
                        let len = 3 + 2 + wnode_len + msg_len;
                        buf[index] = 0x01;
                        index += 1;
                        #[cfg(target_endian = "big")]
                        {
                            buf[index] = len as u8;
                            index += 1;
                            buf[index] = (len >> 8) as u8;
                            index += 1;
                        }
                        #[cfg(target_endian = "little")]
                        {
                            buf[index] = (len >> 8) as u8;
                            index += 1;
                            buf[index] = len as u8;
                            index += 1;
                        }
                    }
                }
                buf[index] = 0xFE;
                index += 1;
                buf[index] = *control;
                index += 1;
                if let Some(wnode_id) = *wnode_id {
                    buf[index] = wnode_id.len() as u8;
                    index += 1;
                    for c in wnode_id {
                        buf[index] = *c;
                        index += 1;
                    }
                }
                if let Some(mqttsn_msg) = *mqttsn_msg {
                    for c in mqttsn_msg {
                        buf[index] = *c;
                        index += 1;
                    }
                }
                index
            }
            PacketType::GwInfo {
                len_type,
                gwid,
                gwaddr,
            } => {
                match len_type {
                    PacketLengthType::Short => match *gwaddr {
                        Some(gwaddr) => {
                            buf[index] = 1 + 2 + gwaddr.len() as u8;
                            index += 1;
                        }
                        None => {
                            buf[index] = 1 + 2;
                            index += 1;
                        }
                    },
                    PacketLengthType::Long => {
                        let len = match *gwaddr {
                            Some(gwaddr) => 3 + 2 + gwaddr.len() as u16,
                            None => 3 + 2,
                        };
                        buf[index] = 0x01;
                        index += 1;
                        #[cfg(target_endian = "big")]
                        {
                            buf[index] = len as u8;
                            index += 1;
                            buf[index] = (len >> 8) as u8;
                            index += 1;
                        }
                        #[cfg(target_endian = "little")]
                        {
                            buf[index] = (len >> 8) as u8;
                            index += 1;
                            buf[index] = len as u8;
                            index += 1;
                        }
                    }
                }
                buf[index] = 0x02;
                index += 1;
                buf[index] = *gwid;
                index += 1;
                if let Some(gwaddr) = *gwaddr {
                    for c in gwaddr {
                        buf[index] = *c;
                        index += 1;
                    }
                }
                index
            }
            PacketType::PingReq {
                len_type,
                client_id,
            } => {
                match len_type {
                    PacketLengthType::Short => match *client_id {
                        Some(client_id) => {
                            buf[index] = 1 + 1 + client_id.len() as u8;
                            index += 1;
                        }
                        None => {
                            buf[index] = 1 + 1;
                            index += 1;
                        }
                    },
                    PacketLengthType::Long => {
                        let len = match *client_id {
                            Some(client_id) => 3 + 1 + client_id.len() as u16,
                            None => 3 + 1,
                        };
                        buf[index] = 0x01;
                        index += 1;
                        #[cfg(target_endian = "big")]
                        {
                            buf[index] = len as u8;
                            index += 1;
                            buf[index] = (len >> 8) as u8;
                            index += 1;
                        }
                        #[cfg(target_endian = "little")]
                        {
                            buf[index] = (len >> 8) as u8;
                            index += 1;
                            buf[index] = len as u8;
                            index += 1;
                        }
                    }
                }
                buf[index] = 0x16;
                index += 1;
                if let Some(client_id) = *client_id {
                    for c in client_id {
                        buf[index] = *c;
                        index += 1;
                    }
                }
                index
            }
            PacketType::PingResp { len_type } => {
                match len_type {
                    PacketLengthType::Short => {
                        buf[index] = 1 + 1;
                        index += 1;
                    }
                    PacketLengthType::Long => {
                        let len: u16 = 3 + 1;
                        buf[index] = 0x01;
                        index += 1;
                        #[cfg(target_endian = "big")]
                        {
                            buf[index] = len as u8;
                            index += 1;
                            buf[index] = (len >> 8) as u8;
                            index += 1;
                        }
                        #[cfg(target_endian = "little")]
                        {
                            buf[index] = (len >> 8) as u8;
                            index += 1;
                            buf[index] = len as u8;
                            index += 1;
                        }
                    }
                }
                buf[index] = 0x17;
                index += 1;
                index
            }
            PacketType::PubAck {
                len_type,
                topicid,
                msgid,
                return_code,
            } => {
                match len_type {
                    PacketLengthType::Short => {
                        buf[index] = 1 + 6;
                        index += 1;
                    }
                    PacketLengthType::Long => {
                        let len: u16 = 3 + 6;
                        buf[index] = 0x01;
                        index += 1;
                        #[cfg(target_endian = "big")]
                        {
                            buf[index] = len as u8;
                            index += 1;
                            buf[index] = (len >> 8) as u8;
                            index += 1;
                        }
                        #[cfg(target_endian = "little")]
                        {
                            buf[index] = (len >> 8) as u8;
                            index += 1;
                            buf[index] = len as u8;
                            index += 1;
                        }
                    }
                }
                buf[index] = 0x0D;
                index += 1;
                #[cfg(target_endian = "big")]
                {
                    buf[index] = *topicid as u8;
                    index += 1;
                    buf[index] = (*topicid >> 8) as u8;
                    index += 1;
                }
                #[cfg(target_endian = "little")]
                {
                    buf[index] = (*topicid >> 8) as u8;
                    index += 1;
                    buf[index] = *topicid as u8;
                    index += 1;
                }

                #[cfg(target_endian = "big")]
                {
                    buf[index] = *msgid as u8;
                    index += 1;
                    buf[index] = (*msgid >> 8) as u8;
                    index += 1;
                }
                #[cfg(target_endian = "little")]
                {
                    buf[index] = (*msgid >> 8) as u8;
                    index += 1;
                    buf[index] = *msgid as u8;
                    index += 1;
                }

                buf[index] = *return_code;
                index += 1;
                index
            }
            PacketType::PubComp { len_type, msgid } => {
                match len_type {
                    PacketLengthType::Short => {
                        buf[index] = 1 + 3;
                        index += 1;
                    }
                    PacketLengthType::Long => {
                        let len: u16 = 3 + 3;
                        buf[index] = 0x01;
                        index += 1;
                        #[cfg(target_endian = "big")]
                        {
                            buf[index] = len as u8;
                            index += 1;
                            buf[index] = (len >> 8) as u8;
                            index += 1;
                        }
                        #[cfg(target_endian = "little")]
                        {
                            buf[index] = (len >> 8) as u8;
                            index += 1;
                            buf[index] = len as u8;
                            index += 1;
                        }
                    }
                }
                buf[index] = 0x0E;
                index += 1;
                #[cfg(target_endian = "big")]
                {
                    buf[index] = *msgid as u8;
                    index += 1;
                    buf[index] = (*msgid >> 8) as u8;
                    index += 1;
                }
                #[cfg(target_endian = "little")]
                {
                    buf[index] = (*msgid >> 8) as u8;
                    index += 1;
                    buf[index] = *msgid as u8;
                    index += 1;
                }
                index
            }
            PacketType::Publish {
                len_type,
                flags,
                topicid,
                msgid,
                data,
            } => {
                match len_type {
                    PacketLengthType::Short => match *data {
                        Some(data) => {
                            buf[index] = 1 + 6 + data.len() as u8;
                            index += 1;
                        }
                        None => {
                            buf[index] = 1 + 6;
                            index += 1;
                        }
                    },
                    PacketLengthType::Long => {
                        let len = match *data {
                            Some(data) => 3 + 6 + data.len() as u16,
                            None => 3 + 6,
                        };
                        buf[index] = 0x01;
                        index += 1;
                        #[cfg(target_endian = "big")]
                        {
                            buf[index] = len as u8;
                            index += 1;
                            buf[index] = (len >> 8) as u8;
                            index += 1;
                        }
                        #[cfg(target_endian = "little")]
                        {
                            buf[index] = (len >> 8) as u8;
                            index += 1;
                            buf[index] = len as u8;
                            index += 1;
                        }
                    }
                }
                buf[index] = 0x0C;
                index += 1;
                buf[index] = *flags;
                index += 1;
                #[cfg(target_endian = "big")]
                {
                    buf[index] = *topicid as u8;
                    index += 1;
                    buf[index] = (*topicid >> 8) as u8;
                    index += 1;
                }
                #[cfg(target_endian = "little")]
                {
                    buf[index] = (*topicid >> 8) as u8;
                    index += 1;
                    buf[index] = *topicid as u8;
                    index += 1;
                }

                #[cfg(target_endian = "big")]
                {
                    buf[index] = *msgid as u8;
                    index += 1;
                    buf[index] = (*msgid >> 8) as u8;
                    index += 1;
                }
                #[cfg(target_endian = "little")]
                {
                    buf[index] = (*msgid >> 8) as u8;
                    index += 1;
                    buf[index] = *msgid as u8;
                    index += 1;
                }

                if let Some(data) = *data {
                    for c in data {
                        buf[index] = *c;
                        index += 1;
                    }
                }
                index
            }
            PacketType::PubRec { len_type, msgid } => {
                match len_type {
                    PacketLengthType::Short => {
                        buf[index] = 1 + 3;
                        index += 1;
                    }
                    PacketLengthType::Long => {
                        let len: u16 = 3 + 3;
                        buf[index] = 0x01;
                        index += 1;
                        #[cfg(target_endian = "big")]
                        {
                            buf[index] = len as u8;
                            index += 1;
                            buf[index] = (len >> 8) as u8;
                            index += 1;
                        }
                        #[cfg(target_endian = "little")]
                        {
                            buf[index] = (len >> 8) as u8;
                            index += 1;
                            buf[index] = len as u8;
                            index += 1;
                        }
                    }
                }
                buf[index] = 0x0F;
                index += 1;

                #[cfg(target_endian = "big")]
                {
                    buf[index] = *msgid as u8;
                    index += 1;
                    buf[index] = (*msgid >> 8) as u8;
                    index += 1;
                }
                #[cfg(target_endian = "little")]
                {
                    buf[index] = (*msgid >> 8) as u8;
                    index += 1;
                    buf[index] = *msgid as u8;
                    index += 1;
                }

                index
            }
            PacketType::PubRel { len_type, msgid } => {
                match len_type {
                    PacketLengthType::Short => {
                        buf[index] = 1 + 3;
                        index += 1;
                    }
                    PacketLengthType::Long => {
                        let len: u16 = 3 + 3;
                        buf[index] = 0x01;
                        index += 1;
                        #[cfg(target_endian = "big")]
                        {
                            buf[index] = len as u8;
                            index += 1;
                            buf[index] = (len >> 8) as u8;
                            index += 1;
                        }
                        #[cfg(target_endian = "little")]
                        {
                            buf[index] = (len >> 8) as u8;
                            index += 1;
                            buf[index] = len as u8;
                            index += 1;
                        }
                    }
                }
                buf[index] = 0x10;
                index += 1;

                #[cfg(target_endian = "big")]
                {
                    buf[index] = *msgid as u8;
                    index += 1;
                    buf[index] = (*msgid >> 8) as u8;
                    index += 1;
                }
                #[cfg(target_endian = "little")]
                {
                    buf[index] = (*msgid >> 8) as u8;
                    index += 1;
                    buf[index] = *msgid as u8;
                    index += 1;
                }
                index
            }
            PacketType::RegAck {
                len_type,
                topicid,
                msgid,
                return_code,
            } => {
                match len_type {
                    PacketLengthType::Short => {
                        buf[index] = 1 + 6;
                        index += 1;
                    }
                    PacketLengthType::Long => {
                        let len: u16 = 3 + 6;
                        buf[index] = 0x01;
                        index += 1;
                        #[cfg(target_endian = "big")]
                        {
                            buf[index] = len as u8;
                            index += 1;
                            buf[index] = (len >> 8) as u8;
                            index += 1;
                        }
                        #[cfg(target_endian = "little")]
                        {
                            buf[index] = (len >> 8) as u8;
                            index += 1;
                            buf[index] = len as u8;
                            index += 1;
                        }
                    }
                }
                buf[index] = 0x0B;
                index += 1;
                #[cfg(target_endian = "big")]
                {
                    buf[index] = *topicid as u8;
                    index += 1;
                    buf[index] = (*topicid >> 8) as u8;
                    index += 1;
                }
                #[cfg(target_endian = "little")]
                {
                    buf[index] = (*topicid >> 8) as u8;
                    index += 1;
                    buf[index] = *topicid as u8;
                    index += 1;
                }

                #[cfg(target_endian = "big")]
                {
                    buf[index] = *msgid as u8;
                    index += 1;
                    buf[index] = (*msgid >> 8) as u8;
                    index += 1;
                }
                #[cfg(target_endian = "little")]
                {
                    buf[index] = (*msgid >> 8) as u8;
                    index += 1;
                    buf[index] = *msgid as u8;
                    index += 1;
                }

                buf[index] = *return_code;
                index += 1;
                index
            }
            PacketType::Register {
                len_type,
                topicid,
                msgid,
                topic_name,
            } => {
                match len_type {
                    PacketLengthType::Short => match *topic_name {
                        Some(tname) => {
                            buf[index] = 1 + 5 + tname.len() as u8;
                            index += 1;
                        }
                        None => {
                            buf[index] = 1 + 5;
                            index += 1;
                        }
                    },
                    PacketLengthType::Long => {
                        let len = match *topic_name {
                            Some(tname) => 3 + 5 + tname.len() as u16,
                            None => 3 + 5,
                        };
                        buf[index] = 0x01;
                        index += 1;
                        #[cfg(target_endian = "big")]
                        {
                            buf[index] = len as u8;
                            index += 1;
                            buf[index] = (len >> 8) as u8;
                            index += 1;
                        }
                        #[cfg(target_endian = "little")]
                        {
                            buf[index] = (len >> 8) as u8;
                            index += 1;
                            buf[index] = len as u8;
                            index += 1;
                        }
                    }
                }
                buf[index] = 0x0A;
                index += 1;

                #[cfg(target_endian = "big")]
                {
                    buf[index] = *topicid as u8;
                    index += 1;
                    buf[index] = (*topicid >> 8) as u8;
                    index += 1;
                }
                #[cfg(target_endian = "little")]
                {
                    buf[index] = (*topicid >> 8) as u8;
                    index += 1;
                    buf[index] = *topicid as u8;
                    index += 1;
                }

                #[cfg(target_endian = "big")]
                {
                    buf[index] = *msgid as u8;
                    index += 1;
                    buf[index] = (*msgid >> 8) as u8;
                    index += 1;
                }
                #[cfg(target_endian = "little")]
                {
                    buf[index] = (*msgid >> 8) as u8;
                    index += 1;
                    buf[index] = *msgid as u8;
                    index += 1;
                }

                if let Some(topic_name) = *topic_name {
                    for c in topic_name {
                        buf[index] = *c;
                        index += 1;
                    }
                }
                index
            }
            PacketType::SearchGw { len_type, radius } => {
                match len_type {
                    PacketLengthType::Short => {
                        buf[index] = 1 + 2;
                        index += 1;
                    }
                    PacketLengthType::Long => {
                        let len: u16 = 3 + 2;
                        buf[index] = 0x01;
                        index += 1;
                        #[cfg(target_endian = "big")]
                        {
                            buf[index] = len as u8;
                            index += 1;
                            buf[index] = (len >> 8) as u8;
                            index += 1;
                        }
                        #[cfg(target_endian = "little")]
                        {
                            buf[index] = (len >> 8) as u8;
                            index += 1;
                            buf[index] = len as u8;
                            index += 1;
                        }
                    }
                }
                buf[index] = 0x01;
                index += 1;
                buf[index] = *radius;
                index += 1;
                index
            }
            PacketType::SubAck {
                len_type,
                flags,
                topicid,
                msgid,
                return_code,
            } => {
                match len_type {
                    PacketLengthType::Short => {
                        buf[index] = 1 + 7;
                        index += 1;
                    }
                    PacketLengthType::Long => {
                        let len: u16 = 3 + 7;
                        buf[index] = 0x01;
                        index += 1;
                        #[cfg(target_endian = "big")]
                        {
                            buf[index] = len as u8;
                            index += 1;
                            buf[index] = (len >> 8) as u8;
                            index += 1;
                        }
                        #[cfg(target_endian = "little")]
                        {
                            buf[index] = (len >> 8) as u8;
                            index += 1;
                            buf[index] = len as u8;
                            index += 1;
                        }
                    }
                }
                buf[index] = 0x13;
                index += 1;
                buf[index] = *flags;
                index += 1;

                #[cfg(target_endian = "big")]
                {
                    buf[index] = *topicid as u8;
                    index += 1;
                    buf[index] = (*topicid >> 8) as u8;
                    index += 1;
                }
                #[cfg(target_endian = "little")]
                {
                    buf[index] = (*topicid >> 8) as u8;
                    index += 1;
                    buf[index] = *topicid as u8;
                    index += 1;
                }

                #[cfg(target_endian = "big")]
                {
                    buf[index] = *msgid as u8;
                    index += 1;
                    buf[index] = (*msgid >> 8) as u8;
                    index += 1;
                }
                #[cfg(target_endian = "little")]
                {
                    buf[index] = (*msgid >> 8) as u8;
                    index += 1;
                    buf[index] = *msgid as u8;
                    index += 1;
                }
                buf[index] = *return_code;
                index += 1;
                index
            }
            PacketType::Subscribe {
                len_type,
                flags,
                msgid,
                topic,
            } => {
                match len_type {
                    PacketLengthType::Short => match flags & 0x03 {
                        0x00 => {
                            buf[index] = 1 + 4 + topic.len() as u8 + 2;
                            index += 1;
                        }
                        0x01 | 0x02 => {
                            buf[index] = 1 + 4 + 2;
                            index += 1;
                        }
                        _ => {
                            return 0;
                            // buf[index] = 1 + 4;
                            // index += 1;
                        }
                    },
                    PacketLengthType::Long => {
                        let len = match flags & 0x03 {
                            0x00 => 3 + 4 + topic.len() as u16 + 2,
                            0x01 | 0x02 => 3 + 4 + 2,
                            _ => return 0,
                        };
                        buf[index] = 0x01;
                        index += 1;
                        #[cfg(target_endian = "big")]
                        {
                            buf[index] = len as u8;
                            index += 1;
                            buf[index] = (len >> 8) as u8;
                            index += 1;
                        }
                        #[cfg(target_endian = "little")]
                        {
                            buf[index] = (len >> 8) as u8;
                            index += 1;
                            buf[index] = len as u8;
                            index += 1;
                        }
                    }
                }
                buf[index] = 0x12;
                index += 1;
                buf[index] = *flags;
                index += 1;

                #[cfg(target_endian = "big")]
                {
                    buf[index] = *msgid as u8;
                    index += 1;
                    buf[index] = (*msgid >> 8) as u8;
                    index += 1;
                }
                #[cfg(target_endian = "little")]
                {
                    buf[index] = (*msgid >> 8) as u8;
                    index += 1;
                    buf[index] = *msgid as u8;
                    index += 1;
                }

                match flags & 0x03 {
                    0x00 => {
                        let len = topic.len();
                        #[cfg(target_endian = "big")]
                        {
                            buf[index] = len as u8;
                            index += 1;
                            buf[index] = (len >> 8) as u8;
                            index += 1;
                        }
                        #[cfg(target_endian = "little")]
                        {
                            buf[index] = (len >> 8) as u8;
                            index += 1;
                            buf[index] = len as u8;
                            index += 1;
                        }
                        for c in *topic {
                            buf[index] = *c;
                            index += 1;
                        }
                    }
                    0x01 | 0x02 => {
                        for c in *topic {
                            buf[index] = *c;
                            index += 1;
                        }
                    }
                    _ => return 0,
                }
                index
            }
            PacketType::UnsubAck { len_type, msgid } => {
                match len_type {
                    PacketLengthType::Short => {
                        buf[index] = 1 + 3;
                        index += 1;
                    }
                    PacketLengthType::Long => {
                        let len: u16 = 3 + 3;
                        buf[index] = 0x01;
                        index += 1;
                        #[cfg(target_endian = "big")]
                        {
                            buf[index] = len as u8;
                            index += 1;
                            buf[index] = (len >> 8) as u8;
                            index += 1;
                        }
                        #[cfg(target_endian = "little")]
                        {
                            buf[index] = (len >> 8) as u8;
                            index += 1;
                            buf[index] = len as u8;
                            index += 1;
                        }
                    }
                }
                buf[index] = 0x15;
                index += 1;
                #[cfg(target_endian = "big")]
                {
                    buf[index] = *msgid as u8;
                    index += 1;
                    buf[index] = (*msgid >> 8) as u8;
                    index += 1;
                }
                #[cfg(target_endian = "little")]
                {
                    buf[index] = (*msgid >> 8) as u8;
                    index += 1;
                    buf[index] = *msgid as u8;
                    index += 1;
                }
                index
            }
            PacketType::Unsubscribe {
                len_type,
                flags,
                msgid,
                topic,
            } => {
                match len_type {
                    PacketLengthType::Short => match flags & 0x03 {
                        0x00 => {
                            buf[index] = 1 + 4 + topic.len() as u8 + 2;
                            index += 1;
                        }
                        0x01 | 0x02 => {
                            buf[index] = 1 + 4 + 2;
                            index += 1;
                        }
                        _ => {
                            return 0;
                            // buf[index] = 1 + 4;
                            // index += 1;
                        }
                    },
                    PacketLengthType::Long => {
                        let len = match flags & 0x03 {
                            0x00 => 3 + 4 + topic.len() as u16 + 2,
                            0x01 | 0x02 => 3 + 4 + 2,
                            _ => return 0,
                        };
                        buf[index] = 0x01;
                        index += 1;
                        #[cfg(target_endian = "big")]
                        {
                            buf[index] = len as u8;
                            index += 1;
                            buf[index] = (len >> 8) as u8;
                            index += 1;
                        }
                        #[cfg(target_endian = "little")]
                        {
                            buf[index] = (len >> 8) as u8;
                            index += 1;
                            buf[index] = len as u8;
                            index += 1;
                        }
                    }
                }
                buf[index] = 0x14;
                index += 1;
                buf[index] = *flags;
                index += 1;

                #[cfg(target_endian = "big")]
                {
                    buf[index] = *msgid as u8;
                    index += 1;
                    buf[index] = (*msgid >> 8) as u8;
                    index += 1;
                }
                #[cfg(target_endian = "little")]
                {
                    buf[index] = (*msgid >> 8) as u8;
                    index += 1;
                    buf[index] = *msgid as u8;
                    index += 1;
                }

                match flags & 0x03 {
                    0x00 => {
                        let len = topic.len();
                        #[cfg(target_endian = "big")]
                        {
                            buf[index] = len as u8;
                            index += 1;
                            buf[index] = (len >> 8) as u8;
                            index += 1;
                        }
                        #[cfg(target_endian = "little")]
                        {
                            buf[index] = (len >> 8) as u8;
                            index += 1;
                            buf[index] = len as u8;
                            index += 1;
                        }
                        for c in *topic {
                            buf[index] = *c;
                            index += 1;
                        }
                    }
                    0x01 | 0x02 => {
                        for c in *topic {
                            buf[index] = *c;
                            index += 1;
                        }
                    }
                    _ => return 0,
                }
                index
            }
            PacketType::WillMsg { len_type, will_msg } => {
                match len_type {
                    PacketLengthType::Short => match *will_msg {
                        Some(wmsg) => {
                            buf[index] = 1 + 1 + wmsg.len() as u8;
                            index += 1;
                        }
                        None => {
                            buf[index] = 1 + 1;
                            index += 1;
                        }
                    },
                    PacketLengthType::Long => {
                        let len = match *will_msg {
                            Some(wmsg) => 3 + 1 + wmsg.len() as u16,
                            None => 3 + 1,
                        };
                        buf[index] = 0x01;
                        index += 1;
                        #[cfg(target_endian = "big")]
                        {
                            buf[index] = len as u8;
                            index += 1;
                            buf[index] = (len >> 8) as u8;
                            index += 1;
                        }
                        #[cfg(target_endian = "little")]
                        {
                            buf[index] = (len >> 8) as u8;
                            index += 1;
                            buf[index] = len as u8;
                            index += 1;
                        }
                    }
                }
                buf[index] = 0x09;
                index += 1;
                if let Some(will_msg) = *will_msg {
                    for c in will_msg {
                        buf[index] = *c;
                        index += 1;
                    }
                }
                index
            }
            PacketType::WillMsgReq { len_type } => {
                match len_type {
                    PacketLengthType::Short => {
                        buf[index] = 1 + 1;
                        index += 1;
                    }
                    PacketLengthType::Long => {
                        let len: u16 = 3 + 1;
                        buf[index] = 0x01;
                        index += 1;
                        #[cfg(target_endian = "big")]
                        {
                            buf[index] = len as u8;
                            index += 1;
                            buf[index] = (len >> 8) as u8;
                            index += 1;
                        }
                        #[cfg(target_endian = "little")]
                        {
                            buf[index] = (len >> 8) as u8;
                            index += 1;
                            buf[index] = len as u8;
                            index += 1;
                        }
                    }
                }
                buf[index] = 0x08;
                index += 1;
                index
            }
            PacketType::WillMsgResp {
                len_type,
                return_code,
            } => {
                match len_type {
                    PacketLengthType::Short => {
                        buf[index] = 1 + 2;
                        index += 1;
                    }
                    PacketLengthType::Long => {
                        let len: u16 = 3 + 2;
                        buf[index] = 0x01;
                        index += 1;
                        #[cfg(target_endian = "big")]
                        {
                            buf[index] = len as u8;
                            index += 1;
                            buf[index] = (len >> 8) as u8;
                            index += 1;
                        }
                        #[cfg(target_endian = "little")]
                        {
                            buf[index] = (len >> 8) as u8;
                            index += 1;
                            buf[index] = len as u8;
                            index += 1;
                        }
                    }
                }
                buf[index] = 0x1D;
                index += 1;
                buf[index] = *return_code;
                index += 1;
                index
            }
            PacketType::WillMsgUpd { len_type, will_msg } => {
                match len_type {
                    PacketLengthType::Short => match *will_msg {
                        Some(wtopic) => {
                            buf[index] = 1 + 1 + wtopic.len() as u8;
                            index += 1;
                        }
                        None => {
                            buf[index] = 1 + 1;
                            index += 1;
                        }
                    },
                    PacketLengthType::Long => {
                        let len = match *will_msg {
                            Some(wtopic) => 3 + 1 + wtopic.len() as u16,
                            None => 3 + 1,
                        };
                        buf[index] = 0x01;
                        index += 1;
                        #[cfg(target_endian = "big")]
                        {
                            buf[index] = len as u8;
                            index += 1;
                            buf[index] = (len >> 8) as u8;
                            index += 1;
                        }
                        #[cfg(target_endian = "little")]
                        {
                            buf[index] = (len >> 8) as u8;
                            index += 1;
                            buf[index] = len as u8;
                            index += 1;
                        }
                    }
                }
                buf[index] = 0x1C;
                index += 1;
                if let Some(will_msg) = *will_msg {
                    for c in will_msg {
                        buf[index] = *c;
                        index += 1;
                    }
                }
                index
            }
            PacketType::WillTopic {
                len_type,
                flags,
                will_topic,
            } => {
                match len_type {
                    PacketLengthType::Short => match *will_topic {
                        Some(wtopic) => {
                            buf[index] = 1 + 2 + wtopic.len() as u8;
                            index += 1;
                        }
                        None => {
                            buf[index] = 1 + 2;
                            index += 1;
                        }
                    },
                    PacketLengthType::Long => {
                        let len = match *will_topic {
                            Some(wtopic) => 3 + 2 + wtopic.len() as u16,
                            None => 3 + 2,
                        };
                        buf[index] = 0x01;
                        index += 1;
                        #[cfg(target_endian = "big")]
                        {
                            buf[index] = len as u8;
                            index += 1;
                            buf[index] = (len >> 8) as u8;
                            index += 1;
                        }
                        #[cfg(target_endian = "little")]
                        {
                            buf[index] = (len >> 8) as u8;
                            index += 1;
                            buf[index] = len as u8;
                            index += 1;
                        }
                    }
                }
                buf[index] = 0x07;
                index += 1;
                buf[index] = *flags;
                index += 1;
                if let Some(will_topic) = *will_topic {
                    for c in will_topic {
                        buf[index] = *c;
                        index += 1;
                    }
                }
                index
            }
            PacketType::WillTopicReq { len_type } => {
                match len_type {
                    PacketLengthType::Short => {
                        buf[index] = 1 + 1;
                        index += 1;
                    }
                    PacketLengthType::Long => {
                        let len: u16 = 3 + 1;
                        buf[index] = 0x01;
                        index += 1;
                        #[cfg(target_endian = "big")]
                        {
                            buf[index] = len as u8;
                            index += 1;
                            buf[index] = (len >> 8) as u8;
                            index += 1;
                        }
                        #[cfg(target_endian = "little")]
                        {
                            buf[index] = (len >> 8) as u8;
                            index += 1;
                            buf[index] = len as u8;
                            index += 1;
                        }
                    }
                }
                buf[index] = 0x06;
                index += 1;
                index
            }
            PacketType::WillTopicResp {
                len_type,
                return_code,
            } => {
                match len_type {
                    PacketLengthType::Short => {
                        buf[index] = 1 + 2;
                        index += 1;
                    }
                    PacketLengthType::Long => {
                        let len: u16 = 3 + 2;
                        buf[index] = 0x01;
                        index += 1;
                        #[cfg(target_endian = "big")]
                        {
                            buf[index] = len as u8;
                            index += 1;
                            buf[index] = (len >> 8) as u8;
                            index += 1;
                        }
                        #[cfg(target_endian = "little")]
                        {
                            buf[index] = (len >> 8) as u8;
                            index += 1;
                            buf[index] = len as u8;
                            index += 1;
                        }
                    }
                }
                buf[index] = 0x1B;
                index += 1;
                buf[index] = *return_code;
                index += 1;
                index
            }
            PacketType::WillTopicUpd {
                len_type,
                flags,
                will_topic,
            } => {
                match len_type {
                    PacketLengthType::Short => match *will_topic {
                        Some(wtopic) => {
                            buf[index] = 1 + 2 + wtopic.len() as u8;
                            index += 1;
                        }
                        None => {
                            buf[index] = 1 + 2;
                            index += 1;
                        }
                    },
                    PacketLengthType::Long => {
                        let len = match *will_topic {
                            Some(wtopic) => 3 + 2 + wtopic.len() as u16,
                            None => 3 + 2,
                        };
                        buf[index] = 0x01;
                        index += 1;
                        #[cfg(target_endian = "big")]
                        {
                            buf[index] = len as u8;
                            index += 1;
                            buf[index] = (len >> 8) as u8;
                            index += 1;
                        }
                        #[cfg(target_endian = "little")]
                        {
                            buf[index] = (len >> 8) as u8;
                            index += 1;
                            buf[index] = len as u8;
                            index += 1;
                        }
                    }
                }
                buf[index] = 0x1A;
                index += 1;
                buf[index] = *flags;
                index += 1;
                if let Some(will_topic) = *will_topic {
                    for c in will_topic {
                        buf[index] = *c;
                        index += 1;
                    }
                }
                index
            }
        }
    }
}

impl<'a> PacketType<'a> {
    pub fn deserialize(arr: &'a [u8]) -> Self {
        let (len_type, data) = match arr[0] {
            0x01 => (PacketLengthType::Long, &arr[3..]),
            _ => (PacketLengthType::Short, &arr[1..]),
        };
        match data[0] {
            0x00 => {
                if data.len() < 4 {
                    return Self::None;
                }
                Self::Advertise {
                    len_type,
                    gwid: data[1],
                    duration: u16::from_be_bytes([data[2], data[3]]),
                }
            }
            0x01 => {
                if data.len() < 2 {
                    return Self::None;
                }
                Self::SearchGw {
                    len_type,
                    radius: data[1],
                }
            }
            0x02 => {
                if data.len() < 2 {
                    return Self::None;
                }
                Self::GwInfo {
                    len_type,
                    gwid: data[1],
                    gwaddr: Some(&data[2..]),
                }
            }
            0x04 => {
                if data.len() < 5 {
                    return Self::None;
                }
                Self::Connect {
                    len_type,
                    flags: data[1],
                    protocolid: data[2],
                    duration: u16::from_be_bytes([data[3], data[4]]),
                    client_id: Some(&data[5..]),
                }
            }
            0x05 => {
                if data.len() < 2 {
                    return Self::None;
                }
                Self::Connack {
                    len_type,
                    return_code: data[1],
                }
            }
            0x06 => {
                if data.len() < 1 {
                    return Self::None;
                }
                Self::WillTopicReq { len_type }
            }
            0x07 => {
                if data.len() < 2 {
                    return Self::None;
                }
                Self::WillTopic {
                    len_type,
                    flags: data[1],
                    will_topic: Some(&data[2..]),
                }
            }
            0x08 => {
                if data.len() < 1 {
                    return Self::None;
                }
                Self::WillMsgReq { len_type }
            }
            0x09 => {
                if data.len() < 1 {
                    return Self::None;
                }
                Self::WillMsg {
                    len_type,
                    will_msg: Some(&data[1..]),
                }
            }
            0x0A => {
                if data.len() < 5 {
                    return Self::None;
                }
                Self::Register {
                    len_type,
                    topicid: u16::from_be_bytes([data[1], data[2]]),
                    msgid: u16::from_be_bytes([data[3], data[4]]),
                    topic_name: Some(&data[5..]),
                }
            }
            0x0B => {
                if data.len() < 6 {
                    return Self::None;
                }
                Self::RegAck {
                    len_type,
                    topicid: u16::from_be_bytes([data[1], data[2]]),
                    msgid: u16::from_be_bytes([data[3], data[4]]),
                    return_code: data[5],
                }
            }
            0x0C => {
                if data.len() < 6 {
                    return Self::None;
                }
                Self::Publish {
                    len_type,
                    flags: data[1],
                    topicid: u16::from_be_bytes([data[2], data[3]]),
                    msgid: u16::from_be_bytes([data[4], data[5]]),
                    data: Some(&data[6..]),
                }
            }
            0x0D => {
                if data.len() < 6 {
                    return Self::None;
                }
                Self::PubAck {
                    len_type,
                    topicid: u16::from_be_bytes([data[1], data[2]]),
                    msgid: u16::from_be_bytes([data[3], data[4]]),
                    return_code: data[5],
                }
            }
            0x0E => {
                if data.len() < 3 {
                    return Self::None;
                }
                Self::PubComp {
                    len_type,
                    msgid: u16::from_be_bytes([data[1], data[2]]),
                }
            }
            0x0F => {
                if data.len() < 3 {
                    return Self::None;
                }
                Self::PubRec {
                    len_type,
                    msgid: u16::from_be_bytes([data[1], data[2]]),
                }
            }
            0x10 => {
                if data.len() < 3 {
                    return Self::None;
                }
                Self::PubRel {
                    len_type,
                    msgid: u16::from_be_bytes([data[1], data[2]]),
                }
            }
            0x12 => {
                if data.len() < 6 {
                    return Self::None;
                }
                Self::Subscribe {
                    len_type,
                    flags: data[1],
                    msgid: u16::from_be_bytes([data[2], data[3]]),
                    topic: match data[1] & 0x03 {
                        0x00 => &data[(4 + 2)..],
                        0x01 | 0x02 => &data[4..=5],
                        _ => return Self::None,
                    },
                }
            }
            0x13 => {
                if data.len() < 7 {
                    return Self::None;
                }
                Self::SubAck {
                    len_type,
                    flags: data[1],
                    topicid: u16::from_be_bytes([data[2], data[3]]),
                    msgid: u16::from_be_bytes([data[4], data[5]]),
                    return_code: data[6],
                }
            }
            0x14 => {
                if data.len() < 6 {
                    return Self::None;
                }
                Self::Unsubscribe {
                    len_type,
                    flags: data[1],
                    msgid: u16::from_be_bytes([data[2], data[3]]),
                    topic: match data[1] {
                        0x00 => &data[(4 + 2)..],
                        0x01 | 0x02 => &data[4..=5],
                        _ => return Self::None,
                        // 0 => TopicType::FullName(&data[(4 + 2)..]),
                        // 1 => TopicType::Id(u16::from_be_bytes([data[4], data[5]])),
                        // 2 => TopicType::ShortName([data[4], data[5]]),
                        // _ => panic!(),
                    },
                }
            }
            0x15 => {
                if data.len() < 3 {
                    return Self::None;
                }
                Self::UnsubAck {
                    len_type,
                    msgid: u16::from_be_bytes([data[1], data[2]]),
                }
            }
            0x16 => {
                if data.len() < 2 {
                    return Self::None;
                }
                Self::PingReq {
                    len_type,
                    client_id: Some(&data[1..]),
                }
            }
            0x17 => {
                if data.len() < 1 {
                    return Self::None;
                }
                Self::PingResp { len_type }
            }
            0x18 => {
                if data.len() < 3 {
                    return Self::None;
                }
                Self::Disconnect {
                    len_type,
                    duration: u16::from_be_bytes([data[1], data[2]]),
                }
            }
            0x1A => {
                if data.len() < 2 {
                    return Self::None;
                }
                Self::WillTopicUpd {
                    len_type,
                    flags: data[1],
                    will_topic: Some(&data[2..]),
                }
            }
            0x1B => {
                if data.len() < 2 {
                    return Self::None;
                }
                Self::WillTopicResp {
                    len_type,
                    return_code: data[1],
                }
            }
            0x1C => {
                if data.len() < 1 {
                    return Self::None;
                }
                Self::WillMsgUpd {
                    len_type,
                    will_msg: Some(&data[1..]),
                }
            }
            0x1D => {
                if data.len() < 2 {
                    return Self::None;
                }
                Self::WillMsgResp {
                    len_type,
                    return_code: data[1],
                }
            }
            0xFE => {
                //TODO: verify this
                if data.len() < 4 {
                    return Self::None;
                }
                Self::ForwarderCapsule {
                    len_type,
                    wnode_id: Some(&data[3..(3 + data[2] as usize)]),
                    mqttsn_msg: Some(&data[3 + data[2] as usize..]),
                    control: data[1],
                }
            }
            _ => Self::None,
        }
    }
}
