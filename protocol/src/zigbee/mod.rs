// use crate::PacketSerializer;
//
// pub mod packet;
//
// #[test]
// fn test_at_command() {
//     let msg = packet::PacketSendType::LocalAtCommand {
//         frame_id: 0xA1,
//         at_command: Some([0x4E, 0x49]),
//         parameter: Some(&[0x45, 0x6E, 0x64, 0x20, 0x44, 0x65, 0x76, 0x69, 0x63, 0x65]),
//     };
//     let msg = msg.serialize();
//     assert_eq!(
//         &msg,
//         &[
//             0x7E, 0x00, 0x0E, 0x08, 0xA1, 0x4E, 0x49, 0x45, 0x6E, 0x64, 0x20, 0x44, 0x65, 0x76,
//             0x69, 0x63, 0x65, 0x38
//         ]
//     );
//
//     let msg = packet::PacketSendType::LocalAtCommand {
//         frame_id: 0x17,
//         at_command: Some([0x54, 0x50]),
//         parameter: None,
//     };
//     let msg = msg.serialize();
//     assert_eq!(msg, [0x7E, 0x00, 0x04, 0x08, 0x17, 0x54, 0x50, 0x3C])
// }
// #[test]
// fn test_at_command_response() {
//     let mut res: RecvPacket = RecvPacket::new();
//     res.set_next_byte(0x7E);
//     res.set_next_byte(0x00);
//     res.set_next_byte(0x05);
//     res.set_next_byte(0x88);
//     res.set_next_byte(0x01);
//     res.set_next_byte(0x4E);
//     res.set_next_byte(0x49);
//     res.set_next_byte(0x00);
//     res.set_next_byte(0xDF);
//     let msg: AtCommandResponse = AtCommandResponse::new(res);
//     assert_eq!(msg.get_frame_type(), 0x88);
//     assert_eq!(msg.get_frame_id(), 0x01);
//     assert_eq!(msg.get_at_command(), [0x4E, 0x49]);
//     assert_eq!(msg.get_status(), AtCommandResponseError::Ok);
// }
// #[test]
// fn test_transmit_request() {
//     let msg = packet::PacketSendType::TransmitRequest {
//         frame_id: 0x52,
//         destination_addr64: Some([0x00, 0x13, 0xA2, 0x00, 0x12, 0x34, 0x56, 0x78]),
//         destination_addr16: Some([0xFF, 0xFE]),
//         radius: 0x00,
//         options: 0x00,
//         payload: Some(&[0x54, 0x78, 0x44, 0x61, 0x74, 0x61]),
//     };
//     let mut data = [0u8; 256];
//     let msg = msg.serialize(&mut data);
//     assert_eq!(
//         &data[..msg],
//         [
//             0x7E,
//             0x00,
//             0x14,
//             0x10,
//             0x52,
//             0x00,
//             0x7D,
//             0x13 ^ 0x20,
//             0xA2,
//             0x00,
//             0x12,
//             0x34,
//             0x56,
//             0x78,
//             0xFF,
//             0xFE,
//             0x00,
//             0x00,
//             0x54,
//             0x78,
//             0x44,
//             0x61,
//             0x74,
//             0x61,
//             0x91
//         ]
//     );
//     let msg = packet::PacketSendType::TransmitRequest {
//         frame_id: 0x00,
//         destination_addr64: Some([0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xFF, 0xFF]),
//         destination_addr16: Some([0xFF, 0xFE]),
//         radius: 0x01,
//         options: 0x00,
//         payload: Some(&[0x42, 0x72, 0x6F, 0x61, 0x64, 0x63, 0x61, 0x73, 0x74]),
//     };
//     let mut data = [0u8; 256];
//     let msg = msg.serialize(&mut data);
//     assert_eq!(
//         &data[..msg],
//         [
//             0x7E, 0x00, 0x17, 0x10, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xFF, 0xFF, 0xFF,
//             0xFE, 0x01, 0x00, 0x42, 0x72, 0x6F, 0x61, 0x64, 0x63, 0x61, 0x73, 0x74, 0x60
//         ]
//     )
// }

// #[test]
// fn test_modem_status() {
//     let mut msg: RecvPacket = RecvPacket::new();
//     msg.set_next_byte(0x7E);
//     msg.set_next_byte(0x00);
//     msg.set_next_byte(0x02);
//     msg.set_next_byte(0x8A);
//     msg.set_next_byte(0x00);
//     msg.set_next_byte(0x75);
//     let msg = ModemStatusResponse::new(msg);
//     assert_eq!(msg.get_frame_type(), 0x8A);
//     assert_eq!(msg.get_status(), ModemStatus::HardwareStart);
// }
// #[test]
// fn test_extended_transmission_status() {
//     let mut msg: RecvPacket = RecvPacket::new();
//     msg.set_next_byte(0x7E);
//     msg.set_next_byte(0x00);
//     msg.set_next_byte(0x07);
//     msg.set_next_byte(0x8B);
//     msg.set_next_byte(0x52);
//     msg.set_next_byte(0x12);
//     msg.set_next_byte(0x34);
//     msg.set_next_byte(0x02);
//     msg.set_next_byte(0x00);
//     msg.set_next_byte(0x01);
//     msg.set_next_byte(0xD9);
//     let msg = TransmissionStatusResponse::new(msg);
//     assert_eq!(msg.get_frame_type(), 0x8B);
//     assert_eq!(msg.get_frame_id(), 0x52);
//     assert_eq!(msg.get_destination_address(), [0x12, 0x34]);
//     assert_eq!(msg.get_transmission_count(), 0x02);
//     assert_eq!(msg.get_status(), TransmissionStatus::Success);
//     assert_eq!(
//         msg.get_discovery_status(),
//         DiscoveryStatus::ZigbeeAddressDiscovery
//     );
// }
// #[test]
// fn test_receive_packet() {
//     let mut msg: RecvPacket = RecvPacket::new();
//     msg.set_next_byte(0x7E);
//     msg.set_next_byte(0x00);
//     msg.set_next_byte(0x12);
//     msg.set_next_byte(0x90);
//     msg.set_next_byte(0x00);
//     msg.set_next_byte(0x7D);
//     msg.set_next_byte(0x13 ^ 0x20);
//     msg.set_next_byte(0xA2);
//     msg.set_next_byte(0x00);
//     msg.set_next_byte(0x87);
//     msg.set_next_byte(0x65);
//     msg.set_next_byte(0x43);
//     msg.set_next_byte(0x21);
//     msg.set_next_byte(0x56);
//     msg.set_next_byte(0x14);
//     msg.set_next_byte(0x01);
//     msg.set_next_byte(0x54);
//     msg.set_next_byte(0x78);
//     msg.set_next_byte(0x44);
//     msg.set_next_byte(0x61);
//     msg.set_next_byte(0x74);
//     msg.set_next_byte(0x61);
//     msg.set_next_byte(0xB9);
//     let msg: TranmissionReceive = TranmissionReceive::new(msg);
//     assert_eq!(msg.get_frame_type(), 0x90);
//     assert_eq!(msg.get_source_addr16(), &[0x56, 0x14]);
//     assert_eq!(
//         msg.get_source_addr64(),
//         &[0x00, 0x13, 0xA2, 0x00, 0x87, 0x65, 0x43, 0x21]
//     );
//     assert_eq!(msg.get_receive_options(), 0x01);
//     assert_eq!(msg.get_data(), &[0x54u8, 0x78, 0x44, 0x61, 0x74, 0x61]);
// }
