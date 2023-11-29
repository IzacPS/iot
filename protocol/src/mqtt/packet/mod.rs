// use std::fmt::Debug;

use self::{connect::Will, publish::PublishFlags};
use crate::PacketSerializer;

// pub mod binary_data;
pub mod connect;
// pub mod mqtt_string;
pub mod property;
pub mod publish;
// pub mod str_pair;
pub mod subscribe;
// #[macro_use]
// pub mod topic_filter;
pub mod varint;

#[derive(Debug)]
pub enum PacketType<'pt> {
    None,
    Auth {
        reason_code: u8,
        properties: Option<&'pt [u8]>,
    },
    Connack {
        connect_ack_flags: u8,
        reason_code: u8,
        properties: Option<&'pt [u8]>,
    },
    Connect {
        clean_start: bool,
        keep_alive: u16,
        properties: Option<&'pt [u8]>,
        client_id: Option<&'pt [u8]>,
        will: Option<Will<'pt>>,
        username: Option<&'pt [u8]>,
        password: Option<&'pt [u8]>,
    },
    Disconnect {
        reason_code: u8,
        properties: Option<&'pt [u8]>,
    },
    PingReq,
    PingResp,
    PubAck {
        identifier: u16,
        reason_code: u8,
        properties: Option<&'pt [u8]>,
    },
    PubComp {
        identifier: u16,
        reason_code: u8,
        properties: Option<&'pt [u8]>,
    },
    PubRec {
        identifier: u16,
        reason_code: u8,
        properties: Option<&'pt [u8]>,
    },
    PubRel {
        identifier: u16,
        reason_code: u8,
        properties: Option<&'pt [u8]>,
    },
    Publish {
        control_flags: u8,
        topic_name: Option<&'pt [u8]>,
        identifier: u16,
        properties: Option<&'pt [u8]>,
        payload: Option<&'pt [u8]>,
    },
    SubAck {
        identifier: u16,
        properties: Option<&'pt [u8]>,
        reason_codes: &'pt [u8],
    },
    Subscribe {
        identifier: u16,
        properties: Option<&'pt [u8]>,
        topic_filters: Option<&'pt [u8]>,
    },
    UnsubAck {
        identifier: u16,
        properties: Option<&'pt [u8]>,
        reason_codes: &'pt [u8],
    },
    Unsubscribe {
        identifier: u16,
        properties: Option<&'pt [u8]>,
        topic_filters: Option<&'pt [u8]>,
    },
}

// impl std::fmt::Debug for PacketType<'_> {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         match self {
//             Self::None => write!(f, "None"),
//             Self::Auth {
//                 reason_code,
//                 properties,
//             } => {
//                 let a = 0;
//                 f.debug_struct("Auth")
//                     .field("reason_code", reason_code)
//                     .field("properties", properties)
//                     .finish()
//             }
//             Self::Connack {
//                 connect_ack_flags,
//                 reason_code,
//                 properties,
//             } => {
//                 let a = 0;
//                 f.debug_struct("Connack")
//                     .field("connect_ack_flags", connect_ack_flags)
//                     .field("reason_code", reason_code)
//                     .field("properties", properties)
//                     .finish()
//             }
//             Self::Connect {
//                 clean_start,
//                 keep_alive,
//                 properties,
//                 client_id,
//                 will,
//                 username,
//                 password,
//             } => {
//                 let a = 0;
//                 f.debug_struct("Connect")
//                     .field("clean_start", clean_start)
//                     .field("keep_alive", keep_alive)
//                     .field("properties", properties)
//                     .field("client_id", client_id)
//                     .field("will", will)
//                     .field("username", username)
//                     .field("password", password)
//                     .finish()
//             }
//             Self::Disconnect {
//                 reason_code,
//                 properties,
//             } => {
//                 let a = 0;
//                 f.debug_struct("Disconnect")
//                     .field("reason_code", reason_code)
//                     .field("properties", properties)
//                     .finish()
//             }
//             Self::PingReq => write!(f, "PingReq"),
//             Self::PingResp => write!(f, "PingResp"),
//             Self::PubAck {
//                 identifier,
//                 reason_code,
//                 properties,
//             } => {
//                 let a = 0;
//                 f.debug_struct("PubAck")
//                     .field("identifier", identifier)
//                     .field("reason_code", reason_code)
//                     .field("properties", properties)
//                     .finish()
//             }
//             Self::PubComp {
//                 identifier,
//                 reason_code,
//                 properties,
//             } => {
//                 let a = 0;
//                 f.debug_struct("PubComp")
//                     .field("identifier", identifier)
//                     .field("reason_code", reason_code)
//                     .field("properties", properties)
//                     .finish()
//             }
//             Self::PubRec {
//                 identifier,
//                 reason_code,
//                 properties,
//             } => {
//                 let a = 0;
//                 f.debug_struct("PubRec")
//                     .field("identifier", identifier)
//                     .field("reason_code", reason_code)
//                     .field("properties", properties)
//                     .finish()
//             }
//             Self::PubRel {
//                 identifier,
//                 reason_code,
//                 properties,
//             } => {
//                 let a = 0;
//                 f.debug_struct("PubRel")
//                     .field("identifier", identifier)
//                     .field("reason_code", reason_code)
//                     .field("properties", properties)
//                     .finish()
//             }
//             Self::Publish {
//                 control_flags,
//                 topic_name,
//                 identifier,
//                 properties,
//                 payload,
//             } => {
//                 let a = 0;
//                 f.debug_struct("Publish")
//                     .field("control_flags", control_flags)
//                     .field("topic_name", topic_name)
//                     .field("identifier", identifier)
//                     .field("properties", properties)
//                     .field("payload", payload)
//                     .finish()
//             }
//             Self::SubAck {
//                 identifier,
//                 properties,
//                 reason_codes,
//             } => {
//                 let a = 0;
//                 f.debug_struct("SubAck")
//                     .field("identifier", identifier)
//                     .field("properties", properties)
//                     .field("reason_codes", reason_codes)
//                     .finish()
//             }
//             Self::Subscribe {
//                 identifier,
//                 properties,
//                 topic_filters,
//             } => {
//                 let a = 0;
//                 f.debug_struct("Subscribe")
//                     .field("identifier", identifier)
//                     .field("properties", properties)
//                     .field("topic_filters", topic_filters)
//                     .finish()
//             }
//             Self::UnsubAck {
//                 identifier,
//                 properties,
//                 reason_codes,
//             } => {
//                 let a = 0;
//                 f.debug_struct("UnsubAck")
//                     .field("identifier", identifier)
//                     .field("properties", properties)
//                     .field("reason_codes", reason_codes)
//                     .finish()
//             }
//             Self::Unsubscribe {
//                 identifier,
//                 properties,
//                 topic_filters,
//             } => {
//                 let a = 0;
//                 f
//                 .debug_struct("Unsubscribe")
//                 .field("identifier", identifier)
//                 .field("properties", properties)
//                 .field("topic_filters", topic_filters)
//                 .finish(),
//         }
//     }
// }

