// use super::{binary_data::BinaryData, mqtt_string::MQTTString, str_pair::MQTTStringPair, varint};

use super::varint;

// #[macro_export]
// macro_rules! property_bundle {
//     ($($x:expr),+ $(,)?) => (
//         PropertyBundle::from_array(Box::new([$($x),+]))
//     );
// }

// pub trait PropertyTrait<'a> {
//     type Item;
//     fn set_value(&mut self, prop: Self::Item);
//     fn get_data(&'a self) -> &'a [u8];
//     fn len(&self) -> usize;
// }

pub enum PropertyType<'pt> {
    Empty,
    TopicAlias(u16),
    MaximumQOS(u8),
    ReceiveMaximum(u16),
    ServerKeepAlive(u16),
    RetainAvailable(bool),
    TopicAliasMaximum(u16),
    WillDelayInterval(u32),
    MaximumPacketSize(u32),
    ContentType(&'pt [u8]),
    ReasonString(&'pt [u8]),
    RequestResponseInformation(bool),
    ResponseTopic(&'pt [u8]),
    SessionExpiryInterval(u32),
    MessageExpiryInterval(u32),
    ServerReference(&'pt [u8]),
    CorrelationData(&'pt [u8]),
    SubscriptionIdentifier(u64),
    PayloadFormatIndicator(bool),
    AuthenticationData(&'pt [u8]),
    ResponseInformation(&'pt [u8]),
    RequestProblemInformation(bool),
    AuthenticationMethod(&'pt [u8]),
    SharedSubscriptionAvailable(bool),
    WildcardSubscriptionAvailable(bool),
    UserProperties(&'pt [(&'pt [u8], &'pt [u8])]),
    AssignedClientIdentifier(&'pt [u8]),
    SubscriptionIdentifiersAvailable(bool),
}

impl<'pt> PropertyType<'pt> {
    pub fn get_data<'d>(&self, buf: &'d mut [u8]) -> usize {
        let mut index: usize = 0;
        match self {
            PropertyType::Empty => index,
            PropertyType::MaximumQOS(prop) => {
                buf[index] = 0x24;
                index += 1;
                buf[index] = *prop;
                index += 1;
                index
            }
            PropertyType::RetainAvailable(prop) => {
                buf[index] = 0x25;
                index += 1;
                buf[index] = *prop as u8;
                index += 1;
                index
            }
            PropertyType::RequestResponseInformation(prop) => {
                buf[index] = 0x19;
                index += 1;
                buf[index] = *prop as u8;
                index += 1;
                index
            }
            PropertyType::PayloadFormatIndicator(prop) => {
                buf[index] = 0x01;
                index += 1;
                buf[index] = *prop as u8;
                index += 1;
                index
            }
            PropertyType::RequestProblemInformation(prop) => {
                buf[index] = 0x17;
                index += 1;
                buf[index] = *prop as u8;
                index += 1;
                index
            }
            PropertyType::SharedSubscriptionAvailable(prop) => {
                buf[index] = 0x2A;
                index += 1;
                buf[index] = *prop as u8;
                index += 1;
                index
            }
            PropertyType::WildcardSubscriptionAvailable(prop) => {
                buf[index] = 0x28;
                index += 1;
                buf[index] = *prop as u8;
                index += 1;
                index
            }
            PropertyType::SubscriptionIdentifiersAvailable(prop) => {
                buf[index] = 0x29;
                index += 1;
                buf[index] = *prop as u8;
                index += 1;
                index
            }
            PropertyType::TopicAlias(prop) => {
                buf[index] = 0x23;
                index += 1;
                #[cfg(target_endian = "big")]
                {
                    buf[index] = *prop as u8;
                    index += 1;
                    buf[index] = (*prop >> 8) as u8;
                    index += 1;
                }
                #[cfg(target_endian = "little")]
                {
                    buf[index] = (*prop >> 8) as u8;
                    index += 1;
                    buf[index] = *prop as u8;
                    index += 1;
                }
                // #[cfg(target_endian = "big")]
                // data.extend_from_slice(&prop.to_ne_bytes());
                // #[cfg(target_endian = "little")]
                // data.extend_from_slice(&prop.to_be_bytes());
                index
            }
            PropertyType::ReceiveMaximum(prop) => {
                buf[index] = 0x21;
                index += 1;
                #[cfg(target_endian = "big")]
                {
                    buf[index] = *prop as u8;
                    index += 1;
                    buf[index] = (*prop >> 8) as u8;
                    index += 1;
                }
                #[cfg(target_endian = "little")]
                {
                    buf[index] = (*prop >> 8) as u8;
                    index += 1;
                    buf[index] = *prop as u8;
                    index += 1;
                }
                // #[cfg(target_endian = "big")]
                // data.extend_from_slice(eprop.to_ne_bytes());
                // #[cfg(target_endian = "little")]
                // data.extend_from_slice(&prop.to_be_bytes());
                index
            }
            PropertyType::ServerKeepAlive(prop) => {
                buf[index] = 0x13;
                index += 1;
                #[cfg(target_endian = "big")]
                {
                    buf[index] = *prop as u8;
                    index += 1;
                    buf[index] = (*prop >> 8) as u8;
                    index += 1;
                }
                #[cfg(target_endian = "little")]
                {
                    buf[index] = (*prop >> 8) as u8;
                    index += 1;
                    buf[index] = *prop as u8;
                    index += 1;
                }
                // #[cfg(target_endian = "big")]
                // data.extend_from_slice(&prop.to_ne_bytes());
                // #[cfg(target_endian = "little")]
                // data.extend_from_slice(&prop.to_be_bytes());
                // data
                index
            }
            PropertyType::TopicAliasMaximum(prop) => {
                buf[index] = 0x22;
                index += 1;
                #[cfg(target_endian = "big")]
                {
                    buf[index] = *prop as u8;
                    index += 1;
                    buf[index] = (*prop >> 8) as u8;
                    index += 1;
                }
                #[cfg(target_endian = "little")]
                {
                    buf[index] = (*prop >> 8) as u8;
                    index += 1;
                    buf[index] = *prop as u8;
                    index += 1;
                }
                // #[cfg(target_endian = "big")]
                // data.extend_from_slice(&prop.to_ne_bytes());
                // #[cfg(target_endian = "little")]
                // data.extend_from_slice(&prop.to_be_bytes());
                // data
                index
            }
            PropertyType::WillDelayInterval(prop) => {
                buf[index] = 0x18;
                index += 1;
                #[cfg(target_endian = "big")]
                {
                    buf[index] = *prop as u8;
                    index += 1;
                    buf[index] = (*prop >> 8) as u8;
                    index += 1;
                    buf[index] = (*prop >> 16) as u8;
                    index += 1;
                    buf[index] = (*prop >> 24) as u8;
                    index += 1;
                }
                #[cfg(target_endian = "little")]
                {
                    buf[index] = (*prop >> 24) as u8;
                    index += 1;
                    buf[index] = (*prop >> 16) as u8;
                    index += 1;
                    buf[index] = (*prop >> 8) as u8;
                    index += 1;
                    buf[index] = *prop as u8;
                    index += 1;
                }
                // #[cfg(target_endian = "big")]
                // data.extend_from_slice(&prop.to_ne_bytes());
                // #[cfg(target_endian = "little")]
                // data.extend_from_slice(&prop.to_be_bytes());
                // data
                index
            }
            PropertyType::MaximumPacketSize(prop) => {
                buf[index] = 0x27;
                index += 1;
                #[cfg(target_endian = "big")]
                {
                    buf[index] = *prop as u8;
                    index += 1;
                    buf[index] = (*prop >> 8) as u8;
                    index += 1;
                    buf[index] = (*prop >> 16) as u8;
                    index += 1;
                    buf[index] = (*prop >> 24) as u8;
                    index += 1;
                }
                #[cfg(target_endian = "little")]
                {
                    buf[index] = (*prop >> 24) as u8;
                    index += 1;
                    buf[index] = (*prop >> 16) as u8;
                    index += 1;
                    buf[index] = (*prop >> 8) as u8;
                    index += 1;
                    buf[index] = *prop as u8;
                    index += 1;
                }
                // #[cfg(target_endian = "big")]
                // data.extend_from_slice(&prop.to_ne_bytes());
                // #[cfg(target_endian = "little")]
                // data.extend_from_slice(&prop.to_be_bytes());
                // data
                index
            }
            PropertyType::SessionExpiryInterval(prop) => {
                buf[index] = 0x11;
                index += 1;
                #[cfg(target_endian = "big")]
                {
                    buf[index] = *prop as u8;
                    index += 1;
                    buf[index] = (*prop >> 8) as u8;
                    index += 1;
                    buf[index] = (*prop >> 16) as u8;
                    index += 1;
                    buf[index] = (*prop >> 24) as u8;
                    index += 1;
                }
                #[cfg(target_endian = "little")]
                {
                    buf[index] = (*prop >> 24) as u8;
                    index += 1;
                    buf[index] = (*prop >> 16) as u8;
                    index += 1;
                    buf[index] = (*prop >> 8) as u8;
                    index += 1;
                    buf[index] = *prop as u8;
                    index += 1;
                }
                // #[cfg(target_endian = "big")]
                // data.extend_from_slice(&prop.to_ne_bytes());
                // #[cfg(target_endian = "little")]
                // data.extend_from_slice(&prop.to_be_bytes());
                // data
                index
            }
            PropertyType::MessageExpiryInterval(prop) => {
                buf[index] = 0x02;
                index += 1;
                #[cfg(target_endian = "big")]
                {
                    buf[index] = *prop as u8;
                    index += 1;
                    buf[index] = (*prop >> 8) as u8;
                    index += 1;
                    buf[index] = (*prop >> 16) as u8;
                    index += 1;
                    buf[index] = (*prop >> 24) as u8;
                    index += 1;
                }
                #[cfg(target_endian = "little")]
                {
                    buf[index] = (*prop >> 24) as u8;
                    index += 1;
                    buf[index] = (*prop >> 16) as u8;
                    index += 1;
                    buf[index] = (*prop >> 8) as u8;
                    index += 1;
                    buf[index] = *prop as u8;
                    index += 1;
                }
                // #[cfg(target_endian = "big")]
                // data.extend_from_slice(&prop.to_ne_bytes());
                // #[cfg(target_endian = "little")]
                // data.extend_from_slice(&prop.to_be_bytes());
                // data
                index
            }
            PropertyType::SubscriptionIdentifier(prop) => {
                buf[index] = 0x0B;
                index += 1;
                let mut prop_varint = [0u8; 4];
                let len = varint::encode(*prop, &mut prop_varint);
                for idx in 0..len {
                    buf[index] = prop_varint[idx];
                    index += 1;
                }
                index
            }
            PropertyType::ContentType(prop) => {
                buf[index] = 0x03;
                index += 1;
                let len = prop.len();
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
                for c in *prop {
                    buf[index] = *c;
                    index += 1;
                }
                // data.extend_from_slice(prop.as_bytes());
                // data
                index
            }
            PropertyType::ReasonString(prop) => {
                buf[index] = 0x1F;
                index += 1;
                let len = prop.len();
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
                for c in *prop {
                    buf[index] = *c;
                    index += 1;
                }
                // data.extend_from_slice(prop.as_bytes());
                // data
                index
            }
            PropertyType::ResponseTopic(prop) => {
                buf[index] = 0x08;
                index += 1;
                let len = prop.len();
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
                for c in *prop {
                    buf[index] = *c;
                    index += 1;
                }
                // data.extend_from_slice(prop.as_bytes());
                // data
                index
            }
            PropertyType::ServerReference(prop) => {
                buf[index] = 0x1C;
                index += 1;
                let len = prop.len();
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
                for c in *prop {
                    buf[index] = *c;
                    index += 1;
                }
                // data.extend_from_slice(prop.as_bytes());
                // data
                index
            }
            PropertyType::ResponseInformation(prop) => {
                buf[index] = 0x1A;
                index += 1;
                let len = prop.len();
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
                for c in *prop {
                    buf[index] = *c;
                    index += 1;
                }
                // data.extend_from_slice(prop.as_bytes());
                // data
                index
            }
            PropertyType::AuthenticationMethod(prop) => {
                buf[index] = 0x15;
                index += 1;
                let len = prop.len();
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
                for c in *prop {
                    buf[index] = *c;
                    index += 1;
                }
                // data.extend_from_slice(prop.as_bytes());
                // data
                index
            }
            PropertyType::AssignedClientIdentifier(prop) => {
                buf[index] = 0x12;
                index += 1;
                let len = prop.len();
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
                for c in *prop {
                    buf[index] = *c;
                    index += 1;
                }
                // data.extend_from_slice(prop.as_bytes());
                // data
                index
            }
            PropertyType::CorrelationData(prop) => {
                buf[index] = 0x09;
                index += 1;
                let len = prop.len();
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
                for c in *prop {
                    buf[index] = *c;
                    index += 1;
                }
                // data.extend_from_slice(prop.as_slice());
                // data
                index
            }
            PropertyType::AuthenticationData(prop) => {
                buf[index] = 0x16;
                index += 1;
                let len = prop.len();
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
                for c in *prop {
                    buf[index] = *c;
                    index += 1;
                }
                // data.extend_from_slice(prop.as_slice());
                // data
                index
            }
            PropertyType::UserProperties(prop) => {
                for (key, value) in *prop {
                    buf[index] = 0x26;
                    index += 1;
                    let len = key.len();
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
                    for c in *key {
                        buf[index] = *c;
                        index += 1;
                    }
                    let len = value.len();
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
                    for c in *value {
                        buf[index] = *c;
                        index += 1;
                    }
                }
                // for p in prop {
                //     data.push(0x26);
                //     data.extend_from_slice(p.as_bytes());
                // }
                // data
                index
            }
        }
    }
}
pub fn bundle_properties(buf: &mut [u8], props: &[PropertyType]) -> usize {
    let mut index: usize = 0;
    let mut prop_buf = [0u8; 256];
    for prop in props {
        let prop_len = prop.get_data(&mut prop_buf);
        for c in &prop_buf[..prop_len] {
            buf[index] = *c;
            index += 1;
        }
    }
    index
}
pub fn bundle_sub_topic_filters(buf: &mut [u8], filters: &[(&[u8], u8)]) -> usize {
    let mut index = 0;
    for (filter, flag) in filters {
        let len = filter.len();
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
        for c in *filter {
            buf[index] = *c;
            index += 1;
        }
        buf[index] = *flag;
        index += 1;
    }
    index
}

pub fn bundle_unsub_topic_filters(buf: &mut [u8], filters: &[&[u8]]) -> usize {
    let mut index = 0;
    for filter in filters {
        let len = filter.len();
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
        for c in *filter {
            buf[index] = *c;
            index += 1;
        }
    }
    index
}
