pub mod packet;

#[cfg(test)]
mod tests_connect {
    use crate::{
        mqtt::packet::{
            connect::{Will, WillFlags},
            property::{self, PropertyType},
            publish::PublishFlags,
            subscribe::SubFlags,
            PacketType,
        },
        PacketSerializer,
    };

    #[test]
    fn test_connect() {
        let mut props_buf = [0u8; 1024];
        let props_buf_len = property::bundle_properties(
            &mut props_buf,
            &[
                PropertyType::SessionExpiryInterval(1234),
                PropertyType::ReceiveMaximum(432),
                PropertyType::MaximumPacketSize(100),
                PropertyType::TopicAliasMaximum(456),
                PropertyType::RequestResponseInformation(true),
                PropertyType::RequestProblemInformation(true),
                PropertyType::UserProperties(&[("test".as_bytes(), "test".as_bytes())]),
                PropertyType::AuthenticationMethod("test".as_bytes()),
                PropertyType::AuthenticationData(&[1, 2, 3, 4]),
            ],
        );

        let mut will_props_buf = [0u8; 1024];
        let will_props_buf_len = property::bundle_properties(
            &mut will_props_buf,
            &[
                PropertyType::WillDelayInterval(1234),
                PropertyType::PayloadFormatIndicator(false),
                PropertyType::MessageExpiryInterval(4321),
                PropertyType::ContentType("test".as_bytes()),
                PropertyType::ResponseTopic("topic".as_bytes()),
                PropertyType::CorrelationData(&[1, 2, 3, 4]),
                PropertyType::UserProperties(&[("test".as_bytes(), "test".as_bytes())]),
            ],
        );

        let topic = "topic".as_bytes();
        let payload = &[4, 3, 2, 1];
        let will = Will {
            flags: WillFlags::WillRetain.value() | WillFlags::WillQos2.value(),
            properties: Some(&will_props_buf[..will_props_buf_len]),
            topic: Some(topic),
            payload: Some(payload),
        };

        let client_id = "test".as_bytes();
        let msg = super::packet::PacketType::Connect {
            clean_start: true,
            keep_alive: 30,
            properties: Some(&props_buf[..props_buf_len]),
            client_id: Some(client_id),
            will: Some(will),
            username: None,
            password: None,
        };
        let mut buf = [0u8; 1024];
        let buf_len = msg.serialize(&mut buf);
        let expected = [
            16, 125, // Header
            0, 4, // Protocol ID length
            77, 81, 84, 84, // Protocol ID
            5,  // Protocol version
            54, // Connect flags
            0, 30, // Keepalive
            47, // properties length
            17, 0, 0, 4, 210, // sessionExpiryInterval
            33, 1, 176, // receiveMaximum
            39, 0, 0, 0, 100, // maximumPacketSize
            34, 1, 200, // topicAliasMaximum
            25, 1, // requestResponseInformation
            23, 1, // requestProblemInformation,
            38, 0, 4, 116, 101, 115, 116, 0, 4, 116, 101, 115, 116, // userProperties,
            21, 0, 4, 116, 101, 115, 116, // authenticationMethod
            22, 0, 4, 1, 2, 3, 4, // authenticationData
            0, 4, // Client ID length
            116, 101, 115, 116, // Client ID
            47,  // will properties
            24, 0, 0, 4, 210, // will delay interval
            1, 0, // payload format indicator
            2, 0, 0, 16, 225, // message expiry interval
            3, 0, 4, 116, 101, 115, 116, // content type
            8, 0, 5, 116, 111, 112, 105, 99, // response topic
            9, 0, 4, 1, 2, 3, 4, // corelation data
            38, 0, 4, 116, 101, 115, 116, 0, 4, 116, 101, 115, 116, // user properties
            0, 5, // Will topic length
            116, 111, 112, 105, 99, // Will topic
            0, 4, // Will payload length
            4, 3, 2, 1, // Will payload
        ];
        assert_eq!(&buf[..buf_len], &expected);
        let value_from_byte_array = PacketType::deserialize(&buf[..buf_len]);
        let mut buf = [0u8; 1024];
        let buf_len = value_from_byte_array.serialize(&mut buf);
        assert_eq!(&buf[..buf_len], &expected)
    }

