MQTT-5, MQTT-SN and Zigbee packet implementation

Example MQTT-5: 

```rust
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
```

Example MQTT-SN: 
```rust
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
```