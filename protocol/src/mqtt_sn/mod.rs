pub mod packet;
pub mod utils;

#[cfg(test)]

mod tests_packet {
    use crate::{
        mqtt_sn::packet::{PacketLengthType, PacketType},
        PacketSerializer,
    };

    #[test]
    fn test_advertise_message() {
        {
            let msg = PacketType::Advertise {
                len_type: PacketLengthType::Short,
                gwid: 0x01,
                duration: 128,
            };
            let mut data = [0u8; 256];
            let n = msg.serialize(&mut data);
            assert_eq!(&data[..n], &[0x05, 0x00, 0x01, 0x00, 128]);

            let mut data2 = [0u8; 256];
            let n = PacketType::deserialize(&data[..n]).serialize(&mut data2);
            assert_eq!(&data[..n], &[0x05, 0x00, 0x01, 0x00, 128]);
        }
        //---------------------------------------------------------------------------------//
        {
            let mut data = [0u8; 256];
            let msg = PacketType::Advertise {
                len_type: PacketLengthType::Long,
                gwid: 0x01,
                duration: 128,
            };
            let n = msg.serialize(&mut data);
            assert_eq!(&data[..n], &[0x01, 0x00, 0x07, 0x00, 0x01, 0x00, 128]);

            let mut data2 = [0u8; 256];
            let n = PacketType::deserialize(&data[..n]).serialize(&mut data2);
            assert_eq!(&data2[..n], &[0x01, 0x00, 0x07, 0x00, 0x01, 0x00, 128]);
        }
    }
    #[test]
    fn test_searchgw_message() {
        let msg = PacketType::SearchGw {
            len_type: PacketLengthType::Short,
            radius: 8,
        };
        let mut data = [0u8; 256];
        let msg = msg.serialize(&mut data);
        assert_eq!(&data[..msg], &[0x03, 0x01, 0x08]);
        let mut data2 = [0u8; 256];
        let msg = PacketType::deserialize(&data[..msg]).serialize(&mut data2);
        assert_eq!(&data2[..msg], &[0x03, 0x01, 0x08]);
        //     //---------------------------------------------------------------------------------//
        let msg = PacketType::SearchGw {
            len_type: PacketLengthType::Long,
            radius: 8,
        };
        let mut data = [0u8; 256];
        let msg = msg.serialize(&mut data);
        assert_eq!(&data[..msg], &[0x01, 0x00, 0x05, 0x01, 0x08]);
        let mut data2 = [0u8; 256];
        let msg = PacketType::deserialize(&data[..msg]).serialize(&mut data2);
        assert_eq!(&data2[..msg], &[0x01, 0x00, 0x05, 0x01, 0x08]);
        //     //---------------------------------------------------------------------------------//
        let msg = PacketType::GwInfo {
            len_type: PacketLengthType::Short,
            gwid: 4,
            gwaddr: Some(&[1, 2, 3, 4]),
        };
        let mut data = [0u8; 256];
        let msg = msg.serialize(&mut data);
        assert_eq!(&data[..msg], &[0x07, 0x02, 0x04, 1, 2, 3, 4]);
        let mut data2 = [0u8; 256];
        let msg = PacketType::deserialize(&data[..msg]).serialize(&mut data2);
        assert_eq!(&data2[..msg], &[0x07, 0x02, 0x04, 1, 2, 3, 4]);
        //     //---------------------------------------------------------------------------------//
        let msg = PacketType::GwInfo {
            len_type: PacketLengthType::Long,
            gwid: 4,
            gwaddr: Some(&[1, 2, 3, 4]),
        };
        let mut data = [0u8; 256];
        let msg = msg.serialize(&mut data);
        assert_eq!(&data[..msg], &[0x01, 0x00, 0x09, 0x02, 0x04, 1, 2, 3, 4]);
        let mut data2 = [0u8; 256];
        let msg = PacketType::deserialize(&data[..msg]).serialize(&mut data2);
        assert_eq!(&data2[..msg], &[0x01, 0x00, 0x09, 0x02, 0x04, 1, 2, 3, 4]);
        //     //---------------------------------------------------------------------------------//
    }
    #[test]
    fn test_connect_message() {
        let msg = PacketType::Connect {
            len_type: PacketLengthType::Short,
            flags: 0xFF,
            protocolid: 0x1C,
            duration: 100,
            client_id: Some(&[1, 2, 3, 4, 5, 6]),
        };
        let mut data = [0u8; 256];
        //
        let msg = msg.serialize(&mut data);
        assert_eq!(
            &data[..msg],
            &[12, 0x04, 0xFF, 0x1C, 0, 100, 1, 2, 3, 4, 5, 6]
        );
        let mut data2 = [0u8; 256];
        let msg = PacketType::deserialize(&data[..msg]).serialize(&mut data2);
        assert_eq!(
            &data2[..msg],
            &[12, 0x04, 0xFF, 0x1C, 0, 100, 1, 2, 3, 4, 5, 6]
        );
        //     //---------------------------------------------------------------------------------//
        let msg = PacketType::Connect {
            len_type: PacketLengthType::Long,
            flags: 0xFF,
            protocolid: 0x1C,
            duration: 100,
            client_id: Some(&[1, 2, 3, 4, 5, 6]),
        };
        let mut data = [0u8; 256];
        let msg = msg.serialize(&mut data);
        assert_eq!(
            &data[..msg],
            &[0x1, 0x00, 14, 0x04, 0xFF, 0x1C, 0, 100, 1, 2, 3, 4, 5, 6]
        );
        let mut data2 = [0u8; 256];
        let msg = PacketType::deserialize(&data[..msg]).serialize(&mut data2);
        assert_eq!(
            &data2[..msg],
            &[0x1, 0x00, 14, 0x04, 0xFF, 0x1C, 0, 100, 1, 2, 3, 4, 5, 6]
        );
        //     //---------------------------------------------------------------------------------//
        let msg = PacketType::Connack {
            len_type: PacketLengthType::Short,
            return_code: 10,
        };
        let mut data = [0u8; 256];
        let msg = msg.serialize(&mut data);
        assert_eq!(&data[..msg], &[3, 0x05, 10]);
        let mut data2 = [0u8; 256];
        let msg = PacketType::deserialize(&data[..msg]).serialize(&mut data2);
        assert_eq!(&data2[..msg], &[3, 0x05, 10]);
        //     //---------------------------------------------------------------------------------//
        let msg = PacketType::Connack {
            len_type: PacketLengthType::Long,
            return_code: 10,
        };
        let mut data = [0u8; 256];
        let msg = msg.serialize(&mut data);
        assert_eq!(&data[..msg], &[0x01, 0x00, 5, 0x05, 10]);
        let mut data2 = [0u8; 256];
        let msg = PacketType::deserialize(&data[..msg]).serialize(&mut data2);
        assert_eq!(&data2[..msg], &[0x01, 0x00, 5, 0x05, 10]);
    }
    //
    #[test]
    fn test_will_msg() {
        let msg = PacketType::WillTopicReq {
            len_type: PacketLengthType::Short,
        };
        let mut data = [0u8; 256];
        let msg = msg.serialize(&mut data);
        assert_eq!(&data[..msg], &[2, 0x06]);
        let mut data2 = [0u8; 256];
        let msg = PacketType::deserialize(&data[..msg]).serialize(&mut data2);
        assert_eq!(&data2[..msg], &[2, 0x06]);
        //     //---------------------------------------------------------------------------------//
        let msg = PacketType::WillTopicReq {
            len_type: PacketLengthType::Long,
        };
        let mut data = [0u8; 256];
        let msg = msg.serialize(&mut data);
        assert_eq!(&data[..msg], &[0x01, 0x00, 0x04, 0x06]);
        let mut data2 = [0u8; 256];
        let msg = PacketType::deserialize(&data[..msg]).serialize(&mut data2);
        assert_eq!(&data2[..msg], &[0x01, 0x00, 0x04, 0x06]);
        //     //---------------------------------------------------------------------------------//
        let msg = PacketType::WillTopic {
            len_type: PacketLengthType::Short,
            flags: 0x11,
            will_topic: Some(&[1, 2, 3, 4, 5]),
        };
        let mut data = [0u8; 256];
        let msg = msg.serialize(&mut data);
        assert_eq!(&data[..msg], &[8, 0x07, 0x11, 1, 2, 3, 4, 5]);
        let mut data2 = [0u8; 256];
        let msg = PacketType::deserialize(&data[..msg]).serialize(&mut data2);
        assert_eq!(&data2[..msg], &[8, 0x07, 0x11, 1, 2, 3, 4, 5]);
        //     //---------------------------------------------------------------------------------//
        let msg = PacketType::WillTopic {
            len_type: PacketLengthType::Long,
            flags: 0x11,
            will_topic: Some(&[1, 2, 3, 4, 5]),
        };
        let mut data = [0u8; 256];
        let msg = msg.serialize(&mut data);
        assert_eq!(&data[..msg], &[0x01, 0x00, 10, 0x07, 0x11, 1, 2, 3, 4, 5]);
        let mut data2 = [0u8; 256];
        let msg = PacketType::deserialize(&data[..msg]).serialize(&mut data2);
        assert_eq!(&data2[..msg], &[0x01, 0x00, 10, 0x07, 0x11, 1, 2, 3, 4, 5]);
        //     //---------------------------------------------------------------------------------//
        let msg = PacketType::WillMsgReq {
            len_type: PacketLengthType::Short,
        };
        let mut data = [0u8; 256];
        let msg = msg.serialize(&mut data);
        assert_eq!(&data[..msg], &[2, 0x08]);
        let mut data2 = [0u8; 256];
        let msg = PacketType::deserialize(&data[..msg]).serialize(&mut data2);
        assert_eq!(&data2[..msg], &[2, 0x08]);
        //     //---------------------------------------------------------------------------------//
        let msg = PacketType::WillMsgReq {
            len_type: PacketLengthType::Long,
        };
        let mut data = [0u8; 256];
        let msg = msg.serialize(&mut data);
        assert_eq!(&data[..msg], &[0x01, 0x00, 4, 0x08]);
        let mut data2 = [0u8; 256];
        let msg = PacketType::deserialize(&data[..msg]).serialize(&mut data2);
        assert_eq!(&data2[..msg], &[0x01, 0x00, 4, 0x08]);
        //     //---------------------------------------------------------------------------------//
        let msg = PacketType::WillMsg {
            len_type: PacketLengthType::Short,
            will_msg: Some(&[1, 2, 3, 4, 5]),
        };
        let mut data = [0u8; 256];
        let msg = msg.serialize(&mut data);
        assert_eq!(&data[..msg], &[7, 0x09, 1, 2, 3, 4, 5]);
        let mut data2 = [0u8; 256];
        let msg = PacketType::deserialize(&data[..msg]).serialize(&mut data2);
        assert_eq!(&data2[..msg], &[7, 0x09, 1, 2, 3, 4, 5]);
        //     //---------------------------------------------------------------------------------//
        let msg = PacketType::WillMsg {
            len_type: PacketLengthType::Long,
            will_msg: Some(&[1, 2, 3, 4, 5]),
        };
        let mut data = [0u8; 256];
        let msg = msg.serialize(&mut data);
        assert_eq!(&data[..msg], &[0x01, 0x00, 9, 0x09, 1, 2, 3, 4, 5]);
        let mut data2 = [0u8; 256];
        let msg = PacketType::deserialize(&data[..msg]).serialize(&mut data2);
        assert_eq!(&data2[..msg], &[0x01, 0x00, 9, 0x09, 1, 2, 3, 4, 5]);
    }
    #[test]
    pub fn test_register_msg() {
        let msg = PacketType::Register {
            len_type: PacketLengthType::Short,
            topicid: 10,
            msgid: 11,
            topic_name: Some(&[1, 2, 3, 4, 5]),
        };
        let mut data = [0u8; 256];
        let msg = msg.serialize(&mut data);
        assert_eq!(&data[..msg], &[11, 0x0A, 00, 10, 00, 11, 1, 2, 3, 4, 5]);
        let mut data2 = [0u8; 256];
        let msg = PacketType::deserialize(&data[..msg]).serialize(&mut data2);
        assert_eq!(&data2[..msg], &[11, 0x0A, 00, 10, 00, 11, 1, 2, 3, 4, 5]);
        //     //---------------------------------------------------------------------------------//
        let msg = PacketType::Register {
            len_type: PacketLengthType::Long,
            topicid: 10,
            msgid: 11,
            topic_name: Some(&[1, 2, 3, 4, 5]),
        };
        let mut data = [0u8; 256];
        let msg = msg.serialize(&mut data);
        assert_eq!(
            &data[..msg],
            &[0x01, 0x00, 13, 0x0A, 00, 10, 00, 11, 1, 2, 3, 4, 5]
        );
        let mut data2 = [0u8; 256];
        let msg = PacketType::deserialize(&data[..msg]).serialize(&mut data2);
        assert_eq!(
            &data2[..msg],
            &[0x01, 0x00, 13, 0x0A, 00, 10, 00, 11, 1, 2, 3, 4, 5]
        );
        //     //---------------------------------------------------------------------------------//
        let msg = PacketType::RegAck {
            len_type: PacketLengthType::Short,
            topicid: 0x11,
            msgid: 0x12,
            return_code: 0x05,
        };
        let mut data = [0u8; 256];
        let msg = msg.serialize(&mut data);
        assert_eq!(&data[..msg], &[7, 0x0B, 0x00, 0x11, 0x00, 0x12, 0x05]);
        let mut data2 = [0u8; 256];
        let msg = PacketType::deserialize(&data[..msg]).serialize(&mut data2);
        assert_eq!(&data2[..msg], &[7, 0x0B, 0x00, 0x11, 0x00, 0x12, 0x05]);
        //     //---------------------------------------------------------------------------------//
        let msg = PacketType::RegAck {
            len_type: PacketLengthType::Long,
            topicid: 0x11,
            msgid: 0x12,
            return_code: 0x05,
        };
        let mut data = [0u8; 256];
        let msg = msg.serialize(&mut data);
        assert_eq!(
            &data[..msg],
            &[0x01, 0x00, 9, 0x0B, 0x00, 0x11, 0x00, 0x12, 0x05]
        );
        let mut data2 = [0u8; 256];
        let msg = PacketType::deserialize(&data[..msg]).serialize(&mut data2);
        assert_eq!(
            &data2[..msg],
            &[0x01, 0x00, 9, 0x0B, 0x00, 0x11, 0x00, 0x12, 0x05]
        );
    }
    #[test]
    pub fn test_publish_msg() {
        let msg = PacketType::Publish {
            len_type: PacketLengthType::Short,
            flags: 0x10,
            topicid: 0x11,
            msgid: 0x12,
            data: Some(&[1, 2, 3, 4, 5]),
        };
        let mut data = [0u8; 256];
        let msg = msg.serialize(&mut data);
        assert_eq!(
            &data[..msg],
            &[12, 0x0C, 0x10, 0x00, 0x11, 0x00, 0x12, 1, 2, 3, 4, 5]
        );
        let mut data2 = [0u8; 256];
        let msg = PacketType::deserialize(&data[..msg]).serialize(&mut data2);
        assert_eq!(
            &data2[..msg],
            &[12, 0x0C, 0x10, 0x00, 0x11, 0x00, 0x12, 1, 2, 3, 4, 5]
        );
        //     //---------------------------------------------------------------------------------//
        let msg = PacketType::Publish {
            len_type: PacketLengthType::Long,
            flags: 0x10,
            topicid: 0x11,
            msgid: 0x12,
            data: Some(&[1, 2, 3, 4, 5]),
        };
        let mut data = [0u8; 256];
        let msg = msg.serialize(&mut data);
        assert_eq!(
            &data[..msg],
            &[0x01, 0x00, 14, 0x0C, 0x10, 0x00, 0x11, 0x00, 0x12, 1, 2, 3, 4, 5]
        );
        let mut data2 = [0u8; 256];
        let msg = PacketType::deserialize(&data[..msg]).serialize(&mut data2);
        assert_eq!(
            &data2[..msg],
            &[0x01, 0x00, 14, 0x0C, 0x10, 0x00, 0x11, 0x00, 0x12, 1, 2, 3, 4, 5]
        );
        //     //---------------------------------------------------------------------------------//
        let msg = PacketType::PubAck {
            len_type: PacketLengthType::Short,
            topicid: 0x11,
            msgid: 0x12,
            return_code: 0x05,
        };
        let mut data = [0u8; 256];
        let msg = msg.serialize(&mut data);
        assert_eq!(&data[..msg], &[7, 0x0d, 0x00, 0x11, 0x00, 0x12, 0x05]);
        let mut data2 = [0u8; 256];
        let msg = PacketType::deserialize(&data[..msg]).serialize(&mut data2);
        assert_eq!(&data2[..msg], &[7, 0x0d, 0x00, 0x11, 0x00, 0x12, 0x05]);
        //     //---------------------------------------------------------------------------------//
        let msg = PacketType::PubAck {
            len_type: PacketLengthType::Long,
            topicid: 0x11,
            msgid: 0x12,
            return_code: 0x05,
        };
        let mut data = [0u8; 256];
        let msg = msg.serialize(&mut data);
        assert_eq!(
            &data[..msg],
            &[0x01, 0x00, 9, 0x0d, 0x00, 0x11, 0x00, 0x12, 0x05]
        );
        let mut data2 = [0u8; 256];
        let msg = PacketType::deserialize(&data[..msg]).serialize(&mut data2);
        assert_eq!(
            &data2[..msg],
            &[0x01, 0x00, 9, 0x0d, 0x00, 0x11, 0x00, 0x12, 0x05]
        );
    }
    #[test]
    pub fn test_pub_rec_rel_comp_msg() {
        let msg = PacketType::PubRec {
            len_type: PacketLengthType::Short,
            msgid: 0x01,
        };
        let mut data = [0u8; 256];
        let msg = msg.serialize(&mut data);
        assert_eq!(&data[..msg], &[4, 0x0F, 0x00, 0x01]);
        let mut data2 = [0u8; 256];
        let msg = PacketType::deserialize(&data[..msg]).serialize(&mut data2);
        assert_eq!(&data2[..msg], &[4, 0x0F, 0x00, 0x01]);
        //     //---------------------------------------------------------------------------------//
        let msg = PacketType::PubRec {
            len_type: PacketLengthType::Long,
            msgid: 0x01,
        };
        let mut data = [0u8; 256];
        let msg = msg.serialize(&mut data);
        assert_eq!(&data[..msg], &[0x01, 0x00, 6, 0x0F, 0x00, 0x01]);
        let mut data2 = [0u8; 256];
        let msg = PacketType::deserialize(&data[..msg]).serialize(&mut data2);
        assert_eq!(&data2[..msg], &[0x01, 0x00, 6, 0x0F, 0x00, 0x01]);
        //     //---------------------------------------------------------------------------------//
        let msg = PacketType::PubRel {
            len_type: PacketLengthType::Short,
            msgid: 0x02,
        };
        let mut data = [0u8; 256];
        let msg = msg.serialize(&mut data);
        assert_eq!(&data[..msg], &[4, 0x010, 0x00, 0x02]);
        let mut data2 = [0u8; 256];
        let msg = PacketType::deserialize(&data[..msg]).serialize(&mut data2);
        assert_eq!(&data2[..msg], &[4, 0x010, 0x00, 0x02]);
        //     //---------------------------------------------------------------------------------//
        let msg = PacketType::PubRel {
            len_type: PacketLengthType::Long,
            msgid: 0x02,
        };
        let mut data = [0u8; 256];
        let msg = msg.serialize(&mut data);
        assert_eq!(&data[..msg], &[0x01, 0x00, 6, 0x010, 0x00, 0x02]);
        let mut data2 = [0u8; 256];
        let msg = PacketType::deserialize(&data[..msg]).serialize(&mut data2);
        assert_eq!(&data2[..msg], &[0x01, 0x00, 6, 0x010, 0x00, 0x02]);
        //     //---------------------------------------------------------------------------------//
        let msg = PacketType::PubComp {
            len_type: PacketLengthType::Short,
            msgid: 0x03,
        };
        let mut data = [0u8; 256];
        let msg = msg.serialize(&mut data);
        assert_eq!(&data[..msg], &[4, 0x0E, 0x00, 0x03]);
        let mut data2 = [0u8; 256];
        let msg = PacketType::deserialize(&data[..msg]).serialize(&mut data2);
        assert_eq!(&data2[..msg], &[4, 0x0E, 0x00, 0x03]);
        //     //---------------------------------------------------------------------------------//
        let msg = PacketType::PubComp {
            len_type: PacketLengthType::Long,
            msgid: 0x03,
        };
        let mut data = [0u8; 256];
        let msg = msg.serialize(&mut data);
        assert_eq!(&data[..msg], &[0x01, 0x00, 6, 0x0E, 0x00, 0x03]);
        let mut data2 = [0u8; 256];
        let msg = PacketType::deserialize(&data[..msg]).serialize(&mut data2);
        assert_eq!(&data2[..msg], &[0x01, 0x00, 6, 0x0E, 0x00, 0x03]);
    }
    #[test]
    pub fn test_subscribe_msg() {
        let msg = PacketType::Subscribe {
            len_type: PacketLengthType::Short,
            flags: 0x00,
            msgid: 0x11,
            topic: "12345".as_bytes(),
        };
        let mut data = [0u8; 256];
        let msg = msg.serialize(&mut data);
        assert_eq!(
            &data[..msg],
            &[12, 0x12, 0x00, 0x00, 0x11, 0x00, 0x05, 49, 50, 51, 52, 53]
        );
        let mut data2 = [0u8; 256];
        let msg = PacketType::deserialize(&data[..msg]).serialize(&mut data2[..msg]);
        assert_eq!(
            &data2[..msg],
            &[12, 0x12, 0x00, 0x00, 0x11, 0x00, 0x05, 49, 50, 51, 52, 53]
        );
        //     //---------------------------------------------------------------------------------//
        let msg = PacketType::Subscribe {
            len_type: PacketLengthType::Short,
            flags: 0x01,
            msgid: 0x11,
            topic: &100_u16.to_be_bytes(),
        };
        let mut data = [0u8; 256];
        let msg = msg.serialize(&mut data);
        assert_eq!(&data[..msg], &[7, 0x012, 0x01, 0x00, 0x11, 0x00, 100]);
        let mut data2 = [0u8; 256];
        let msg = PacketType::deserialize(&data[..msg]).serialize(&mut data2);
        assert_eq!(&data2[..msg], &[7, 0x012, 0x01, 0x00, 0x11, 0x00, 100]);
        //     //---------------------------------------------------------------------------------//
        let msg = PacketType::Subscribe {
            len_type: PacketLengthType::Short,
            flags: 0x02,
            msgid: 0x11,
            topic: &[1, 2],
        };
        let mut data = [0u8; 256];
        let msg = msg.serialize(&mut data);
        assert_eq!(&data[..msg], &[7, 0x012, 0x02, 0x00, 0x11, 1, 2]);
        let mut data2 = [0u8; 256];
        let msg = PacketType::deserialize(&data[..msg]).serialize(&mut data2);
        assert_eq!(&data2[..msg], &[7, 0x012, 0x02, 0x00, 0x11, 1, 2]);
        //     //---------------------------------------------------------------------------------//
        let msg = PacketType::Subscribe {
            len_type: PacketLengthType::Long,
            flags: 0x00,
            msgid: 0x11,
            topic: "12345".as_bytes(),
        };
        let mut data = [0u8; 256];
        let msg = msg.serialize(&mut data);
        assert_eq!(
            &data[..msg],
            &[0x01, 0x00, 14, 0x012, 0x00, 0x00, 0x11, 0x00, 0x05, 49, 50, 51, 52, 53]
        );
        let mut data2 = [0u8; 256];
        let msg = PacketType::deserialize(&data[..msg]).serialize(&mut data2);
        assert_eq!(
            &data2[..msg],
            &[0x01, 0x00, 14, 0x012, 0x00, 0x00, 0x11, 0x00, 0x05, 49, 50, 51, 52, 53]
        );
        //     //---------------------------------------------------------------------------------//
        let msg = PacketType::Subscribe {
            len_type: PacketLengthType::Long,
            flags: 0x01,
            msgid: 0x11,
            topic: &100_u16.to_be_bytes(),
        };
        let mut data = [0u8; 256];
        let msg = msg.serialize(&mut data);
        assert_eq!(
            &data[..msg],
            &[0x01, 0x00, 9, 0x012, 0x01, 0x00, 0x11, 0x00, 100]
        );
        let mut data2 = [0u8; 256];
        let msg = PacketType::deserialize(&data[..msg]).serialize(&mut data2);
        assert_eq!(
            &data2[..msg],
            &[0x01, 0x00, 9, 0x012, 0x01, 0x00, 0x11, 0x00, 100]
        );
        //     //---------------------------------------------------------------------------------//
        let msg = PacketType::Subscribe {
            len_type: PacketLengthType::Long,
            flags: 0x02,
            msgid: 0x11,
            topic: &[1, 2],
        };
        let mut data = [0u8; 256];
        let msg = msg.serialize(&mut data);
        assert_eq!(
            &data[..msg],
            &[0x01, 0x00, 9, 0x012, 0x02, 0x00, 0x11, 1, 2]
        );
        let mut data2 = [0u8; 256];
        let msg = PacketType::deserialize(&data[..msg]).serialize(&mut data2);
        assert_eq!(
            &data2[..msg],
            &[0x01, 0x00, 9, 0x012, 0x02, 0x00, 0x11, 1, 2]
        );
        //     //---------------------------------------------------------------------------------//
        let msg = PacketType::SubAck {
            len_type: PacketLengthType::Short,
            flags: 0x01,
            topicid: 0x11,
            msgid: 0x012,
            return_code: 0x05,
        };
        let mut data = [0u8; 256];
        let msg = msg.serialize(&mut data);
        assert_eq!(&data[..msg], &[8, 0x13, 0x01, 0x00, 0x11, 0x00, 0x12, 0x05]);
        let mut data2 = [0u8; 256];
        let msg = PacketType::deserialize(&data[..msg]).serialize(&mut data2);
        assert_eq!(
            &data2[..msg],
            &[8, 0x13, 0x01, 0x00, 0x11, 0x00, 0x12, 0x05]
        );
        //     //---------------------------------------------------------------------------------//
        let msg = PacketType::SubAck {
            len_type: PacketLengthType::Long,
            flags: 0x01,
            topicid: 0x11,
            msgid: 0x012,
            return_code: 0x05,
        };
        let mut data = [0u8; 256];
        let msg = msg.serialize(&mut data);
        assert_eq!(
            &data[..msg],
            &[0x01, 0x00, 10, 0x13, 0x01, 0x00, 0x11, 0x00, 0x12, 0x05]
        );
        let mut data2 = [0u8; 256];
        let msg = PacketType::deserialize(&data[..msg]).serialize(&mut data2);
        assert_eq!(
            &data2[..msg],
            &[0x01, 0x00, 10, 0x13, 0x01, 0x00, 0x11, 0x00, 0x12, 0x05]
        );
        //     //---------------------------------------------------------------------------------//
        let msg = PacketType::Unsubscribe {
            len_type: PacketLengthType::Short,
            flags: 0x00,
            msgid: 0x11,
            topic: "12345".as_bytes(),
        };
        let mut data = [0u8; 256];
        let msg = msg.serialize(&mut data);
        assert_eq!(
            &data[..msg],
            &[12, 0x014, 0x00, 0x00, 0x11, 0x00, 0x05, 49, 50, 51, 52, 53]
        );
        let mut data2 = [0u8; 256];
        let msg = PacketType::deserialize(&data[..msg]).serialize(&mut data2);
        assert_eq!(
            &data2[..msg],
            &[12, 0x014, 0x00, 0x00, 0x11, 0x00, 0x05, 49, 50, 51, 52, 53]
        );
        //     //---------------------------------------------------------------------------------//
        let msg = PacketType::Unsubscribe {
            len_type: PacketLengthType::Short,
            flags: 0x01,
            msgid: 0x11,
            topic: &100_u16.to_be_bytes(),
        };
        let mut data = [0u8; 256];
        let msg = msg.serialize(&mut data);
        assert_eq!(&data[..msg], &[7, 0x014, 0x01, 0x00, 0x11, 0x00, 100]);
        let mut data2 = [0u8; 256];
        let msg = PacketType::deserialize(&data[..msg]).serialize(&mut data2);
        assert_eq!(&data2[..msg], &[7, 0x014, 0x01, 0x00, 0x11, 0x00, 100]);
        //     //---------------------------------------------------------------------------------//
        let msg = PacketType::Unsubscribe {
            len_type: PacketLengthType::Short,
            flags: 0x02,
            msgid: 0x11,
            topic: &[1, 2],
        };
        let mut data = [0u8; 256];
        let msg = msg.serialize(&mut data);
        assert_eq!(&data[..msg], &[7, 0x014, 0x02, 0x00, 0x11, 1, 2]);
        let mut data2 = [0u8; 256];
        let msg = PacketType::deserialize(&data[..msg]).serialize(&mut data2);
        assert_eq!(&data2[..msg], &[7, 0x014, 0x02, 0x00, 0x11, 1, 2]);
        //     //---------------------------------------------------------------------------------//
        let msg = PacketType::Unsubscribe {
            len_type: PacketLengthType::Long,
            flags: 0x00,
            msgid: 0x11,
            topic: "12345".as_bytes(),
        };
        let mut data = [0u8; 256];
        let msg = msg.serialize(&mut data);
        assert_eq!(
            &data[..msg],
            &[0x01, 0x00, 14, 0x014, 0x00, 0x00, 0x11, 0, 5, 49, 50, 51, 52, 53]
        );
        let mut data2 = [0u8; 256];
        let msg = PacketType::deserialize(&data[..msg]).serialize(&mut data2);
        assert_eq!(
            &data2[..msg],
            &[0x01, 0x00, 14, 0x014, 0x00, 0x00, 0x11, 0, 5, 49, 50, 51, 52, 53]
        );
        //     //---------------------------------------------------------------------------------//
        let msg = PacketType::Unsubscribe {
            len_type: PacketLengthType::Long,
            flags: 0x01,
            msgid: 0x11,
            topic: &100_u16.to_be_bytes(),
        };
        let mut data = [0u8; 256];
        let msg = msg.serialize(&mut data);
        assert_eq!(
            &data[..msg],
            &[0x01, 0x00, 9, 0x014, 0x01, 0x00, 0x11, 0x00, 100]
        );
        let mut data2 = [0u8; 256];
        let msg = PacketType::deserialize(&data[..msg]).serialize(&mut data2);
        assert_eq!(
            &data[..msg],
            &[0x01, 0x00, 9, 0x014, 0x01, 0x00, 0x11, 0x00, 100]
        );
        //     //---------------------------------------------------------------------------------//
        let msg = PacketType::Unsubscribe {
            len_type: PacketLengthType::Long,
            flags: 0x02,
            msgid: 0x11,
            topic: &[1, 2],
        };
        let mut data = [0u8; 256];
        let msg = msg.serialize(&mut data);
        assert_eq!(
            &data[..msg],
            &[0x01, 0x00, 9, 0x014, 0x02, 0x00, 0x11, 1, 2]
        );
        let mut data2 = [0u8; 256];
        let msg = PacketType::deserialize(&data[..msg]).serialize(&mut data2);
        assert_eq!(
            &data2[..msg],
            &[0x01, 0x00, 9, 0x014, 0x02, 0x00, 0x11, 1, 2]
        );
        //     //---------------------------------------------------------------------------------//
        let msg = PacketType::UnsubAck {
            len_type: PacketLengthType::Short,
            msgid: 0x11,
        };
        let mut data = [0u8; 256];
        let msg = msg.serialize(&mut data);
        assert_eq!(&data[..msg], &[4, 0x15, 0x00, 0x11]);
        let mut data2 = [0u8; 256];
        let msg = PacketType::deserialize(&data[..msg]).serialize(&mut data2);
        assert_eq!(&data2[..msg], &[4, 0x15, 0x00, 0x11]);
        //     //---------------------------------------------------------------------------------//
        let msg = PacketType::UnsubAck {
            len_type: PacketLengthType::Long,
            msgid: 0x11,
        };
        let mut data = [0u8; 256];
        let msg = msg.serialize(&mut data);
        assert_eq!(&data[..msg], &[0x01, 0x00, 6, 0x15, 0x00, 0x11]);
        let mut data2 = [0u8; 256];
        let msg = PacketType::deserialize(&data[..msg]).serialize(&mut data2);
        assert_eq!(&data2[..msg], &[0x01, 0x00, 6, 0x15, 0x00, 0x11]);
    }
    #[test]
    pub fn test_ping_msg() {
        let msg = PacketType::PingReq {
            len_type: PacketLengthType::Short,
            client_id: Some(&[1, 2, 3, 4, 5]),
        };
        let mut data = [0u8; 256];
        let msg = msg.serialize(&mut data);
        assert_eq!(&data[..msg], &[7, 0x16, 1, 2, 3, 4, 5]);
        let mut data2 = [0u8; 256];
        let msg = PacketType::deserialize(&data[..msg]).serialize(&mut data2);
        assert_eq!(&data2[..msg], &[7, 0x16, 1, 2, 3, 4, 5]);
        //     //---------------------------------------------------------------------------------//
        let msg = PacketType::PingReq {
            len_type: PacketLengthType::Long,
            client_id: Some(&[1, 2, 3, 4, 5]),
        };
        let mut data = [0u8; 256];
        let msg = msg.serialize(&mut data);
        assert_eq!(&data[..msg], &[0x01, 0x00, 9, 0x16, 1, 2, 3, 4, 5]);
        let mut data2 = [0u8; 256];
        let msg = PacketType::deserialize(&data[..msg]).serialize(&mut data2);
        assert_eq!(&data2[..msg], &[0x01, 0x00, 9, 0x16, 1, 2, 3, 4, 5]);
        //     //---------------------------------------------------------------------------------//
        let msg = PacketType::PingResp {
            len_type: PacketLengthType::Short,
        };
        let mut data = [0u8; 256];
        let msg = msg.serialize(&mut data);
        assert_eq!(&data[..msg], &[2, 0x17]);
        let mut data2 = [0u8; 256];
        let msg = PacketType::deserialize(&data[..msg]).serialize(&mut data2);
        assert_eq!(&data2[..msg], &[2, 0x17]);
        //     //---------------------------------------------------------------------------------//
        let msg = PacketType::PingResp {
            len_type: PacketLengthType::Long,
        };
        let mut data = [0u8; 256];
        let msg = msg.serialize(&mut data);
        assert_eq!(&data[..msg], &[0x01, 0x00, 4, 0x17]);
        let mut data2 = [0u8; 256];
        let msg = PacketType::deserialize(&data[..msg]).serialize(&mut data2);
        assert_eq!(&data2[..msg], &[0x01, 0x00, 4, 0x17]);
    }
    #[test]
    pub fn test_disconnect() {
        let msg = PacketType::Disconnect {
            len_type: PacketLengthType::Short,
            duration: 100,
        };
        let mut data = [0u8; 256];
        let msg = msg.serialize(&mut data);
        assert_eq!(&data[..msg], &[4, 0x18, 0x00, 100]);
        let mut data2 = [0u8; 256];
        let msg = PacketType::deserialize(&data[..msg]).serialize(&mut data2);
        assert_eq!(&data2[..msg], &[4, 0x18, 0x00, 100]);
        //     //---------------------------------------------------------------------------------//
        let msg = PacketType::Disconnect {
            len_type: PacketLengthType::Long,
            duration: 100,
        };
        let mut data = [0u8; 256];
        let msg = msg.serialize(&mut data);
        assert_eq!(&data[..msg], &[0x01, 0x00, 6, 0x18, 0x00, 100]);
        let mut data2 = [0u8; 256];
        let msg = PacketType::deserialize(&data[..msg]).serialize(&mut data2);
        assert_eq!(&data2[..msg], &[0x01, 0x00, 6, 0x18, 0x00, 100]);
    }
    #[test]
    pub fn test_will_update_msg() {
        let msg = PacketType::WillTopicUpd {
            len_type: PacketLengthType::Short,
            flags: 0x11,
            will_topic: Some(&[1, 2, 3, 4, 5]),
        };
        let mut data = [0u8; 256];
        let msg = msg.serialize(&mut data);
        assert_eq!(&data[..msg], &[8, 0x1A, 0x11, 1, 2, 3, 4, 5]);
        let mut data2 = [0u8; 256];
        let msg = PacketType::deserialize(&data[..msg]).serialize(&mut data2);
        assert_eq!(&data2[..msg], &[8, 0x1A, 0x11, 1, 2, 3, 4, 5]);
        //     //---------------------------------------------------------------------------------//
        let msg = PacketType::WillTopicUpd {
            len_type: PacketLengthType::Long,
            flags: 0x11,
            will_topic: Some(&[1, 2, 3, 4, 5]),
        };
        let mut data = [0u8; 256];
        let msg = msg.serialize(&mut data);
        assert_eq!(&data[..msg], &[0x01, 0x00, 10, 0x1A, 0x11, 1, 2, 3, 4, 5]);
        let mut data2 = [0u8; 256];
        let msg = PacketType::deserialize(&data[..msg]).serialize(&mut data2);
        assert_eq!(&data2[..msg], &[0x01, 0x00, 10, 0x1A, 0x11, 1, 2, 3, 4, 5]);
        //     //---------------------------------------------------------------------------------//
        let msg = PacketType::WillMsgUpd {
            len_type: PacketLengthType::Short,
            will_msg: Some(&[1, 2, 3, 4, 5]),
        };
        let mut data = [0u8; 256];
        let msg = msg.serialize(&mut data);
        assert_eq!(&data[..msg], &[7, 0x1C, 1, 2, 3, 4, 5]);
        let mut data2 = [0u8; 256];
        let msg = PacketType::deserialize(&data[..msg]).serialize(&mut data2);
        assert_eq!(&data2[..msg], &[7, 0x1C, 1, 2, 3, 4, 5]);
        //     //---------------------------------------------------------------------------------//
        let msg = PacketType::WillMsgUpd {
            len_type: PacketLengthType::Long,
            will_msg: Some(&[1, 2, 3, 4, 5]),
        };
        let mut data = [0u8; 256];
        let msg = msg.serialize(&mut data);
        assert_eq!(&data[..msg], &[0x01, 0x00, 9, 0x1C, 1, 2, 3, 4, 5]);
        let mut data2 = [0u8; 256];
        let msg = PacketType::deserialize(&data[..msg]).serialize(&mut data2);
        assert_eq!(&data2[..msg], &[0x01, 0x00, 9, 0x1C, 1, 2, 3, 4, 5]);
        //     //---------------------------------------------------------------------------------//
        let msg = PacketType::WillTopicResp {
            len_type: PacketLengthType::Short,
            return_code: 100,
        };
        let mut data = [0u8; 256];
        let msg = msg.serialize(&mut data);
        assert_eq!(&data[..msg], &[3, 0x1B, 100]);
        let mut data2 = [0u8; 256];
        let msg = PacketType::deserialize(&data[..msg]).serialize(&mut data2);
        assert_eq!(&data2[..msg], &[3, 0x1B, 100]);
        //     //---------------------------------------------------------------------------------//
        let msg = PacketType::WillTopicResp {
            len_type: PacketLengthType::Long,
            return_code: 100,
        };
        let mut data = [0u8; 256];
        let msg = msg.serialize(&mut data);
        assert_eq!(&data[..msg], &[0x01, 0x00, 5, 0x1B, 100]);
        let mut data2 = [0u8; 256];
        let msg = PacketType::deserialize(&data[..msg]).serialize(&mut data2);
        assert_eq!(&data2[..msg], &[0x01, 0x00, 5, 0x1B, 100]);
        //     //---------------------------------------------------------------------------------//
        let msg = PacketType::WillMsgResp {
            len_type: PacketLengthType::Short,
            return_code: 100,
        };
        let mut data = [0u8; 256];
        let msg = msg.serialize(&mut data);
        assert_eq!(&data[..msg], &[3, 0x1D, 100]);
        let mut data2 = [0u8; 256];
        let msg = PacketType::deserialize(&data[..msg]).serialize(&mut data2);
        assert_eq!(&data2[..msg], &[3, 0x1D, 100]);
        //     //---------------------------------------------------------------------------------//
        let msg = PacketType::WillMsgResp {
            len_type: PacketLengthType::Long,
            return_code: 100,
        };
        let mut data = [0u8; 256];
        let msg = msg.serialize(&mut data);
        assert_eq!(&data[..msg], &[0x01, 0x00, 5, 0x1D, 100]);
        let mut data2 = [0u8; 256];
        let msg = PacketType::deserialize(&data[..msg]).serialize(&mut data2);
        assert_eq!(&data2[..msg], &[0x01, 0x00, 5, 0x1D, 100]);
    }
    #[test]
    pub fn test_forwarder_capsule() {
        let msg = PacketType::ForwarderCapsule {
            len_type: PacketLengthType::Short,
            control: 0x11,
            wnode_id: Some(&[1, 2, 3, 4, 5]),
            mqttsn_msg: Some(&[11, 22, 33, 44, 55]),
        };
        let mut data = [0u8; 256];
        let msg = msg.serialize(&mut data);
        assert_eq!(
            &data[..msg],
            &[14, 0xFE, 0x11, 0x05, 1, 2, 3, 4, 5, 11, 22, 33, 44, 55]
        );
        let mut data2 = [0u8; 256];
        let msg = PacketType::deserialize(&data[..msg]).serialize(&mut data2);
        assert_eq!(
            &data2[..msg],
            &[14, 0xFE, 0x11, 0x05, 1, 2, 3, 4, 5, 11, 22, 33, 44, 55]
        );
        //---------------------------------------------------------------------------------//
        let msg = PacketType::ForwarderCapsule {
            len_type: PacketLengthType::Long,
            control: 0x11,
            mqttsn_msg: Some(&[66, 22, 33, 44, 55]),
            wnode_id: Some(&[12, 2, 3, 4, 5]),
        };
        let mut data = [0u8; 256];
        let msg = msg.serialize(&mut data);
        assert_eq!(
            &data[..msg],
            &[0x01, 0x00, 16, 0xFE, 0x11, 0x05, 12, 2, 3, 4, 5, 66, 22, 33, 44, 55]
        );
        let mut data2 = [0u8; 256];
        let msg = PacketType::deserialize(&data[..msg]).serialize(&mut data2);
        assert_eq!(
            &data2[..msg],
            &[0x01, 0x00, 16, 0xFE, 0x11, 0x05, 12, 2, 3, 4, 5, 66, 22, 33, 44, 55]
        );
    }
}