impl PacketSerializer for PacketType<'_> {
    fn serialize(&self, buf: &mut [u8]) -> usize {
        let mut index: usize = 0;
        match self {
            PacketType::None => index,
            PacketType::Auth {
                reason_code,
                properties,
            } => {
                //fixed header
                buf[index] = 15 << 4;
                index += 1;
                //remaining length
                let mut remaining_len = 1;
                let props_len = if let Some(props) = properties {
                    props.len()
                } else {
                    0
                };
                remaining_len += props_len;

                let mut properties_varlen = [0u8; 4];
                let prop_varlen_len = varint::encode(props_len as u64, &mut properties_varlen);
                remaining_len += prop_varlen_len;
                let mut remaining_varlen = [0u8; 4];
                if remaining_len > buf.len() {
                    return 0;
                }
                let rem_varlen_len = varint::encode(remaining_len as u64, &mut remaining_varlen);
                for idx in 0..rem_varlen_len {
                    buf[index] = remaining_varlen[idx];
                    index += 1;
                }
                // data.extend(remaining_len.as_slice());
                //variable header
                buf[index] = *reason_code;
                index += 1;
                // properties_len.reverse();
                for idx in 0..prop_varlen_len {
                    buf[index] = properties_varlen[idx];
                    index += 1;
                }
                // data.extend(properties_len.as_slice());
                if let Some(props) = properties {
                    for c in *props {
                        buf[index] = *c;
                        index += 1;
                    }
                }
                // data.extend(properties.as_slice());
                // data
                index
            }
            PacketType::Connack {
                connect_ack_flags,
                reason_code,
                properties,
            } => {
                //fixed header
                buf[index] = 2 << 4;
                index += 1;
                //remaining length
                let mut remaining_len = 2;
                // remaining_len += properties.len();
                let props_len = if let Some(props) = properties {
                    props.len()
                } else {
                    0
                };
                remaining_len += props_len;
                let mut properties_varlen = [0u8; 4];
                let props_varlen_len = varint::encode(props_len as u64, &mut properties_varlen);
                remaining_len += props_varlen_len;
                let mut remaining_varlen = [0u8; 4];
                if remaining_len > buf.len() {
                    return 0;
                }
                let rem_varlen_len = varint::encode(remaining_len as u64, &mut remaining_varlen);
                for idx in 0..rem_varlen_len {
                    buf[index] = remaining_varlen[idx];
                    index += 1;
                }
                // data.extend(remaining_len.as_slice());
                //variable header
                buf[index] = *connect_ack_flags;
                index += 1;
                buf[index] = *reason_code;
                index += 1;
                // properties_len.reverse();
                for idx in 0..props_varlen_len {
                    buf[index] = properties_varlen[idx];
                    index += 1;
                }
                // for
                // data.extend(properties_len.as_slice());
                if let Some(props) = properties {
                    for c in *props {
                        buf[index] = *c;
                        index += 1;
                    }
                }
                // data.extend(properties.as_slice());
                index
            }
            PacketType::Connect {
                clean_start,
                keep_alive,
                properties,
                client_id,
                will,
                username,
                password,
            } => {
                //----- Fixed header
                buf[index] = 1 << 4;
                index += 1;
                //----- remaining length
                let mut remaining_length = 10;

                // let properties_len = properties.len();
                let props_len = if let Some(props) = properties {
                    props.len()
                } else {
                    0
                };
                remaining_length += props_len;

                let mut properties_varlen = [0u8; 4];
                let props_varlen_len = varint::encode(props_len as u64, &mut properties_varlen);
                remaining_length += props_varlen_len;

                if let Some(client_id) = client_id {
                    remaining_length += client_id.len() + 2;
                }
                // let mut will_properties_len = Vec::new();

                let mut will_properties_varlen = [0u8; 4];
                let mut will_props_varlen_len = 0;
                if let Some(will) = will {
                    let props_len = if let Some(props) = will.properties {
                        props.len()
                    } else {
                        0
                    };
                    // let properties_len = props_len;
                    remaining_length += props_len;
                    will_props_varlen_len =
                        varint::encode(props_len as u64, &mut will_properties_varlen);
                    remaining_length += will_props_varlen_len;

                    if let Some(will_topic) = will.topic {
                        remaining_length += will_topic.len() + 2;
                    }
                    // remaining_length += will.topic.as_bytes().len();
                    //will payload
                    // remaining_length += will.payload.as_slice().len();
                    remaining_length += 2;
                    if let Some(will_payload) = will.payload {
                        remaining_length += will_payload.len();
                    }
                    //   (will_properties_varlen, will_props_varlen_len)
                }
                //username
                // remaining_length += 2;
                if let Some(username) = username {
                    remaining_length += username.len() + 2;
                }
                //password
                if let Some(password) = password {
                    remaining_length += password.len();
                }

                let mut remaining_varlen = [0u8; 4];
                let rem_varlen_len = varint::encode(remaining_length as u64, &mut remaining_varlen);
                for idx in 0..rem_varlen_len {
                    buf[index] = remaining_varlen[idx];
                    index += 1;
                }
                // // remaining_len.reverse();
                // data.extend(&remaining_len);

                //Variable Header
                //Protocol Name - MQTT
                //TODO: change protocol name to match MQTTString object
                buf[index] = 0x00;
                index += 1;
                buf[index] = 0x04;
                index += 1;
                buf[index] = 0x4D;
                index += 1;
                buf[index] = 0x51;
                index += 1;
                buf[index] = 0x54;
                index += 1;
                buf[index] = 0x54;
                index += 1;

                //Version - 5
                buf[index] = 0x05;
                index += 1;

                //Connect Flags
                let mut flags = 0;
                if *clean_start {
                    flags |= 1 << 1;
                }
                if let Some(will) = will {
                    flags |= 1 << 2;
                    flags |= will.flags;
                }
                if username.is_some() {
                    flags |= 1 << 7;
                }
                if password.is_some() {
                    flags |= 1 << 6;
                }
                buf[index] = flags;
                index += 1;

                //Keep Alive time
                // #[cfg(target_endian = "big")]
                // data.insert_slice(data.len(), &self.keep_alive.to_ne_bytes());
                // #[cfg(target_endian = "little")]
                // data.replace_or_insert_slice(data.len(), &keep_alive.to_be_bytes());
                #[cfg(target_endian = "big")]
                {
                    buf[index] = *keep_alive as u8;
                    index += 1;
                    buf[index] = (*keep_alive >> 8) as u8;
                    index += 1;
                }
                #[cfg(target_endian = "little")]
                {
                    buf[index] = (*keep_alive >> 8) as u8;
                    index += 1;
                    buf[index] = *keep_alive as u8;
                    index += 1;
                }

                //Properties
                for idx in 0..props_varlen_len {
                    buf[index] = properties_varlen[idx];
                    index += 1;
                }
                if let Some(props) = properties {
                    for c in *props {
                        buf[index] = *c;
                        index += 1;
                    }
                }
                // properties_len.reverse();
                // data.extend(&properties_len);
                // data.extend(properties.as_slice());

                //payload
                //ClientId
                if let Some(client_id) = client_id {
                    let len = client_id.len();
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
                    for c in *client_id {
                        buf[index] = *c;
                        index += 1;
                    }
                    // data.extend(client_id.as_bytes());
                } else {
                    // let cid = self::mqtt_string::MQTTString::new();
                    buf[index] = 0;
                    index += 1;
                    buf[index] = 0;
                    index += 1;
                    // data.extend(cid.as_bytes());
                }
                if let Some(will) = will {
                    //Will Properties
                    for idx in 0..will_props_varlen_len {
                        buf[index] = will_properties_varlen[idx];
                        index += 1;
                    }
                    // will_properties_len.reverse();
                    // data.extend(&will_properties_len);
                    if let Some(props) = will.properties {
                        for c in props {
                            buf[index] = *c;
                            index += 1;
                        }
                    }
                    // data.extend(will.properties.as_slice());

                    //Will Topic
                    if let Some(will_topic) = will.topic {
                        let len = will_topic.len();
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
                        for c in will_topic {
                            buf[index] = *c;
                            index += 1;
                        }
                    }
                    // data.extend(will.topic.as_bytes());

                    //will payload
                    // data.extend(will.payload.as_slice());
                    if let Some(will_payload) = will.payload {
                        let len = will_payload.len();
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
                        for c in will_payload {
                            buf[index] = *c;
                            index += 1;
                        }
                    } else {
                        buf[index] = 0;
                        index += 1;
                        buf[index] = 0;
                        index += 1;
                    }
                }
                //username
                if let Some(username) = username {
                    let len = username.len();
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
                    for c in *username {
                        buf[index] = *c;
                        index += 1;
                    }
                    // data.extend(username.as_bytes());
                }
                //password
                if let Some(password) = password {
                    for c in *password {
                        buf[index] = *c;
                        index += 1;
                    }
                    // data.extend(password.as_slice());
                }
                index
            }
            PacketType::Disconnect {
                reason_code,
                properties,
            } => {
                //fixed header
                buf[index] = 14 << 4;
                index += 1;
                //remaining length
                let mut remaining_len = 1;
                let props_len = if let Some(props) = properties {
                    props.len()
                } else {
                    0
                };
                // remaining_len += properties.len();
                remaining_len += props_len;
                let mut properties_varlen = [0u8; 4];
                let props_varlen_len = varint::encode(props_len as u64, &mut properties_varlen);
                remaining_len += props_varlen_len;
                let mut remaining_varlen = [0u8; 4];
                if remaining_len > buf.len() {
                    return 0;
                }
                let rem_varlen_len = varint::encode(remaining_len as u64, &mut remaining_varlen);
                for idx in 0..rem_varlen_len {
                    buf[index] = remaining_varlen[idx];
                    index += 1;
                }
                // data.extend(remaining_len.as_slice());
                //variable header
                buf[index] = *reason_code;
                index += 1;
                // properties_len.reverse();
                for idx in 0..props_varlen_len {
                    buf[index] = properties_varlen[idx];
                    index += 1;
                }
                // data.extend(properties_len.as_slice());
                // data.extend(properties.as_slice());
                if let Some(props) = properties {
                    for c in *props {
                        buf[index] = *c;
                        index += 1;
                    }
                }
                index
            }
            PacketType::PingReq => {
                //fixed header
                buf[index] = 12 << 4;
                index += 1;
                //remaining length
                buf[index] = 0;
                index += 1;
                index
            }
            PacketType::PingResp => {
                //fixed header
                buf[index] = 13 << 4;
                index += 1;
                //remaining length
                buf[index] = 0;
                index += 1;
                index
            }
            PacketType::PubAck {
                identifier,
                reason_code,
                properties,
            } => {
                //fixed header
                buf[index] = 4 << 4;
                index += 1;
                //remaining length
                let mut remaining_len = 3;
                let props_len = if let Some(props) = properties {
                    props.len()
                } else {
                    0
                };
                // remaining_len += properties.len();
                remaining_len += props_len;
                let mut properties_varlen = [0u8; 4];
                let props_varlen_len = varint::encode(props_len as u64, &mut properties_varlen);
                remaining_len += props_varlen_len;
                let mut remaining_varlen = [0u8; 4];
                if remaining_len > buf.len() {
                    return 0;
                }
                let rem_varlen_len = varint::encode(remaining_len as u64, &mut remaining_varlen);
                for idx in 0..rem_varlen_len {
                    buf[index] = remaining_varlen[idx];
                    index += 1;
                }
                // data.extend(remaining_len.as_slice());
                //variable header
                #[cfg(target_endian = "big")]
                {
                    buf[index] = *identifier as u8;
                    index += 1;
                    buf[index] = (*identifier >> 8) as u8;
                    index += 1;
                }
                #[cfg(target_endian = "little")]
                {
                    buf[index] = (*identifier >> 8) as u8;
                    index += 1;
                    buf[index] = *identifier as u8;
                    index += 1;
                }
                // #[cfg(target_endian = "big")]
                // data.insert_slice(data.len(), &self.identifier.to_ne_bytes());
                // #[cfg(target_endian = "little")]
                // data.replace_or_insert_slice(data.len(), &identifier.to_be_bytes());
                buf[index] = *reason_code;
                index += 1;
                // properties_len.reverse();
                for idx in 0..props_varlen_len {
                    buf[index] = properties_varlen[idx];
                    index += 1;
                }
                // data.extend(properties_len.as_slice());
                if let Some(props) = properties {
                    for c in *props {
                        buf[index] = *c;
                        index += 1;
                    }
                }
                // data.extend(properties.as_slice());
                index
            }
            PacketType::PubComp {
                identifier,
                reason_code,
                properties,
            } => {
                //fixed header
                buf[index] = 7 << 4;
                index += 1;
                //remaining length
                let mut remaining_len = 3;
                let props_len = if let Some(props) = properties {
                    props.len()
                } else {
                    0
                };
                // remaining_len += properties.len();
                remaining_len += props_len;
                let mut properties_varlen = [0u8; 4];
                let props_varlen_len = varint::encode(props_len as u64, &mut properties_varlen);
                remaining_len += props_varlen_len;
                let mut remaining_varlen = [0u8; 4];
                if remaining_len > buf.len() {
                    return 0;
                }
                let rem_varlen_len = varint::encode(remaining_len as u64, &mut remaining_varlen);
                for idx in 0..rem_varlen_len {
                    buf[index] = remaining_varlen[idx];
                    index += 1;
                }
                // data.extend(remaining_len.as_slice());
                //variable header
                #[cfg(target_endian = "big")]
                {
                    buf[index] = *identifier as u8;
                    index += 1;
                    buf[index] = (*identifier >> 8) as u8;
                    index += 1;
                }
                #[cfg(target_endian = "little")]
                {
                    buf[index] = (*identifier >> 8) as u8;
                    index += 1;
                    buf[index] = *identifier as u8;
                    index += 1;
                }
                // #[cfg(target_endian = "big")]
                // data.replace_or_insert_slice(data.len(), &self.identifier.to_ne_bytes());
                // #[cfg(target_endian = "little")]
                // data.replace_or_insert_slice(data.len(), &identifier.to_be_bytes());
                buf[index] = *reason_code;
                index += 1;
                for idx in 0..props_varlen_len {
                    buf[index] = properties_varlen[idx];
                    index += 1;
                }
                // properties_len.reverse();
                // data.extend(properties_len.as_slice());
                if let Some(props) = properties {
                    for c in *props {
                        buf[index] = *c;
                        index += 1;
                    }
                }
                // data.extend(properties.as_slice());
                index
            }
            PacketType::PubRec {
                identifier,
                reason_code,
                properties,
            } => {
                //fixed header
                buf[index] = 5 << 4;
                index += 1;
                //remaining length
                let mut remaining_len = 3;
                let props_len = if let Some(props) = properties {
                    props.len()
                } else {
                    0
                };
                remaining_len += props_len;
                let mut properties_varlen = [0u8; 4];
                let props_varlen_len = varint::encode(props_len as u64, &mut properties_varlen);
                remaining_len += props_varlen_len;
                let mut remaining_varlen = [0u8; 4];
                if remaining_len > buf.len() {
                    return 0;
                }
                let rem_varlen_len = varint::encode(remaining_len as u64, &mut remaining_varlen);
                // data.extend(remaining_len.as_slice());
                for idx in 0..rem_varlen_len {
                    buf[index] = remaining_varlen[idx];
                    index += 1;
                }
                //variable header
                #[cfg(target_endian = "big")]
                {
                    buf[index] = *identifier as u8;
                    index += 1;
                    buf[index] = (*identifier >> 8) as u8;
                    index += 1;
                }
                #[cfg(target_endian = "little")]
                {
                    buf[index] = (*identifier >> 8) as u8;
                    index += 1;
                    buf[index] = *identifier as u8;
                    index += 1;
                }
                // #[cfg(target_endian = "big")]
                // data.insert_slice(data.len(), &self.identifier.to_ne_bytes());
                // #[cfg(target_endian = "little")]
                // data.replace_or_insert_slice(data.len(), &identifier.to_be_bytes());
                buf[index] = *reason_code;
                index += 1;
                // properties_len.reverse();
                for idx in 0..props_varlen_len {
                    buf[index] = properties_varlen[idx];
                    index += 1;
                }
                // data.extend(properties_len.as_slice());
                if let Some(props) = properties {
                    for c in *props {
                        buf[index] = *c;
                        index += 1;
                    }
                }
                // data.extend(properties.as_slice());
                index
            }
            PacketType::PubRel {
                identifier,
                reason_code,
                properties,
            } => {
                //fixed header
                buf[index] = 6 << 4 | 1 << 1;
                index += 1;
                //remaining length
                let mut remaining_len = 3;
                let props_len = if let Some(props) = properties {
                    props.len()
                } else {
                    0
                };
                remaining_len += props_len;
                let mut properties_varlen = [0u8; 4];
                let props_varlen_len = varint::encode(props_len as u64, &mut properties_varlen);
                remaining_len += props_varlen_len;
                let mut remaining_varlen = [0u8; 4];
                if remaining_len > buf.len() {
                    return 0;
                }
                let rem_varlen_len = varint::encode(remaining_len as u64, &mut remaining_varlen);
                for idx in 0..rem_varlen_len {
                    buf[index] = remaining_varlen[idx];
                    index += 1;
                }
                // data.extend(remaining_len.as_slice());
                //variable header
                #[cfg(target_endian = "big")]
                {
                    buf[index] = *identifier as u8;
                    index += 1;
                    buf[index] = (*identifier >> 8) as u8;
                    index += 1;
                }
                #[cfg(target_endian = "little")]
                {
                    buf[index] = (*identifier >> 8) as u8;
                    index += 1;
                    buf[index] = *identifier as u8;
                    index += 1;
                }
                // #[cfg(target_endian = "big")]
                // data.insert_slice(data.len(), &self.identifier.to_ne_bytes());
                // #[cfg(target_endian = "little")]
                // data.replace_or_insert_slice(data.len(), &identifier.to_be_bytes());
                buf[index] = *reason_code;
                index += 1;
                // properties_len.reverse();
                for idx in 0..props_varlen_len {
                    buf[index] = properties_varlen[idx];
                    index += 1;
                }
                // data.extend(properties_len.as_slice());
                if let Some(props) = properties {
                    for c in *props {
                        buf[index] = *c;
                        index += 1;
                    }
                }
                // data.extend(properties.as_slice());
                index
            }
            PacketType::Publish {
                control_flags,
                topic_name,
                identifier,
                properties,
                payload,
            } => {
                //fixed header
                let header = 3 << 4 | *control_flags;
                println!("PUBLISH FLAG: {header:#08b}");
                buf[index] = header;
                index += 1;
                //remaining length
                let mut remaining_len = 0;
                if let Some(tname) = topic_name {
                    remaining_len += tname.len() + 2;
                }
                // let properties_len = properties.len();
                let props_len = if let Some(props) = properties {
                    props.len()
                } else {
                    0
                };
                remaining_len += props_len;
                let mut properties_varlen = [0u8; 4];
                let props_varlen_len = varint::encode(props_len as u64, &mut properties_varlen);
                remaining_len += props_varlen_len;
                if let Some(p) = payload {
                    remaining_len += p.len();
                }
                if (*control_flags & (PublishFlags::QoS1.value() | PublishFlags::QoS2.value())) != 0
                {
                    remaining_len += 2;
                }
                let mut remaining_varlen = [0u8; 4];
                if remaining_len > buf.len() {
                    return 0;
                }
                let rem_varlen_len = varint::encode(remaining_len as u64, &mut remaining_varlen);
                for idx in 0..rem_varlen_len {
                    buf[index] = remaining_varlen[idx];
                    index += 1;
                }
                // data.extend(remaining_len.as_slice());
                //variable header
                if let Some(tname) = topic_name {
                    let len = tname.len();
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
                    for c in *tname {
                        buf[index] = *c;
                        index += 1;
                    }
                }
                // data.extend(topic_name.as_bytes());
                if (*control_flags & (PublishFlags::QoS1.value() | PublishFlags::QoS2.value())) != 0
                {
                    #[cfg(target_endian = "big")]
                    {
                        buf[index] = *identifier as u8;
                        index += 1;
                        buf[index] = (*identifier >> 8) as u8;
                        index += 1;
                    }
                    #[cfg(target_endian = "little")]
                    {
                        buf[index] = (*identifier >> 8) as u8;
                        index += 1;
                        buf[index] = *identifier as u8;
                        index += 1;
                    }
                    // #[cfg(target_endian = "big")]
                    // data.insert_slice(data.len(), &self.identifier.to_ne_bytes());
                    // #[cfg(target_endian = "little")]
                    // data.replace_or_insert_slice(data.len(), &identifier.to_be_bytes());
                }
                for idx in 0..props_varlen_len {
                    buf[index] = properties_varlen[idx];
                    index += 1;
                }
                // properties_len.reverse();
                // data.extend(properties_len.as_slice());
                if let Some(props) = properties {
                    for c in *props {
                        buf[index] = *c;
                        index += 1;
                    }
                }
                // data.extend(properties.as_slice());
                if let Some(p) = payload {
                    for c in *p {
                        buf[index] = *c;
                        index += 1;
                    }
                    // data.extend(p.as_slice());
                }
                //payload
                index
            }
            PacketType::SubAck {
                identifier,
                properties,
                reason_codes,
            } => {
                //fixed header
                buf[index] = 9 << 4;
                index += 1;
                //remaining length
                let mut remaining_len = 2;
                let props_len = if let Some(props) = properties {
                    props.len()
                } else {
                    0
                };
                // remaining_len += properties.len();
                remaining_len += props_len;
                let mut properties_varlen = [0u8; 4];
                let props_varlen_len = varint::encode(props_len as u64, &mut properties_varlen);
                remaining_len += props_varlen_len;
                remaining_len += reason_codes.len();
                let mut remaining_varlen = [0u8; 4];
                if remaining_len > buf.len() {
                    return 0;
                }
                let rem_varlen_len = varint::encode(remaining_len as u64, &mut remaining_varlen);
                for idx in 0..rem_varlen_len {
                    buf[index] = remaining_varlen[idx];
                    index += 1;
                }
                // data.extend(remaining_len.as_slice());
                //variable header
                #[cfg(target_endian = "big")]
                {
                    buf[index] = *identifier as u8;
                    index += 1;
                    buf[index] = (*identifier >> 8) as u8;
                    index += 1;
                }
                #[cfg(target_endian = "little")]
                {
                    buf[index] = (*identifier >> 8) as u8;
                    index += 1;
                    buf[index] = *identifier as u8;
                    index += 1;
                }
                // #[cfg(target_endian = "big")]
                // data.replace_or_insert_slice(data.len(), &self.identifier.to_ne_bytes());
                // #[cfg(target_endian = "little")]
                // data.replace_or_insert_slice(data.len(), &identifier.to_be_bytes());
                for idx in 0..props_varlen_len {
                    buf[index] = properties_varlen[idx];
                    index += 1;
                }
                // properties_len.reverse();
                // data.extend(properties_len.as_slice());
                if let Some(props) = properties {
                    for c in *props {
                        buf[index] = *c;
                        index += 1;
                    }
                }
                // data.extend(properties.as_slice());
                for c in *reason_codes {
                    buf[index] = *c;
                    index += 1;
                }
                // data.extend(reason_codes.as_slice());
                index
            }
            PacketType::Subscribe {
                identifier,
                properties,
                topic_filters,
            } => {
                //fixed header
                buf[index] = 8 << 4 | 1 << 1;
                index += 1;
                //remaining length
                let mut remaining_len = 2;
                let props_len = if let Some(props) = properties {
                    props.len()
                } else {
                    0
                };
                remaining_len += props_len;
                let mut properties_varlen = [0u8; 4];
                let props_varlen_len = varint::encode(props_len as u64, &mut properties_varlen);
                remaining_len += props_varlen_len;
                // remaining_len += topic_filters.as_slice().len();
                if let Some(tfilters) = topic_filters {
                    remaining_len += tfilters.len();
                }
                let mut remaining_varlen = [0u8; 4];
                if remaining_len > buf.len() {
                    return 0;
                }
                let rem_varlen_len = varint::encode(remaining_len as u64, &mut remaining_varlen);
                for idx in 0..rem_varlen_len {
                    buf[index] = remaining_varlen[idx];
                    index += 1;
                }
                // data.extend(remaining_len.as_slice());
                //variable header
                #[cfg(target_endian = "big")]
                {
                    buf[index] = *identifier as u8;
                    index += 1;
                    buf[index] = (*identifier >> 8) as u8;
                    index += 1;
                }
                #[cfg(target_endian = "little")]
                {
                    buf[index] = (*identifier >> 8) as u8;
                    index += 1;
                    buf[index] = *identifier as u8;
                    index += 1;
                }
                // #[cfg(target_endian = "big")]
                // data.replace_or_insert_slice(data.len(), &self.identifier.to_ne_bytes());
                // #[cfg(target_endian = "little")]
                // data.replace_or_insert_slice(data.len(), &identifier.to_be_bytes());
                for idx in 0..props_varlen_len {
                    buf[index] = properties_varlen[idx];
                    index += 1;
                }
                // properties_len.reverse();
                // data.extend(properties_len.as_slice());
                if let Some(props) = properties {
                    for c in *props {
                        buf[index] = *c;
                        index += 1;
                    }
                }
                // data.extend(properties.as_slice());
                if let Some(tfilters) = topic_filters {
                    for c in *tfilters {
                        buf[index] = *c;
                        index += 1;
                    }
                }
                // data.extend(topic_filters.as_slice());
                index
            }
            PacketType::UnsubAck {
                identifier,
                properties,
                reason_codes,
            } => {
                //fixed header
                buf[index] = 11 << 4;
                index += 1;
                //remaining length
                let mut remaining_len = 2;
                let props_len = if let Some(props) = properties {
                    props.len()
                } else {
                    0
                };
                remaining_len += props_len;
                let mut properties_varlen = [0u8; 4];
                let props_varlen_len = varint::encode(props_len as u64, &mut properties_varlen);
                remaining_len += props_varlen_len;
                remaining_len += reason_codes.len();
                let mut remaining_varlen = [0u8; 4];
                if remaining_len > buf.len() {
                    return 0;
                }
                let rem_varlen_len = varint::encode(remaining_len as u64, &mut remaining_varlen);
                for idx in 0..rem_varlen_len {
                    buf[index] = remaining_varlen[idx];
                    index += 1;
                }
                // data.extend(remaining_len.as_slice());
                //variable header
                #[cfg(target_endian = "big")]
                {
                    buf[index] = *identifier as u8;
                    index += 1;
                    buf[index] = (*identifier >> 8) as u8;
                    index += 1;
                }
                #[cfg(target_endian = "little")]
                {
                    buf[index] = (*identifier >> 8) as u8;
                    index += 1;
                    buf[index] = *identifier as u8;
                    index += 1;
                }
                // #[cfg(target_endian = "big")]
                // data.place_or_insert_slice(data.len(), &self.identifier.to_ne_bytes());
                // #[cfg(target_endian = "little")]
                // data.replace_or_insert_slice(data.len(), &identifier.to_be_bytes());
                for idx in 0..props_varlen_len {
                    buf[index] = properties_varlen[idx];
                    index += 1;
                }
                // properties_len.reverse();
                // data.extend(properties_len.as_slice());
                // data.extend(properties.as_slice());
                if let Some(props) = properties {
                    for c in *props {
                        buf[index] = *c;
                        index += 1;
                    }
                }
                for c in *reason_codes {
                    buf[index] = *c;
                    index += 1;
                }
                // data.extend(reason_codes.as_slice());
                index
            }
            PacketType::Unsubscribe {
                identifier,
                properties,
                topic_filters,
            } => {
                //fixed header
                buf[index] = 10 << 4 | 1 << 1;
                index += 1;
                //remaining length
                let mut remaining_len = 2;
                let props_len = if let Some(props) = properties {
                    props.len()
                } else {
                    0
                };
                remaining_len += props_len;
                let mut properties_varlen = [0u8; 4];
                let props_varlen_len = varint::encode(props_len as u64, &mut properties_varlen);
                remaining_len += props_varlen_len;
                if let Some(tfilters) = topic_filters {
                    remaining_len += tfilters.len();
                }
                // remaining_len += topic_filters.as_slice().len();
                let mut remaining_varlen = [0u8; 4];
                if remaining_len > buf.len() {
                    return 0;
                }
                let rem_varlen_len = varint::encode(remaining_len as u64, &mut remaining_varlen);
                for idx in 0..rem_varlen_len {
                    buf[index] = remaining_varlen[idx];
                    index += 1;
                }
                // data.extend(remaining_len.as_slice());
                //variable header
                #[cfg(target_endian = "big")]
                {
                    buf[index] = *identifier as u8;
                    index += 1;
                    buf[index] = (*identifier >> 8) as u8;
                    index += 1;
                }
                #[cfg(target_endian = "little")]
                {
                    buf[index] = (*identifier >> 8) as u8;
                    index += 1;
                    buf[index] = *identifier as u8;
                    index += 1;
                }
                // #[cfg(target_endian = "big")]
                // data.replace_or_insert_slice(data.len(), &self.identifier.to_ne_bytes());
                // #[cfg(target_endian = "little")]
                // data.replace_or_insert_slice(data.len(), &identifier.to_be_bytes());
                for idx in 0..props_varlen_len {
                    buf[index] = properties_varlen[idx];
                    index += 1;
                }
                // properties_len.reverse();
                // data.extend(properties_len.as_slice());
                if let Some(props) = properties {
                    for c in *props {
                        buf[index] = *c;
                        index += 1;
                    }
                }
                // data.extend(properties.as_slice());
                if let Some(tfilters) = topic_filters {
                    for c in *tfilters {
                        buf[index] = *c;
                        index += 1;
                    }
                }
                // data.extend(topic_filters.as_slice());
                // data
                index
            }
        }
    }
}