    #[test]
    fn test_connect_with_empty_payload() {
        let data = &[1, 2, 3, 4];
        let mut props_buf = [0u8; 1024];
        let props_buf_len = property::bundle_properties(
            &mut props_buf,
            &[
                PropertyType::SessionExpiryInterval(1234),
                PropertyType::ReceiveMaximum(432),
                PropertyType::MaximumPacketSize(100),
                PropertyType::TopicAliasMaximum(456),
                PropertyType::RequestResponseInformation(true),
                PropertyType::RequestProblemInformation(true),
                PropertyType::UserProperties(&[("test".as_bytes(), "test".as_bytes())]),
                PropertyType::AuthenticationMethod("test".as_bytes()),
                PropertyType::AuthenticationData(data),
            ],
        );

        let corr_data = &[1, 2, 3, 4];
        let mut will_props_buf = [0u8; 1024];
        let will_props_buf_len = property::bundle_properties(
            &mut will_props_buf,
            &[
                PropertyType::WillDelayInterval(1234),
                PropertyType::PayloadFormatIndicator(false),
                PropertyType::MessageExpiryInterval(4321),
                PropertyType::ContentType("test".as_bytes()),
                PropertyType::ResponseTopic("topic".as_bytes()),
                PropertyType::CorrelationData(corr_data),
                PropertyType::UserProperties(&[("test".as_bytes(), "test".as_bytes())]),
            ],
        );

        let topic = "topic".as_bytes();
        let will = Will {
            flags: WillFlags::WillRetain.value() | WillFlags::WillQos2.value(),
            properties: Some(&will_props_buf[..will_props_buf_len]),
            topic: Some(topic),
            payload: None,
        };

        let client_id = "test".as_bytes();
        let msg = super::packet::PacketType::Connect {
            clean_start: true,
            keep_alive: 30,
            properties: Some(&props_buf[..props_buf_len]),
            client_id: Some(client_id),
            will: Some(will),
            username: None,
            password: None,
        };
        let mut buf = [0u8; 1024];
        let buf_len = msg.serialize(&mut buf);
        let expected = [
            16, 121, // Header
            0, 4, // Protocol ID length
            77, 81, 84, 84, // Protocol ID
            5,  // Protocol version
            54, // Connect flags
            0, 30, // Keepalive
            47, // properties length
            17, 0, 0, 4, 210, // sessionExpiryInterval
            33, 1, 176, // receiveMaximum
            39, 0, 0, 0, 100, // maximumPacketSize
            34, 1, 200, // topicAliasMaximum
            25, 1, // requestResponseInformation
            23, 1, // requestProblemInformation,
            38, 0, 4, 116, 101, 115, 116, 0, 4, 116, 101, 115, 116, // userProperties,
            21, 0, 4, 116, 101, 115, 116, // authenticationMethod
            22, 0, 4, 1, 2, 3, 4, // authenticationData
            0, 4, // Client ID length
            116, 101, 115, 116, // Client ID
            47,  // will properties
            24, 0, 0, 4, 210, // will delay interval
            1, 0, // payload format indicator
            2, 0, 0, 16, 225, // message expiry interval
            3, 0, 4, 116, 101, 115, 116, // content type
            8, 0, 5, 116, 111, 112, 105, 99, // response topic
            9, 0, 4, 1, 2, 3, 4, // corelation data
            38, 0, 4, 116, 101, 115, 116, 0, 4, 116, 101, 115, 116, // user properties
            0, 5, // Will topic length
            116, 111, 112, 105, 99, // Will topic
            0, 0, // Will payload length
        ];

        assert_eq!(&buf[..buf_len], &expected);
        let value_from_byte_array = PacketType::deserialize(&buf[..buf_len]);
        let mut buf = [0u8; 1024];
        let buf_len = value_from_byte_array.serialize(&mut buf);
        assert_eq!(&buf[..buf_len], &expected)
    }

    #[test]
    fn connect_without_will_properties() {
        let data = &[1, 2, 3, 4];
        let mut props_buf = [0u8; 1024];
        let props_buf_len = property::bundle_properties(
            &mut props_buf,
            &[
                PropertyType::SessionExpiryInterval(1234),
                PropertyType::ReceiveMaximum(432),
                PropertyType::MaximumPacketSize(100),
                PropertyType::TopicAliasMaximum(456),
                PropertyType::RequestResponseInformation(true),
                PropertyType::RequestProblemInformation(true),
                PropertyType::UserProperties(&[("test".as_bytes(), "test".as_bytes())]),
                PropertyType::AuthenticationMethod("test".as_bytes()),
                PropertyType::AuthenticationData(data),
            ],
        );

        // let mut will_props_buf = [0u8; 1024];
        let topic = "topic".as_bytes();
        let payload = &[4, 3, 2, 1];

        let will = Will {
            flags: WillFlags::WillRetain.value() | WillFlags::WillQos2.value(),
            properties: None,
            topic: Some(topic),
            payload: Some(payload),
        };

        let client_id = "test".as_bytes();
        let msg = super::packet::PacketType::Connect {
            clean_start: true,
            keep_alive: 30,
            properties: Some(&props_buf[..props_buf_len]),
            client_id: Some(client_id),
            will: Some(will),
            username: None,
            password: None,
        };
        let mut buf = [0u8; 1024];
        let buf_len = msg.serialize(&mut buf);
        let expected = [
            16, 78, // Header
            0, 4, // Protocol ID length
            77, 81, 84, 84, // Protocol ID
            5,  // Protocol version
            54, // Connect flags
            0, 30, // Keepalive
            47, // properties length
            17, 0, 0, 4, 210, // sessionExpiryInterval
            33, 1, 176, // receiveMaximum
            39, 0, 0, 0, 100, // maximumPacketSize
            34, 1, 200, // topicAliasMaximum
            25, 1, // requestResponseInformation
            23, 1, // requestProblemInformation,
            38, 0, 4, 116, 101, 115, 116, 0, 4, 116, 101, 115, 116, // userProperties,
            21, 0, 4, 116, 101, 115, 116, // authenticationMethod
            22, 0, 4, 1, 2, 3, 4, // authenticationData
            0, 4, // Client ID length
            116, 101, 115, 116, // Client ID
            0,   // will properties
            0, 5, // Will topic length
            116, 111, 112, 105, 99, // Will topic
            0, 4, // Will payload length
            4, 3, 2, 1, // Will payload
        ];
        let value_from_byte_array = PacketType::deserialize(&buf[..buf_len]);
        assert_eq!(&buf[..buf_len], &expected);
        let mut buf = [0u8; 1024];
        let buf_len = value_from_byte_array.serialize(&mut buf);
        assert_eq!(&buf[..buf_len], &expected)
    }