// #[cfg(test)]
// mod test_utils {
//     use super::utils::{Flags, QoSType, TopicIdType};

//     #[test]
//     fn test_dup() {
//         let mut flags = Flags::new();
//         flags.set_dup(true);
//         assert_eq!(flags.get(), 0b10000000);
//         flags.set_dup(false);
//         assert_eq!(flags.get(), 0u8);
//     }

//     #[test]
//     fn test_qos() {
//         let mut flags = Flags::new();
//         flags.set_qos(QoSType::AtMostOnce);
//         assert_eq!(flags.get(), 0u8);
//         flags.set_qos(QoSType::AtLeastOnce);
//         assert_eq!(flags.get(), 0b00100000);
//         flags.set_qos(QoSType::ExactlyOnce);
//         assert_eq!(flags.get(), 0b01000000);
//     }

//     #[test]
//     fn test_retain() {
//         let mut flags = Flags::new();
//         flags.set_retain(true);
//         assert_eq!(flags.get(), 0b00010000);
//         flags.set_retain(false);
//         assert_eq!(flags.get(), 0u8);
//     }

//     #[test]
//     fn test_will() {
//         let mut flags = Flags::new();
//         flags.set_will(true);
//         assert_eq!(flags.get(), 0b00001000);
//         flags.set_will(false);
//         assert_eq!(flags.get(), 0u8);
//     }

//     #[test]
//     fn clean_session() {
//         let mut flags = Flags::new();
//         flags.set_clean_session(true);
//         assert_eq!(flags.get(), 0b00000100);
//         flags.set_clean_session(false);
//         assert_eq!(flags.get(), 0u8);
//     }

//     #[test]
//     fn topic_id_type() {
//         let mut flags = Flags::new();
//         flags.set_topic_id_type(TopicIdType::PreDefinedTopic);
//         assert_eq!(flags.get(), 0b00000001);
//         flags.set_topic_id_type(TopicIdType::ShortNameTopic);
//         assert_eq!(flags.get(), 0b00000010);
//         flags.set_topic_id_type(TopicIdType::NormalTopic);
//         assert_eq!(flags.get(), 0u8);
//     }
// }