impl<'pt> PacketType<'pt> {
    pub fn deserialize(arr: &'pt [u8]) -> Self {
        if arr.len() < 2 {
            return Self::None;
        }

        match arr[0] & 0xF0 {
            240 => {
                if arr.len() < 3 {
                    return Self::None;
                }
                let (_remain_len, remain_len_size) =
                    if let Some((_remain_len, remain_len_size)) = varint::decode(&arr[1..], 4) {
                        (_remain_len, remain_len_size)
                    } else {
                        return Self::None;
                    };
                if _remain_len > arr.len() as u64 {
                    return Self::None;
                }
                let data = &arr[1 + remain_len_size as usize..];
                let reason_code_start = 0;
                let reason_code_len = 1;
                let reason_code = data[reason_code_start];

                let properties_start = reason_code_start + reason_code_len;
                // let (_, properties_offset) = varint::decode(&data[properties_start..]);
                let properties_offset = if let Some((_, properties_offset)) =
                    varint::decode(&data[properties_start..], 4)
                {
                    properties_offset
                } else {
                    return Self::None;
                };
                let properties_start = properties_start + properties_offset as usize;

                PacketType::Auth {
                    reason_code,
                    properties: if properties_start < data.len() - 1 {
                        Some(&data[properties_start..])
                    } else {
                        None
                    },
                }
            }
            32 => {
                if arr.len() < 3 {
                    return Self::None;
                }
                let (_remain_len, remain_len_size) =
                    if let Some((_remain_len, remain_len_size)) = varint::decode(&arr[1..], 4) {
                        (_remain_len, remain_len_size)
                    } else {
                        return Self::None;
                    };
                if _remain_len > arr.len() as u64 {
                    return Self::None;
                }
                let data = &arr[1 + remain_len_size as usize..];
                let ack_flags_start = 0;
                let ack_flags_len = 1;
                let flags = data[ack_flags_start];

                let reason_code_start = ack_flags_start + ack_flags_len;
                let reason_code_len = 1;
                let reason_code = data[reason_code_start];

                let properties_start = reason_code_start + reason_code_len;
                let (properties_len, properties_offset) =
                    if let Some((properties_len, properties_offset)) =
                        varint::decode(&data[properties_start..], 4)
                    {
                        (properties_len, properties_offset)
                    } else {
                        return Self::None;
                    };
                let properties_start = properties_start + properties_offset as usize;

                PacketType::Connack {
                    connect_ack_flags: flags,
                    reason_code,
                    properties: if properties_len > 0 {
                        Some(&data[properties_start..(properties_start + properties_len as usize)])
                    } else {
                        None
                    },
                }
            }
            16 => {
                if arr.len() < 3 {
                    return Self::None;
                }
                let (_remain_len, remain_len_size) =
                    if let Some((_remain_len, remain_len_size)) = varint::decode(&arr[1..], 4) {
                        (_remain_len, remain_len_size)
                    } else {
                        return Self::None;
                    };
                if _remain_len > arr.len() as u64 {
                    return Self::None;
                }
                let data = &arr[1 + remain_len_size as usize..];
                let flags_start = 7;
                let flags_len = 1;
                let _flags = data[flags_start];

                let keep_alive_start = flags_start + flags_len;
                let keep_alive_len = 2;
                let keep_alive =
                    u16::from_be_bytes([data[keep_alive_start], data[keep_alive_start + 1]]);

                let properties_start = keep_alive_start + keep_alive_len;
                let (properties_len, properties_offset) =
                    if let Some((properties_len, properties_offset)) =
                        varint::decode(&data[properties_start..], 4)
                    {
                        (properties_len, properties_offset)
                    } else {
                        return Self::None;
                    };
                let properties_start = properties_start + properties_offset as usize;

                let payload_start = properties_start + properties_len as usize;

                let client_id_start = payload_start;
                let client_id_len =
                    u16::from_be_bytes([data[client_id_start], data[client_id_start + 1]]) + 2;
                let client_id = if client_id_len > 2 {
                    Some(&data[(client_id_start + 2)..(client_id_start + client_id_len as usize)])
                } else {
                    None
                };

                let (will, will_offset) = if _flags & 0x04 != 0 {
                    let will_properties_start = client_id_start + client_id_len as usize;
                    let (will_properties_len, will_properties_offset) =
                        if let Some((will_properties_len, will_properties_offset)) =
                            varint::decode(&data[will_properties_start..], 4)
                        {
                            (will_properties_len, will_properties_offset)
                        } else {
                            return Self::None;
                        };
                    let will_properties_start =
                        will_properties_start + will_properties_offset as usize;

                    let will_topic_start = will_properties_start + will_properties_len as usize;
                    let will_topic_len =
                        u16::from_be_bytes([data[will_topic_start], data[will_topic_start + 1]])
                            + 2;

                    let will_payload_start = will_topic_start + will_topic_len as usize;
                    let will_payload_len = u16::from_be_bytes([
                        data[will_payload_start],
                        data[will_payload_start + 1],
                    ]) + 2;

                    (
                        Some(Will {
                            flags: _flags & 0x38,
                            properties: if will_properties_len > 0 {
                                Some(
                                    &data[will_properties_start
                                        ..(will_properties_start + will_properties_len as usize)],
                                )
                            } else {
                                None
                            },
                            topic: if will_topic_len > 0 {
                                Some(
                                    &data[(will_topic_start + 2)
                                        ..(will_topic_start + will_topic_len as usize)],
                                )
                            } else {
                                None
                            },
                            payload: if will_payload_len > 0 {
                                Some(
                                    &data[(will_payload_start + 2)
                                        ..(will_payload_start + will_payload_len as usize)],
                                )
                            } else {
                                None
                            },
                        }),
                        will_payload_start + will_payload_len as usize,
                    )
                } else {
                    (None, client_id_start + client_id_len as usize)
                };

                let username_start = will_offset;

                let (username, username_offset) = if username_start < data.len() {
                    let username_len =
                        u16::from_be_bytes([data[username_start], data[username_start + 1]]) + 2;
                    (
                        Some(&data[(username_start + 2)..(username_start + username_len as usize)]),
                        username_start + username_len as usize,
                    )
                } else {
                    (None, username_start)
                };

                let password_start = username_offset;
                let password = if password_start < data.len() {
                    let password_len =
                        u16::from_be_bytes([data[password_start], data[password_start + 1]]) + 2;
                    Some(&data[password_start..(password_start + password_len as usize)])
                } else {
                    None
                };

                PacketType::Connect {
                    clean_start: (_flags & 1 << 1) != 0,
                    keep_alive,
                    properties: if properties_len > 0 {
                        Some(&data[properties_start..(properties_start + properties_len as usize)])
                    } else {
                        None
                    },
                    client_id,
                    will,
                    username,
                    password,
                }
            }
            224 => {
                if arr.len() < 3 {
                    return Self::None;
                }
                let (_remain_len, remain_len_size) =
                    if let Some((_remain_len, remain_len_size)) = varint::decode(&arr[1..], 4) {
                        (_remain_len, remain_len_size)
                    } else {
                        return Self::None;
                    };
                if _remain_len > arr.len() as u64 {
                    return Self::None;
                }
                let data = &arr[1 + remain_len_size as usize..];
                let reason_code_start = 0;
                let reason_code_len = 1;
                let reason_code = data[reason_code_start];

                let properties_start = reason_code_start + reason_code_len;
                let properties_offset = if let Some((_, properties_offset)) =
                    varint::decode(&data[properties_start..], 4)
                {
                    properties_offset
                } else {
                    return Self::None;
                };
                let properties_start = properties_start + properties_offset as usize;

                PacketType::Disconnect {
                    reason_code,
                    properties: if properties_start < data.len() - 1 {
                        Some(&data[properties_start..])
                    } else {
                        None
                    },
                }
            }
            192 => {
                if arr.len() < 2 {
                    return Self::None;
                }
                PacketType::PingReq
            }
            208 => {
                if arr.len() < 2 {
                    return Self::None;
                }
                PacketType::PingResp
            }
            64 => {
                if arr.len() < 3 {
                    return Self::None;
                }
                let (_remain_len, remain_len_size) =
                    if let Some((_remain_len, remain_len_size)) = varint::decode(&arr[1..], 4) {
                        (_remain_len, remain_len_size)
                    } else {
                        return Self::None;
                    };
                if _remain_len > arr.len() as u64 {
                    return Self::None;
                }
                println!("{} {}", _remain_len, remain_len_size);
                let data = &arr[1 + remain_len_size as usize..];
                println!("data: {:?}", data);
                let id_start = 0;
                let id_len = 2;
                let identifier = u16::from_be_bytes([data[id_start], data[id_start + 1]]);
                let (reason_code, properties) = if _remain_len > 2 {
                    let reason_code_start = id_start + id_len;
                    let reason_code_len = 1;
                    let reason_code = data[reason_code_start];

                    let properties_start = reason_code_start + reason_code_len;
                    let properties_offset = if let Some((_, properties_offset)) =
                        varint::decode(&data[properties_start..], 4)
                    {
                        properties_offset
                    } else {
                        return Self::None;
                    };
                    let properties_start = properties_start + properties_offset as usize;
                    (
                        reason_code,
                        if properties_start < data.len() - 1 {
                            Some(&data[properties_start..])
                        } else {
                            None
                        },
                    )
                } else {
                    (0, None)
                };
                PacketType::PubAck {
                    identifier,
                    reason_code,
                    properties,
                }
            }
            112 => {
                if arr.len() < 3 {
                    return Self::None;
                }
                let (_remain_len, remain_len_size) =
                    if let Some((_remain_len, remain_len_size)) = varint::decode(&arr[1..], 4) {
                        (_remain_len, remain_len_size)
                    } else {
                        return Self::None;
                    };
                if _remain_len > arr.len() as u64 {
                    return Self::None;
                }
                let data = &arr[1 + remain_len_size as usize..];
                let id_start = 0;
                let id_len = 2;
                let identifier = u16::from_be_bytes([data[id_start], data[id_start + 1]]);

                let (reason_code, properties) = if _remain_len > 2 {
                    let reason_code_start = id_start + id_len;
                    let reason_code_len = 1;
                    let reason_code = data[reason_code_start];

                    let properties_start = reason_code_start + reason_code_len;
                    let properties_offset = if let Some((_, properties_offset)) =
                        varint::decode(&data[properties_start..], 4)
                    {
                        properties_offset
                    } else {
                        return Self::None;
                    };
                    let properties_start = properties_start + properties_offset as usize;
                    (
                        reason_code,
                        if properties_start < data.len() - 1 {
                            Some(&data[properties_start..])
                        } else {
                            None
                        },
                    )
                } else {
                    (0, None)
                };
                // let reason_code_start = id_start + id_len;
                // let reason_code_len = 1;
                // let reason_code = data[reason_code_start];

                // let properties_start = reason_code_start + reason_code_len;
                // let properties_offset = if let Some((_, properties_offset)) =
                //     varint::decode(&data[properties_start..], 4)
                // {
                //     properties_offset
                // } else {
                //     return Self::None;
                // };
                // let properties_start = properties_start + properties_offset as usize;

                PacketType::PubComp {
                    identifier,
                    reason_code,
                    properties,
                }
            }
            80 => {
                if arr.len() < 3 {
                    return Self::None;
                }
                let (_remain_len, remain_len_size) =
                    if let Some((_remain_len, remain_len_size)) = varint::decode(&arr[1..], 4) {
                        (_remain_len, remain_len_size)
                    } else {
                        return Self::None;
                    };
                if _remain_len > arr.len() as u64 {
                    return Self::None;
                }
                let data = &arr[1 + remain_len_size as usize..];
                let id_start = 0;
                let id_len = 2;
                let identifier = u16::from_be_bytes([data[id_start], data[id_start + 1]]);

                let (reason_code, properties) = if _remain_len > 2 {
                    let reason_code_start = id_start + id_len;
                    let reason_code_len = 1;
                    let reason_code = data[reason_code_start];

                    let properties_start = reason_code_start + reason_code_len;
                    let properties_offset = if let Some((_, properties_offset)) =
                        varint::decode(&data[properties_start..], 4)
                    {
                        properties_offset
                    } else {
                        return Self::None;
                    };
                    let properties_start = properties_start + properties_offset as usize;
                    (
                        reason_code,
                        if properties_start < data.len() - 1 {
                            Some(&data[properties_start..])
                        } else {
                            None
                        },
                    )
                } else {
                    (0, None)
                };
                // let reason_code_start = id_start + id_len;
                // let reason_code_len = 1;
                // let reason_code = data[reason_code_start];

                // let properties_start = reason_code_start + reason_code_len;
                // let properties_offset = if let Some((_, properties_offset)) =
                //     varint::decode(&data[properties_start..], 4)
                // {
                //     properties_offset
                // } else {
                //     return Self::None;
                // };
                // let properties_start = properties_start + properties_offset as usize;

                PacketType::PubRec {
                    identifier,
                    reason_code,
                    properties,
                }
            }
            96 => {
                if arr.len() < 3 {
                    return Self::None;
                }
                let (_remain_len, remain_len_size) =
                    if let Some((_remain_len, remain_len_size)) = varint::decode(&arr[1..], 4) {
                        (_remain_len, remain_len_size)
                    } else {
                        return Self::None;
                    };
                if _remain_len > arr.len() as u64 {
                    return Self::None;
                }
                let data = &arr[1 + remain_len_size as usize..];
                let id_start = 0;
                let id_len = 2;
                let identifier = u16::from_be_bytes([data[id_start], data[id_start + 1]]);

                let (reason_code, properties) = if _remain_len > 2 {
                    let reason_code_start = id_start + id_len;
                    let reason_code_len = 1;
                    let reason_code = data[reason_code_start];

                    let properties_start = reason_code_start + reason_code_len;
                    let properties_offset = if let Some((_, properties_offset)) =
                        varint::decode(&data[properties_start..], 4)
                    {
                        properties_offset
                    } else {
                        return Self::None;
                    };
                    let properties_start = properties_start + properties_offset as usize;
                    (
                        reason_code,
                        if properties_start < data.len() - 1 {
                            Some(&data[properties_start..])
                        } else {
                            None
                        },
                    )
                } else {
                    (0, None)
                };
                // let reason_code_start = id_start + id_len;
                // let reason_code_len = 1;
                // let reason_code = data[reason_code_start];

                // let properties_start = reason_code_start + reason_code_len;
                // let properties_offset = if let Some((_, properties_offset)) =
                //     varint::decode(&data[properties_start..], 4)
                // {
                //     properties_offset
                // } else {
                //     return Self::None;
                // };
                // let properties_start = properties_start + properties_offset as usize;

                PacketType::PubRel {
                    identifier,
                    reason_code,
                    properties,
                }
            }
            48 => {
                if arr.len() < 3 {
                    return Self::None;
                }
                let (_remain_len, remain_len_size) =
                    if let Some((_remain_len, remain_len_size)) = varint::decode(&arr[1..], 4) {
                        (_remain_len, remain_len_size)
                    } else {
                        return Self::None;
                    };
                if _remain_len > arr.len() as u64 {
                    return Self::None;
                }
                let data = &arr[1 + remain_len_size as usize..];
                let topic_name_start = 0;
                let topic_name_len =
                    u16::from_be_bytes([data[topic_name_start], data[topic_name_start + 1]]) + 2;

                // let topic_name =
                let id_start = topic_name_start + topic_name_len as usize;
                let mut id_len = 0;
                let mut identifier = 0;
                if arr[0] & 0x06 != 0 {
                    identifier = u16::from_be_bytes([data[id_start], data[id_start + 1]]);
                    id_len = 0;
                }

                let properties_start = id_start + id_len;
                let (properties_len, properties_offset) =
                    if let Some((properties_len, properties_offset)) =
                        varint::decode(&data[properties_start..], 4)
                    {
                        (properties_len, properties_offset)
                    } else {
                        return Self::None;
                    };
                let properties_start = properties_start + properties_offset as usize;

                let payload_start = properties_start + properties_len as usize;
                // let payload =
                PacketType::Publish {
                    control_flags: arr[0] & 0x0F,
                    topic_name: if topic_name_len > 0 {
                        Some(
                            &data[(topic_name_start + 2)
                                ..(topic_name_start + topic_name_len as usize)],
                        )
                    } else {
                        None
                    },
                    identifier,
                    properties: if properties_len > 0 {
                        Some(&data[properties_start..(properties_start + properties_len as usize)])
                    } else {
                        None
                    },
                    payload: if payload_start < data.len() - 1 {
                        Some(&data[payload_start..])
                    } else {
                        None
                    },
                }
            }
            144 => {
                if arr.len() < 3 {
                    return Self::None;
                }
                let (_remain_len, remain_len_size) =
                    if let Some((_remain_len, remain_len_size)) = varint::decode(&arr[1..], 4) {
                        (_remain_len, remain_len_size)
                    } else {
                        return Self::None;
                    };
                if _remain_len > arr.len() as u64 {
                    return Self::None;
                }
                let data = &arr[1 + remain_len_size as usize..];
                let id_start = 0;
                let id_len = 2;
                let identifier = u16::from_be_bytes([data[id_start], data[id_start + 1]]);

                let properties_start = id_start + id_len;
                let (properties_len, properties_offset) =
                    if let Some((properties_len, properties_offset)) =
                        varint::decode(&data[properties_start..], 4)
                    {
                        (properties_len, properties_offset)
                    } else {
                        return Self::None;
                    };
                let properties_start = properties_start + properties_offset as usize;

                let reason_codes_start = properties_start + properties_len as usize;

                PacketType::SubAck {
                    identifier,
                    properties: if properties_len > 0 {
                        Some(&data[properties_start..(properties_start + properties_len as usize)])
                    } else {
                        None
                    },
                    reason_codes: &data[reason_codes_start..],
                }
            }
            128 => {
                if arr.len() < 3 {
                    return Self::None;
                }
                let (_remain_len, remain_len_size) =
                    if let Some((_remain_len, remain_len_size)) = varint::decode(&arr[1..], 4) {
                        (_remain_len, remain_len_size)
                    } else {
                        return Self::None;
                    };
                if _remain_len > arr.len() as u64 {
                    return Self::None;
                }
                let data = &arr[1 + remain_len_size as usize..];
                let id_start = 0;
                let id_len = 2;
                let identifier = u16::from_be_bytes([data[id_start], data[id_start + 1]]);

                let properties_start = id_start + id_len;
                let (properties_len, properties_offset) =
                    if let Some((properties_len, properties_offset)) =
                        varint::decode(&data[properties_start..], 4)
                    {
                        (properties_len, properties_offset)
                    } else {
                        return Self::None;
                    };
                let properties_start = properties_start + properties_offset as usize;

                let filters_start = properties_start + properties_len as usize;
                PacketType::Subscribe {
                    identifier,
                    properties: if properties_len > 0 {
                        Some(&data[properties_start..(properties_start + properties_len as usize)])
                    } else {
                        None
                    },
                    topic_filters: if filters_start < data.len() - 1 {
                        Some(&data[filters_start..])
                    } else {
                        None
                    },
                }
            }
            176 => {
                if arr.len() < 3 {
                    return Self::None;
                }
                let (_remain_len, remain_len_size) =
                    if let Some((_remain_len, remain_len_size)) = varint::decode(&arr[1..], 4) {
                        (_remain_len, remain_len_size)
                    } else {
                        return Self::None;
                    };
                if _remain_len > arr.len() as u64 {
                    return Self::None;
                }
                let data = &arr[1 + remain_len_size as usize..];
                let id_start = 0;
                let id_len = 2;
                let identifier = u16::from_be_bytes([data[id_start], data[id_start + 1]]);

                let properties_start = id_start + id_len;
                let (properties_len, properties_offset) =
                    if let Some((properties_len, properties_offset)) =
                        varint::decode(&data[properties_start..], 4)
                    {
                        (properties_len, properties_offset)
                    } else {
                        return Self::None;
                    };
                let properties_start = properties_start + properties_offset as usize;

                let reason_codes_start = properties_start + properties_len as usize;

                PacketType::UnsubAck {
                    identifier,
                    properties: if properties_len > 0 {
                        Some(&data[properties_start..(properties_start + properties_len as usize)])
                    } else {
                        None
                    },
                    reason_codes: &data[reason_codes_start..],
                }
            }
            160 => {
                if arr.len() < 3 {
                    return Self::None;
                }
                let (_remain_len, remain_len_size) =
                    if let Some((_remain_len, remain_len_size)) = varint::decode(&arr[1..], 4) {
                        (_remain_len, remain_len_size)
                    } else {
                        return Self::None;
                    };
                if _remain_len > arr.len() as u64 {
                    return Self::None;
                }
                let data = &arr[1 + remain_len_size as usize..];
                let id_start = 0;
                let id_len = 2;
                let identifier = u16::from_be_bytes([data[id_start], data[id_start + 1]]);

                let properties_start = id_start + id_len;
                let (properties_len, properties_offset) =
                    if let Some((properties_len, properties_offset)) =
                        varint::decode(&data[properties_start..], 4)
                    {
                        (properties_len, properties_offset)
                    } else {
                        return Self::None;
                    };
                let properties_start = properties_start + properties_offset as usize;

                let filters_start = properties_start + properties_len as usize;
                PacketType::Unsubscribe {
                    identifier,
                    properties: if properties_len > 0 {
                        Some(&data[properties_start..(properties_start + properties_len as usize)])
                    } else {
                        None
                    },
                    topic_filters: if filters_start < data.len() - 1 {
                        Some(&data[filters_start..])
                    } else {
                        None
                    },
                }
            }
            _ => PacketType::None,
        }
    }
}