    #[test]
    fn connect_with_client_utf_8() {
        let client_id = "Ŧėśt🜄".as_bytes();
        let msg = super::packet::PacketType::Connect {
            clean_start: true,
            keep_alive: 30,
            properties: None,
            client_id: Some(client_id),
            will: None,
            username: None,
            password: None,
        };
        let mut buf = [0u8; 1024];
        let buf_len = msg.serialize(&mut buf);
        let expected = [
            16, 24, // Header
            0, 4, // Protocol ID length
            77, 81, 84, 84, // Protocol ID
            5,  // Protocol version
            2,  // Connect flags
            0, 30, // Keepalive
            0,  //properties length
            0, 11, // Client ID length
            197, 166, // Ŧ (UTF-8: 0xc5a6)
            196, 151, // ė (UTF-8: 0xc497)
            197, 155, // ś (utf-8: 0xc59b)
            116, // t (utf-8: 0x74)
            240, 159, 156, 132, // 🜄 (utf-8: 0xf09f9c84)
        ];
        let value_from_byte_array = PacketType::deserialize(&buf[..buf_len]);
        assert_eq!(&buf[..buf_len], &expected);
        let mut buf = [0u8; 1024];
        let buf_len = value_from_byte_array.serialize(&mut buf);
        assert_eq!(&buf[..buf_len], &expected)
    }

    #[test]
    fn test_auth() {
        let mut props_buf = [0u8; 1024];
        let props_buf_len = property::bundle_properties(
            &mut props_buf,
            &[
                PropertyType::AuthenticationMethod("test".as_bytes()),
                PropertyType::AuthenticationData(&[0, 1, 2, 3]),
                PropertyType::ReasonString("test".as_bytes()),
                PropertyType::UserProperties(&[("test".as_bytes(), "test".as_bytes())]),
            ],
        );
        let msg = PacketType::Auth {
            reason_code: 0,
            properties: Some(&props_buf[..props_buf_len]),
        };

        let mut buf = [0u8; 1024];
        let buf_len = msg.serialize(&mut buf);
        let expected = [
            240, 36, // Header
            0,  // reason code
            34, // properties length
            21, 0, 4, 116, 101, 115, 116, // auth method
            22, 0, 4, 0, 1, 2, 3, // auth data
            31, 0, 4, 116, 101, 115, 116, // reasonString
            38, 0, 4, 116, 101, 115, 116, 0, 4, 116, 101, 115, 116, // userProperties
        ];
        let value_from_byte_array = PacketType::deserialize(&buf[..buf_len]);
        assert_eq!(&buf[..buf_len], &expected);
        let mut buf = [0u8; 1024];
        let buf_len = value_from_byte_array.serialize(&mut buf);
        assert_eq!(&buf[..buf_len], &expected)
    }

    #[test]
    fn test_connack() {
        let msg = PacketType::Connack {
            connect_ack_flags: 0,
            reason_code: 140,
            properties: None,
        };
        let mut buf = [0u8; 1024];
        let buf_len = msg.serialize(&mut buf);
        let expected = [
            32, 3, // Fixed Header (CONNACK, Remaining Length)
            0, 140, // Variable Header (Session not present, Bad authentication method)
            0,   // Property Length Zero
        ];
        let value_from_byte_array = PacketType::deserialize(&buf[..buf_len]);
        assert_eq!(&buf[..buf_len], &expected);
        let mut buf = [0u8; 1024];
        let buf_len = value_from_byte_array.serialize(&mut buf);
        assert_eq!(&buf[..buf_len], &expected)
    }

    #[test]
    fn test_connack_with_properties() {
        let mut props_buf = [0u8; 1024];
        let props_buf_len = property::bundle_properties(
            &mut props_buf,
            &[
                PropertyType::SessionExpiryInterval(1234),
                PropertyType::ReceiveMaximum(432),
                PropertyType::MaximumQOS(2),
                PropertyType::RetainAvailable(true),
                PropertyType::MaximumPacketSize(100),
                PropertyType::AssignedClientIdentifier("test".as_bytes()),
                PropertyType::TopicAliasMaximum(456),
                PropertyType::ReasonString("test".as_bytes()),
                PropertyType::UserProperties(&[("test".as_bytes(), "test".as_bytes())]),
                PropertyType::WildcardSubscriptionAvailable(true),
                PropertyType::SubscriptionIdentifiersAvailable(true),
                PropertyType::SharedSubscriptionAvailable(false),
                PropertyType::ServerKeepAlive(1234),
                PropertyType::ResponseInformation("test".as_bytes()),
                PropertyType::ServerReference("test".as_bytes()),
                PropertyType::AuthenticationMethod("test".as_bytes()),
                PropertyType::AuthenticationData(&[1, 2, 3, 4]),
            ],
        );
        let msg = PacketType::Connack {
            connect_ack_flags: 0,
            reason_code: 0,
            properties: Some(&props_buf[..props_buf_len]),
        };
        let mut buf = [0u8; 1024];
        let buf_len = msg.serialize(&mut buf);
        let expected = [
            32, 87, 0, 0, 84, // properties length
            17, 0, 0, 4, 210, // sessionExpiryInterval
            33, 1, 176, // receiveMaximum
            36, 2, // Maximum qos
            37, 1, // retainAvailable
            39, 0, 0, 0, 100, // maximumPacketSize
            18, 0, 4, 116, 101, 115, 116, // assignedClientIdentifier
            34, 1, 200, // topicAliasMaximum
            31, 0, 4, 116, 101, 115, 116, // reasonString
            38, 0, 4, 116, 101, 115, 116, 0, 4, 116, 101, 115, 116, // userProperties
            40, 1, // wildcardSubscriptionAvailable
            41, 1, // subscriptionIdentifiersAvailable
            42, 0, // sharedSubscriptionAvailable
            19, 4, 210, // serverKeepAlive
            26, 0, 4, 116, 101, 115, 116, // responseInformation
            28, 0, 4, 116, 101, 115, 116, // serverReference
            21, 0, 4, 116, 101, 115, 116, // authenticationMethod
            22, 0, 4, 1, 2, 3, 4, // authenticationData
        ];
        let value_from_byte_array = PacketType::deserialize(&buf[..buf_len]);
        assert_eq!(&buf[..buf_len], &expected);
        let mut buf = [0u8; 1024];
        let buf_len = value_from_byte_array.serialize(&mut buf[..buf_len]);
        assert_eq!(&buf[..buf_len], &expected)
    }