// // // header
// // // [ 8  ]
// // mod PropertyId {
// //     const PAYLOAD_FORMAT_INDICATOR: u8 = 0x01; //Byte PUBLISH, Will Properties
// //     const MESSAGE_EXPIRY_INTERVAL: u8 = 0x02; //Four Byte Integer PUBLISH, Will Properties
// //     const CONTENT_TYPE: u8 = 0x03; //UTF-8 Encoded String PUBLISH, Will Properties
// //     const RESPONSE_TOPIC: u8 = 0x08; //UTF-8 Encoded String PUBLISH, Will Properties
// //     const CORRELATION_DATA: u8 = 0x09; //Binary Data PUBLISH, Will Properties
// //     const SUBSCRIPTION_IDENTIFIER: u8 = 0x0B; //Variable Byte Integer PUBLISH, SUBSCRIBE
// //     const SESSION_EXPIRY_INTERVAL: u8 = 0x11; //Four Byte Integer CONNECT, CONNACK, DISCONNECT mqtt-v5.0-os 07 March 2019 Standards Track Work Product Copyright © OASIS Open 2019. All Rights Reserved. Page 26 of 137
// //     const ASSIGNED_CLIENT_IDENTIFIER: u8 = 0x12; //UTF-8 Encoded String CONNACK
// //     const SERVER_KEEP_ALIVE: u8 = 0x13; //Two Byte Integer CONNACK
// //     const AUTHENTICATION_METHOD: u8 = 0x15; //UTF-8 Encoded String CONNECT, CONNACK, AUTH
// //     const AUTHENTICATION_DATA: u8 = 0x16; //Binary Data CONNECT, CONNACK, AUTH
// //     const REQUEST_PROBLEM_INFORMATION: u8 = 0x17; //Byte CONNECT
// //     const WILL_DELAY_INTERVAL: u8 = 0x18; //Four Byte Integer Will Properties
// //     const REQUEST_RESPONSE_INFORMATION: u8 = 0x19; // Byte CONNECT
// //     const RESPONSE_INFORMATION: u8 = 0x1A; //UTF-8 Encoded String CONNACK
// //     const SERVER_REFERENCE: u8 = 0x1C; //UTF-8 Encoded String CONNACK, DISCONNECT
// //     const REASON_STRING: u8 = 0x1F; //UTF-8 Encoded String CONNACK, PUBACK, PUBREC, PUBREL, PUBCOMP, SUBACK, UNSUBACK, DISCONNECT, AUTH
// //     const RECEIVE_MAXIMUM: u8 = 0x21; //Two Byte Integer CONNECT, CONNACK
// //     const TOPIC_ALIAS_MAXIMUM: u8 = 0x22; //Two Byte Integer CONNECT, CONNACK
// //     const TOPIC_ALIAS: u8 = 0x23; //Two Byte Integer PUBLISH
// //     const MAXIMUM_QOS: u8 = 0x24; //Byte CONNACK
// //     const RETAIN_AVAILABLE: u8 = 0x25; //Byte CONNACK
// //     const USER_PROPERTY: u8 = 0x26; //UTF-8 String Pair CONNECT, CONNACK, PUBLISH, Will Properties, PUBACK, PUBREC, PUBREL, PUBCOMP, SUBSCRIBE, SUBACK, UNSUBSCRIBE, UNSUBACK, DISCONNECT, AUTH
// //     const MAXIMUM_PACKET_SIZE: u8 = 0x27; //Four Byte Integer CONNECT, CONNACK
// //     const WILDCARD_SUBSCRIPTION_AVAILABLE: u8 = 0x28; //Byte CONNACK
// //     const SUBSCRIPTION_IDENTIFIER_AVAILABLE: u8 = 0x29; //Byte CONNACK
// //     const SHARED_SUBSCRIPTION_AVAILABLE: u8 = 0x2A; //Byte CONNACK
// // }