    #[test]
    fn tests_puback() {
        let msg = PacketType::PubAck {
            identifier: 42,
            reason_code: 0,
            properties: None,
        };
        let mut buf = [0u8; 1024];
        let buf_len = msg.serialize(&mut buf);
        let expected = [
            64, 4, // Fixed Header (PUBACK, Remaining Length)
            0, 42,
            0, // Variable Header (2 Bytes: Packet Identifier 42, Reason code: 0 Success)
            0, // no properties
        ];
        let value_from_byte_array = PacketType::deserialize(&buf[..buf_len]);
        assert_eq!(&buf[..buf_len], &expected);
        let mut buf = [0u8; 1024];
        let buf_len = value_from_byte_array.serialize(&mut buf);
        assert_eq!(&buf[..buf_len], &expected)
    }

    #[test]
    fn test_disconnect() {
        let msg = PacketType::Disconnect {
            reason_code: 0,
            properties: None,
        };

        let mut buf = [0u8; 1024];
        let buf_len = msg.serialize(&mut buf);
        let expected = [
            224, 2, // Fixed Header (DISCONNECT, Remaining Length)
            0, // reason Code (Normal disconnection)
            0, // no properties
        ];
        let value_from_byte_array = PacketType::deserialize(&buf[..buf_len]);
        assert_eq!(&buf[..buf_len], &expected);
        let mut buf = [0u8; 1024];
        let buf_len = value_from_byte_array.serialize(&mut buf);
        assert_eq!(&buf[..buf_len], &expected)
    }

    #[test]
    fn tests_publish_no_property() {
        let topic = "test".as_bytes();
        let msg = PacketType::Publish {
            control_flags: 0,
            topic_name: Some(topic),
            identifier: 2,
            properties: None,
            payload: Some(&[116, 101, 115, 116, 32, 100, 97, 116, 97]),
        };
        let mut buf = [0u8; 1024];
        let buf_len = msg.serialize(&mut buf);
        let expected = [
            48, 16, // Header
            0, 4, // Topic length
            116, 101, 115, 116, // Topic (test)
            0,   // properties length
            116, 101, 115, 116, 32, 100, 97, 116, 97, // Payload (test)
        ];
        let value_from_byte_array = PacketType::deserialize(&buf[..buf_len]);
        assert_eq!(&buf[..buf_len], &expected);
        let mut buf = [0u8; 1024];
        let buf_len = value_from_byte_array.serialize(&mut buf);
        assert_eq!(&buf[..buf_len], &expected);
    }
    #[test]
    fn tests_publish() {
        let topic = "test".as_bytes();
        let mut props_buf = [0u8; 1024];
        let props_buf_len = property::bundle_properties(
            &mut props_buf,
            &[
                PropertyType::PayloadFormatIndicator(true),
                PropertyType::MessageExpiryInterval(4321),
                PropertyType::TopicAlias(100),
                PropertyType::ResponseTopic("topic".as_bytes()),
                PropertyType::CorrelationData(&[1, 2, 3, 4]),
                PropertyType::UserProperties(&[
                    ("test".as_bytes(), "test".as_bytes()),
                    ("test".as_bytes(), "test".as_bytes()),
                    ("test".as_bytes(), "test".as_bytes()),
                ]),
                PropertyType::SubscriptionIdentifier(120),
                PropertyType::ContentType("test".as_bytes()),
            ],
        );
        let msg = PacketType::Publish {
            control_flags: PublishFlags::Retain.value()
                | PublishFlags::DUP.value()
                | PublishFlags::QoS2.value(),
            topic_name: Some(topic),
            identifier: 10,
            properties: Some(&props_buf[..props_buf_len]),
            payload: Some(&[116, 101, 115, 116]),
        };
        let mut buf = [0u8; 1024];
        let buf_len = msg.serialize(&mut buf);
        let expected = [
            61, 86, // Header
            0, 4, // Topic length
            116, 101, 115, 116, // Topic (test)
            0, 10, // Message ID
            73, // properties length
            1, 1, // payloadFormatIndicator
            2, 0, 0, 16, 225, // message expiry interval
            35, 0, 100, // topicAlias
            8, 0, 5, 116, 111, 112, 105, 99, // response topic
            9, 0, 4, 1, 2, 3, 4, // correlationData
            38, 0, 4, 116, 101, 115, 116, 0, 4, 116, 101, 115, 116, // userProperties
            38, 0, 4, 116, 101, 115, 116, 0, 4, 116, 101, 115, 116, // userProperties
            38, 0, 4, 116, 101, 115, 116, 0, 4, 116, 101, 115, 116, // userProperties
            11, 120, // subscriptionIdentifier
            3, 0, 4, 116, 101, 115, 116, // content type
            116, 101, 115, 116, // Payload (test)
        ];
        let value_from_byte_array = PacketType::deserialize(&buf[..buf_len]);
        assert_eq!(&buf[..buf_len], &expected);
        let mut buf = [0u8; 1024];
        let buf_len = value_from_byte_array.serialize(&mut buf);
        assert_eq!(&buf[..buf_len], &expected)
    }

    #[test]
    fn tests_publish_multiple_same_properties() {
        let mut props_buf = [0u8; 1024];
        let topic = "test".as_bytes();
        let props_buf_len = property::bundle_properties(
            &mut props_buf,
            &[
                PropertyType::PayloadFormatIndicator(true),
                PropertyType::MessageExpiryInterval(4321),
                PropertyType::TopicAlias(100),
                PropertyType::ResponseTopic("topic".as_bytes()),
                PropertyType::CorrelationData(&[1, 2, 3, 4]),
                PropertyType::UserProperties(&[("test".as_bytes(), "test".as_bytes())]),
                PropertyType::SubscriptionIdentifier(120),
                PropertyType::SubscriptionIdentifier(121),
                PropertyType::SubscriptionIdentifier(122),
                PropertyType::ContentType("test".as_bytes()),
            ],
        );
        let msg = PacketType::Publish {
            control_flags: PublishFlags::Retain.value()
                | PublishFlags::DUP.value()
                | PublishFlags::QoS2.value(),
            topic_name: Some(topic),
            identifier: 10,
            properties: Some(&props_buf[..props_buf_len]),
            payload: Some(&[116, 101, 115, 116]),
        };
        let mut buf = [0u8; 1024];
        let buf_len = msg.serialize(&mut buf);
        let expected = [
            61, 64, // Header
            0, 4, // Topic length
            116, 101, 115, 116, // Topic (test)
            0, 10, // Message ID
            51, // properties length
            1, 1, // payloadFormatIndicator
            2, 0, 0, 16, 225, // message expiry interval
            35, 0, 100, // topicAlias
            8, 0, 5, 116, 111, 112, 105, 99, // response topic
            9, 0, 4, 1, 2, 3, 4, // correlationData
            38, 0, 4, 116, 101, 115, 116, 0, 4, 116, 101, 115, 116, // userProperties
            11, 120, // subscriptionIdentifier
            11, 121, // subscriptionIdentifier
            11, 122, // subscriptionIdentifier
            3, 0, 4, 116, 101, 115, 116, // content type
            116, 101, 115, 116, // Payload (test)
        ];
        let value_from_byte_array = PacketType::deserialize(&buf[..buf_len]);
        assert_eq!(&buf[..buf_len], &expected);
        let mut buf = [0u8; 1024];
        let buf_len = value_from_byte_array.serialize(&mut buf);
        assert_eq!(&buf[..buf_len], &expected)
    }

    #[test]
    fn tests_publish_4byte_varint() {
        let topic = "test".as_bytes();
        let mut props_buf = [0u8; 1024];
        let props_buf_len = property::bundle_properties(
            &mut props_buf,
            &[
                PropertyType::PayloadFormatIndicator(false),
                PropertyType::SubscriptionIdentifier(128),
                PropertyType::SubscriptionIdentifier(16384),
                PropertyType::SubscriptionIdentifier(2097152),
            ],
        );
        let msg = PacketType::Publish {
            control_flags: PublishFlags::Retain.value()
                | PublishFlags::DUP.value()
                | PublishFlags::QoS2.value(),
            topic_name: Some(topic),
            identifier: 10,
            properties: Some(&props_buf[..props_buf_len]),
            payload: Some(&[116, 101, 115, 116]),
        };
        let mut buf = [0u8; 1024];
        let buf_len = msg.serialize(&mut buf);
        let expected = [
            61, 27, // Header
            0, 4, // Topic length
            116, 101, 115, 116, // Topic (test)
            0, 10, // Message ID
            14, // properties length
            1, 0, // payloadFormatIndicator
            11, 128, 1, // subscriptionIdentifier
            11, 128, 128, 1, // subscriptionIdentifier
            11, 128, 128, 128, 1, // subscriptionIdentifier
            116, 101, 115, 116, // Payload (test)
        ];
        let value_from_byte_array = PacketType::deserialize(&buf[..buf_len]);
        assert_eq!(&buf[..buf_len], &expected);
        let mut buf = [0u8; 1024];
        let buf_len = value_from_byte_array.serialize(&mut buf);
        assert_eq!(&buf[..buf_len], &expected)
    }