// // mod ReasonCode {
// //     const SUCCESS: u8 = 0x00; //CONNACK, PUBACK, PUBREC, PUBREL, PUBCOMP,UNSUBACK, AUTH
// //     const NORMAL_DISCONNECTION: u8 = 0x00; //DISCONNECT mqtt-v5.0-os 07 March 2019 Standards Track Work Product Copyright © OASIS Open 2019. All Rights Reserved. Page 28 of 137
// //     const GRANTED_QOS_0: u8 = 0x00; //SUBACK
// //     const GRANTED_QOS_1: u8 = 0x01; //SUBACK
// //     const GRANTED_QOS_2: u8 = 0x02; //SUBACK
// //     const DISCONNECT_WITH_WILL_MESSAGE: u8 = 0x04; //DISCONNECT
// //     const NO_MATCHING_SUBSCRIBERS: u8 = 0x10; //PUBACK, PUBREC
// //     const NO_SUBSCRIPTION_EXISTED: u8 = 0x11; //UNSUBACK
// //     const CONTINUE_AUTHENTICATION: u8 = 0x18; //AUTH
// //     const REAUTHENTICATE: u8 = 0x19; //AUTH
// //     const UNSPECIFIED_ERROR: u8 = 0x80; //CONNACK, PUBACK, PUBREC, SUBACK, UNSUBACK, DISCONNECT
// //     const MALFORMED_PACKET: u8 = 0x81; //CONNACK, DISCONNECT
// //     const PROTOCOL_ERROR: u8 = 0x82; //CONNACK, DISCONNECT
// //     const IMPLEMENTATION_SPECIFIC_ERROR: u8 = 0x83; //CONNACK, PUBACK, PUBREC, SUBACK, UNSUBACK, DISCONNECT
// //     const UNSUPPORTED_PROTOCOL_VERSION: u8 = 0x84; //CONNACK
// //     const CLIENT_IDENTIFIER_NOT_VALID: u8 = 0x85; //CONNACK
// //     const BAD_USER_NAME_OR_PASSWORD: u8 = 0x86; //CONNACK
// //     const NOT_AUTHORIZED: u8 = 0x87; //CONNACK, PUBACK, PUBREC, SUBACK, UNSUBACK, DISCONNECT
// //     const SERVER_UNAVAILABLE: u8 = 0x88; //CONNACK
// //     const SERVER_BUSY: u8 = 0x89; //CONNACK, DISCONNECT
// //     const BANNED: u8 = 0x8A; //CONNACK
// //     const SERVER_SHUTTING_DOWN: u8 = 0x8B; //DISCONNECT
// //     const BAD_AUTHENTICATION_METHOD: u8 = 0x8C; //CONNACK, DISCONNECT
// //     const KEEP_ALIVE_TIMEOUT: u8 = 0x8D; //DISCONNECT
// //     const SESSION_TAKEN_OVER: u8 = 0x8E; //DISCONNECT
// //     const TOPIC_FILTER_INVALID: u8 = 0x8F; //SUBACK, UNSUBACK, DISCONNECT
// //     const TOPIC_NAME_INVALID: u8 = 0x90; //CONNACK, PUBACK, PUBREC, DISCONNECT
// //     const PACKET_IDENTIFIER_IN_USE: u8 = 0x91; //PUBACK, PUBREC, SUBACK, UNSUBACK
// //     const PACKET_IDENTIFIER_NOT_FOUND: u8 = 0x92; //PUBREL, PUBCOMP
// //     const RECEIVE_MAXIMUM_EXCEEDED: u8 = 0x93; //DISCONNECT
// //     const TOPIC_ALIAS_INVALID: u8 = 0x94; //DISCONNECT
// //     const PACKET_TOO_LARGE: u8 = 0x95; //CONNACK, DISCONNECT
// //     const MESSAGE_RATE_TOO_HIGH: u8 = 0x96; //DISCONNECT
// //     const QUOTA_EXCEEDED: u8 = 0x97; //CONNACK, PUBACK, PUBREC, SUBACK, DISCONNECT
// //     const ADMINISTRATIVE_ACTION: u8 = 0x98; //DISCONNECT
// //     const PAYLOAD_FORMAT_INVALID: u8 = 0x99; //CONNACK, PUBACK, PUBREC, DISCONNECT
// //     const RETAIN_NOT_SUPPORTED: u8 = 0x9A; //CONNACK, DISCONNECT
// //     const QOS_NOT_SUPPORTED: u8 = 0x9B; //CONNACK, DISCONNECT
// //     const USE_ANOTHER_SERVER: u8 = 0x9C; //CONNACK, DISCONNECT
// //     const SERVER_MOVED: u8 = 0x9D; //CONNACK, DISCONNECT
// //     const SHARED_SUBSCRIPTIONS_NOT_SUPPORTED: u8 = 0x9E; //SUBACK, DISCONNECT
// //     const CONNECTION_RATE_EXCEEDED: u8 = 0x9F; //CONNACK, DISCONNECT
// //     const MAXIMUM_CONNECT_TIME: u8 = 0xA0; //DISCONNECT
// //     const SUBSCRIPTION_IDENTIFIERS_NOT_SUPPORTED: u8 = 0xA1; //SUBACK, DISCONNECT
// //     const WILDCARD_SUBSCRIPTIONS_NOT_SUPPORTED: u8 = 0xA2; //SUBACK, DISCONNECT
// // }