    #[test]
    fn tests_publish_property_with_max_varint() {
        let topic = "test".as_bytes();
        let mut props_buf = [0u8; 1024];
        let props_buf_len = property::bundle_properties(
            &mut props_buf,
            &[
                PropertyType::PayloadFormatIndicator(false),
                PropertyType::SubscriptionIdentifier(1),
                PropertyType::SubscriptionIdentifier(268435455),
            ],
        );
        let msg = PacketType::Publish {
            control_flags: PublishFlags::Retain.value()
                | PublishFlags::DUP.value()
                | PublishFlags::QoS2.value(),
            topic_name: Some(topic),
            identifier: 10,
            properties: Some(&props_buf[..props_buf_len]),
            payload: Some(&[116, 101, 115, 116]),
        };
        let mut buf = [0u8; 1024];
        let buf_len = msg.serialize(&mut buf);
        let expected = [
            61, 22, // Header
            0, 4, // Topic length
            116, 101, 115, 116, // Topic (test)
            0, 10, // Message ID
            9,  // properties length
            1, 0, // payloadFormatIndicator
            11, 1, // subscriptionIdentifier
            11, 255, 255, 255, 127, // subscriptionIdentifier (max value)
            116, 101, 115, 116, // Payload (test)
        ];
        let value_from_byte_array = PacketType::deserialize(&buf[..buf_len]);
        assert_eq!(&buf[..buf_len], &expected);
        let mut buf = [0u8; 1024];
        let buf_len = value_from_byte_array.serialize(&mut buf);
        assert_eq!(&buf[..buf_len], &expected)
    }

    #[test]
    fn test_subscribe() {
        let mut props_buf = [0u8; 1024];
        let props_buf_len = property::bundle_properties(
            &mut props_buf,
            &[
                PropertyType::SubscriptionIdentifier(145),
                PropertyType::UserProperties(&[("test".as_bytes(), "test".as_bytes())]),
            ],
        );
        let mut filters_buf = [0u8; 1024];
        let filters_buf_len = property::bundle_sub_topic_filters(
            &mut filters_buf,
            &[(
                "test".as_bytes(),
                SubFlags::RetainAsPlublished.value() | SubFlags::SendRetainedIfNotExists.value(),
            )],
        );
        let msg = PacketType::Subscribe {
            identifier: 6,
            properties: Some(&props_buf[..props_buf_len]),
            topic_filters: Some(&filters_buf[..filters_buf_len]),
        };
        let mut buf = [0u8; 1024];
        let buf_len = msg.serialize(&mut buf);
        let expected = [
            130, 26, // Header (subscribeqos=1length=9)
            0, 6,  // Message ID (6)
            16, // properties length
            11, 145, 1, // subscriptionIdentifier
            38, 0, 4, 116, 101, 115, 116, 0, 4, 116, 101, 115, 116, // userProperties
            0, 4, // Topic length,
            116, 101, 115, 116, // Topic (test)
            24,  // settings(qos: 0, noLocal: false, Retain as Published: true, retain handling: 1)
        ];
        let value_from_byte_array = PacketType::deserialize(&buf[..buf_len]);
        assert_eq!(&buf[..buf_len], &expected);
        let mut buf = [0u8; 1024];
        let buf_len = value_from_byte_array.serialize(&mut buf);
        assert_eq!(&buf[..buf_len], &expected)
    }

    #[test]
    fn test_subscribe_to_several_topics() {
        let mut props_buf = [0u8; 1024];
        let props_buf_len = property::bundle_properties(
            &mut props_buf,
            &[
                PropertyType::SubscriptionIdentifier(145),
                PropertyType::UserProperties(&[("test".as_bytes(), "test".as_bytes())]),
            ],
        );
        let mut filters_buf = [0u8; 1024];
        let filters_buf_len = property::bundle_sub_topic_filters(
            &mut filters_buf,
            &[
                (
                    "test".as_bytes(),
                    SubFlags::RetainAsPlublished.value()
                        | SubFlags::SendRetainedIfNotExists.value(),
                ),
                ("uest".as_bytes(), SubFlags::QoS1.value()),
                (
                    "tfst".as_bytes(),
                    SubFlags::QoS2.value() | SubFlags::NoLocal.value(),
                ),
            ],
        );
        let msg = PacketType::Subscribe {
            identifier: 6,
            properties: Some(&props_buf[..props_buf_len]),
            topic_filters: Some(&filters_buf[..filters_buf_len]),
        };
        let mut buf = [0u8; 1024];
        let buf_len = msg.serialize(&mut buf);
        let expected = [
            130, 40, // Header (subscribeqos=1length=9)
            0, 6,  // Message ID (6)
            16, // properties length
            11, 145, 1, // subscriptionIdentifier
            38, 0, 4, 116, 101, 115, 116, 0, 4, 116, 101, 115, 116, // userProperties
            0, 4, // Topic length,
            116, 101, 115, 116, // Topic (test)
            24,  // settings(qos: 0, noLocal: false, Retain as Published: true, retain handling: 1)
            0, 4, // Topic length
            117, 101, 115, 116, // Topic (uest)
            1,   // Qos (1)
            0, 4, // Topic length
            116, 102, 115, 116, // Topic (tfst)
            6,   // Qos (2), No Local: true
        ];
        let value_from_byte_array = PacketType::deserialize(&buf[..buf_len]);
        assert_eq!(&buf[..buf_len], &expected);
        let mut buf = [0u8; 1024];
        let buf_len = value_from_byte_array.serialize(&mut buf);
        assert_eq!(&buf[..buf_len], &expected)
    }