// // pub trait DataTrait {
// //     fn set_byte(&mut self, position: usize, value: u8);
// //     fn get_data(&self) -> &[u8];
// //     fn get_data_mut(&mut self) -> &mut [u8];
// //     fn resize(&mut self, length: u16);
// //     fn set_byte_array(&mut self, values: &[u8]);
// // }

// // pub trait HeaderTrait {
// //     fn set_type(&mut self, pkt_type: u8);
// //     fn set_flags(&mut self, flags: u8);
// // }

// // pub trait VariableHeaderTrait {
// //     fn set_id(&mut self, packet_id: u16);
// //     fn set_property_length(&mut self, length: u8);
// //     fn set_property_id(&mut self, id: u8);
// //     fn set_property_value(&mut self, value: &[u8]);
// // }

// // pub trait PayloadTrait {}

// // pub trait ReasonTrait {
// //     fn set_reason_code(&mut self, code: u8);
// // }

// // //pub trait Packet

// // pub struct PacketData {
// //     data: Vec<u8>,
// // }
// // impl PacketData {
// //     fn new() -> Self {
// //         Self { data: Vec::new() }
// //     }

// //     #[inline(always)]
// //     fn set_byte(&mut self, position: usize, value: u8) {
// //         self.data[position] = value;
// //     }

// //     #[inline(always)]
// //     fn get_data(&self) -> &[u8] {
// //         &self.data
// //     }

// //     #[inline(always)]
// //     fn get_data_mut(&mut self) -> &mut Vec<u8> {
// //         &mut self.data
// //     }

// //     #[inline(always)]
// //     fn resize(&mut self, length: u16) {
// //         self.data.resize(length as usize, 0);
// //     }

// //     #[inline(always)]
// //     fn set_byte_array(&mut self, values: &[u8]) {
// //         self.data.extend_from_slice(values);
// //     }

// //     fn replace_bytes_in_range(&mut self, range: Range<usize>, replace_with: &[u8]) {
// //         let new_len = range.end;
// //         if new_len > self.data.len() {
// //             self.data.resize(new_len, 0);
// //         }
// //         for (i, j) in range.enumerate() {
// //             self.data[j] = replace_with[i];
// //         }
// //     }
// // }