    #[test]
    fn test_pubrec() {
        let mut props_buf = [0u8; 1024];
        let props_buf_len = property::bundle_properties(
            &mut props_buf,
            &[
                PropertyType::ReasonString("test".as_bytes()),
                PropertyType::UserProperties(&[("test".as_bytes(), "test".as_bytes())]),
            ],
        );
        let msg = PacketType::PubRec {
            identifier: 2,
            reason_code: 16,
            properties: Some(&props_buf[..props_buf_len]),
        };
        let mut buf = [0u8; 1024];
        let buf_len = msg.serialize(&mut buf);
        let expected = [
            80, 24, // Header
            0, 2,  // Message ID
            16, // reason code
            20, // properties length
            31, 0, 4, 116, 101, 115, 116, // reasonString
            38, 0, 4, 116, 101, 115, 116, 0, 4, 116, 101, 115, 116, // userProperties
        ];
        let value_from_byte_array = PacketType::deserialize(&buf[..buf_len]);
        assert_eq!(&buf[..buf_len], &expected);
        let mut buf = [0u8; 1024];
        let buf_len = value_from_byte_array.serialize(&mut buf);
        assert_eq!(&buf[..buf_len], &expected)
    }

    #[test]
    fn test_pubrel() {
        let mut props_buf = [0u8; 1024];
        let props_buf_len = property::bundle_properties(
            &mut props_buf,
            &[
                PropertyType::ReasonString("test".as_bytes()),
                PropertyType::UserProperties(&[("test".as_bytes(), "test".as_bytes())]),
            ],
        );
        let msg = PacketType::PubRel {
            identifier: 2,
            reason_code: 0x92,
            properties: Some(&props_buf[..props_buf_len]),
        };
        let mut buf = [0u8; 1024];
        let buf_len = msg.serialize(&mut buf);
        let expected = [
            98, 24, // Header
            0, 2,    // Message ID
            0x92, // reason code
            20,   // properties length
            31, 0, 4, 116, 101, 115, 116, // reasonString
            38, 0, 4, 116, 101, 115, 116, 0, 4, 116, 101, 115, 116, // userProperties
        ];
        let value_from_byte_array = PacketType::deserialize(&buf[..buf_len]);
        assert_eq!(&buf[..buf_len], &expected);
        let mut buf = [0u8; 1024];
        let buf_len = value_from_byte_array.serialize(&mut buf);
        assert_eq!(&buf[..buf_len], &expected)
    }

    #[test]
    fn test_pubcomp() {
        let mut props_buf = [0u8; 1024];
        let props_buf_len = property::bundle_properties(
            &mut props_buf,
            &[
                PropertyType::ReasonString("test".as_bytes()),
                PropertyType::UserProperties(&[("test".as_bytes(), "test".as_bytes())]),
            ],
        );
        let msg = PacketType::PubComp {
            identifier: 2,
            reason_code: 0x92,
            properties: Some(&props_buf[..props_buf_len]),
        };
        let mut buf = [0u8; 1024];
        let buf_len = msg.serialize(&mut buf);
        let expected = [
            112, 24, // Header
            0, 2,    // Message ID
            0x92, // reason code
            20,   // properties length
            31, 0, 4, 116, 101, 115, 116, // reasonString
            38, 0, 4, 116, 101, 115, 116, 0, 4, 116, 101, 115, 116, // userProperties
        ];
        let value_from_byte_array = PacketType::deserialize(&buf[..buf_len]);
        assert_eq!(&buf[..buf_len], &expected);
        let mut buf = [0u8; 1024];
        let buf_len = value_from_byte_array.serialize(&mut buf);
        assert_eq!(&buf[..buf_len], &expected)
    }

    #[test]
    fn test_suback() {
        let mut props_buf = [0u8; 1024];
        let props_buf_len = property::bundle_properties(
            &mut props_buf,
            &[
                PropertyType::ReasonString("test".as_bytes()),
                PropertyType::UserProperties(&[("test".as_bytes(), "test".as_bytes())]),
            ],
        );
        let msg = PacketType::SubAck {
            identifier: 6,
            properties: Some(&props_buf[..props_buf_len]),
            reason_codes: &[0, 1, 2, 128],
        };
        let mut buf = [0u8; 1024];
        let buf_len = msg.serialize(&mut buf);
        let expected = [
            144, 27, // Header
            0, 6,  // Message ID
            20, // properties length
            31, 0, 4, 116, 101, 115, 116, // reasonString
            38, 0, 4, 116, 101, 115, 116, 0, 4, 116, 101, 115, 116, // userProperties
            0, 1, 2, 128, // Granted qos (0, 1, 2) and a rejected being 0x80
        ];
        let value_from_byte_array = PacketType::deserialize(&buf[..buf_len]);
        assert_eq!(&buf[..buf_len], &expected);
        let mut buf = [0u8; 1024];
        let buf_len = value_from_byte_array.serialize(&mut buf);
        assert_eq!(&buf[..buf_len], &expected)
    }

    #[test]
    fn test_unsubscribe() {
        let mut props_buf = [0u8; 1024];
        let props_buf_len = property::bundle_properties(
            &mut props_buf,
            &[PropertyType::UserProperties(&[(
                "test".as_bytes(),
                "test".as_bytes(),
            )])],
        );
        let mut filters_buf = [0u8; 1024];
        let filters_buf_len = property::bundle_unsub_topic_filters(
            &mut filters_buf,
            &["tfst".as_bytes(), "test".as_bytes()],
        );
        let msg = PacketType::Unsubscribe {
            identifier: 7,
            properties: Some(&props_buf[..props_buf_len]),
            topic_filters: Some(&filters_buf[..filters_buf_len]),
        };
        let mut buf = [0u8; 1024];
        let buf_len = msg.serialize(&mut buf);
        let expected = [
            162, 28, 0, 7,  // Message ID (7)
            13, // properties length
            38, 0, 4, 116, 101, 115, 116, 0, 4, 116, 101, 115, 116, // userProperties
            0, 4, // Topic length
            116, 102, 115, 116, // Topic (tfst)
            0, 4, // Topic length,
            116, 101, 115, 116, // Topic (test)
        ];
        let value_from_byte_array = PacketType::deserialize(&buf[..buf_len]);
        assert_eq!(&buf[..buf_len], &expected);
        let mut buf = [0u8; 1024];
        let buf_len = value_from_byte_array.serialize(&mut buf);
        assert_eq!(&buf[..buf_len], &expected)
    }

    #[test]
    fn test_unsuback() {
        let mut props_buf = [0u8; 1024];
        let props_buf_len = property::bundle_properties(
            &mut props_buf,
            &[
                PropertyType::ReasonString("test".as_bytes()),
                PropertyType::UserProperties(&[("test".as_bytes(), "test".as_bytes())]),
            ],
        );
        let msg = PacketType::UnsubAck {
            identifier: 8,
            properties: Some(&props_buf[..props_buf_len]),
            reason_codes: &[0, 128],
        };
        let mut buf = [0u8; 1024];
        let buf_len = msg.serialize(&mut buf);
        let expected = [
            176, 25, // Header
            0, 8,  // Message ID
            20, // properties length
            31, 0, 4, 116, 101, 115, 116, // reasonString
            38, 0, 4, 116, 101, 115, 116, 0, 4, 116, 101, 115, 116, // userProperties
            0, 128, // success and error
        ];
        let value_from_byte_array = PacketType::deserialize(&buf[..buf_len]);
        assert_eq!(&buf[..buf_len], &expected);
        let mut buf = [0u8; 1024];
        let buf_len = value_from_byte_array.serialize(&mut buf);
        assert_eq!(&buf[..buf_len], &expected)
    }

    #[test]
    fn test_pingreq() {
        let msg = PacketType::PingReq;
        let mut buf = [0u8; 1024];
        let buf_len = msg.serialize(&mut buf);
        let expected = [192, 0];
        let value_from_byte_array = PacketType::deserialize(&buf[..buf_len]);
        assert_eq!(&buf[..buf_len], &expected);
        let mut buf = [0u8; 1024];
        let buf_len = value_from_byte_array.serialize(&mut buf);
        assert_eq!(&buf[..buf_len], &expected)
    }

    #[test]
    fn test_pingresp() {
        let msg = PacketType::PingResp;
        let mut buf = [0u8; 1024];
        let buf_len = msg.serialize(&mut buf);
        let expected = [208, 0];
        let value_from_byte_array = PacketType::deserialize(&buf[..buf_len]);
        assert_eq!(&buf[..buf_len], &expected);
        let mut buf = [0u8; 1024];
        let buf_len = value_from_byte_array.serialize(&mut buf);
        assert_eq!(&buf[..buf_len], &expected)
    }

    // #[test]
    // fn test_varint() {
    //     let val = 127;
    //     let arr = varint::encode(val).unwrap();
    //     assert_eq!(arr, [0x7F]);
    //     let (val, size) = varint::decode(&arr).unwrap();
    //     assert_eq!((val, size), (127, 1));
    //     let val = 128;
    //     let arr = varint::encode(val).unwrap();
    //     assert_eq!(arr, [0x80, 0x01]);
    //     let (val, size) = varint::decode(&arr).unwrap();
    //     assert_eq!((val, size), (128, 2));
    //     let val = 16383;
    //     let arr = varint::encode(val).unwrap();
    //     assert_eq!(arr, [0xFF, 0x7F]);
    //     let (val, size) = varint::decode(&arr).unwrap();
    //     assert_eq!((val, size), (16383, 2));
    //     let val = 16384;
    //     let arr = varint::encode(val).unwrap();
    //     assert_eq!(arr, [0x80, 0x80, 0x01]);
    //     let (val, size) = varint::decode(&arr).unwrap();
    //     assert_eq!((val, size), (16384, 3));
    //     let val = 2097151;
    //     let arr = varint::encode(val).unwrap();
    //     assert_eq!(arr, [0xFF, 0xFF, 0x7F]);
    //     let (val, size) = varint::decode(&arr).unwrap();
    //     assert_eq!((val, size), (2097151, 3));
    //     let val = 2097152;
    //     let arr = varint::encode(val).unwrap();
    //     assert_eq!(arr, [0x80, 0x80, 0x80, 0x01]);
    //     let (val, size) = varint::decode(&arr).unwrap();
    //     assert_eq!((val, size), (2097152, 4));
    //     let val = 97152;
    //     let arr = varint::encode(val).unwrap();
    //     assert_eq!(arr, [0x80, 0xF7, 0x05]);
    //     let (val, size) = varint::decode(&arr).unwrap();
    //     assert_eq!((val, size), (97152, 3));
    // }
}
