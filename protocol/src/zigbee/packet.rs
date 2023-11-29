// pub struct SendPacket {
//     data: Vec<u8>,
// }

// impl SendPacket {
//     pub fn new() -> Self {
//         let mut data = vec![0x00; 255];
//         data[0] = 0x7E;
//         Self { data }
//     }
//     pub fn set_byte(&mut self, mut position: usize, value: u8) -> usize {
//         if value == 0x7E || value == 0x7D || value == 0x11 || value == 0x13 {
//             self.data[position] = 0x7D;
//             position += 1;
//             self.data[position] = value ^ 0x20;
//         } else {
//             self.data[position] = value;
//         }
//         position + 1
//     }
//     pub fn get_data(&self) -> &[u8] {
//         &self.data
//     }
//     pub fn get_data_mut(&mut self) -> &mut [u8] {
//         &mut self.data
//     }
// }

// pub struct RecvPacket {
//     data: Vec<u8>,
//     escape: bool,
// }

// impl RecvPacket {
//     pub fn new() -> Self {
//         Self {
//             data: Vec::with_capacity(255),
//             escape: false,
//         }
//     }

//     pub fn set_next_byte(&mut self, value: u8) {
//         if self.data.len() > 0 && (value == 0x7E || value == 0x7D || value == 0x11 || value == 0x13)
//         {
//             self.escape = true;
//             return;
//         }
//         if self.escape {
//             self.data.push(value ^ 0x20);
//             self.escape = false;
//         } else {
//             self.data.push(value);
//         }
//     }
//     pub fn get_data(&self) -> &[u8] {
//         &self.data
//     }
//     pub fn get_data_mut(&mut self) -> &mut [u8] {
//         &mut self.data
//     }
// }

use crate::{push_as_api2, InsertSlice, PacketSerializer, PushAsApi2};
pub enum PacketSendType<'a> {
    LocalAtCommand {
        frame_id: u8,
        at_command: Option<[u8; 2]>,
        parameter: Option<&'a [u8]>,
    },
    TransmitRequest {
        frame_id: u8,
        destination_addr64: Option<[u8; 8]>,
        destination_addr16: Option<[u8; 2]>,
        radius: u8,
        options: u8,
        payload: Option<&'a [u8]>,
    },
}

impl PacketSerializer for PacketSendType<'_> {
    fn serialize(&self, buf: &mut [u8]) -> usize {
        let mut index: usize = 0;
        match self {
            PacketSendType::LocalAtCommand {
                frame_id,
                at_command,
                parameter,
            } => {
                let mut checksum: u16 = 0;
                let mut len: u16 = 1 + 1;

                buf[index] = 0x7E;
                index += 1;

                push_as_api2(buf, &mut index, 0x00);
                push_as_api2(buf, &mut index, 0x00);

                push_as_api2(buf, &mut index, 0x08);

                push_as_api2(buf, &mut index, *frame_id);

                if let Some(at_command) = at_command {
                    len += at_command.len() as u16;
                    for b in at_command {
                        push_as_api2(buf, &mut index, *b);
                    }
                }

                if let Some(parameter) = *parameter {
                    len += parameter.len() as u16;
                    for b in parameter {
                        push_as_api2(buf, &mut index, *b);
                    }
                }

                #[cfg(target_endian = "big")]
                {
                    buf[1] = len as u8;
                    buf[2] = (len >> 8) as u8;
                }
                #[cfg(target_endian = "little")]
                {
                    buf[1] = (len >> 8) as u8;
                    buf[2] = len as u8;
                }

                let mut escape = false;
                for value in &buf[3..] {
                    if *value == 0x7E || *value == 0x7D || *value == 0x11 || *value == 0x13 {
                        escape = true;
                        continue;
                    }
                    if escape {
                        checksum += (*value ^ 20) as u16;
                        escape = false;
                    } else {
                        checksum += *value as u16;
                    }
                }

                let checksum = checksum as u8;
                push_as_api2(buf, &mut index, 0xFF - checksum);
                index
            }
            PacketSendType::TransmitRequest {
                frame_id,
                destination_addr64,
                destination_addr16,
                radius,
                options,
                payload,
            } => {
                let mut checksum: u16 = 1;
                let mut len: u16 = 1 + 1;
                buf[index] = 0x7E;
                index += 1;

                push_as_api2(buf, &mut index, 0x00);
                push_as_api2(buf, &mut index, 0x00);

                push_as_api2(buf, &mut index, 0x10);

                push_as_api2(buf, &mut index, *frame_id);

                if let Some(addr64) = destination_addr64 {
                    len += addr64.len() as u16;
                    for b in addr64 {
                        push_as_api2(buf, &mut index, *b);
                    }
                }

                if let Some(addr16) = destination_addr16 {
                    len += addr16.len() as u16;
                    for b in addr16 {
                        push_as_api2(buf, &mut index, *b);
                    }
                }

                push_as_api2(buf, &mut index, *radius);
                len += 1;

                push_as_api2(buf, &mut index, *options);
                len += 1;

                if let Some(payload) = *payload {
                    len += payload.len() as u16;
                    for b in payload {
                        push_as_api2(buf, &mut index, *b);
                    }
                }

                #[cfg(target_endian = "big")]
                {
                    buf[1] = len as u8;
                    buf[2] = (len >> 8) as u8;
                }
                #[cfg(target_endian = "little")]
                {
                    buf[1] = (len >> 8) as u8;
                    buf[2] = len as u8;
                }

                let mut escape = false;
                for value in &buf[3..] {
                    if *value == 0x7E || *value == 0x7D || *value == 0x11 || *value == 0x13 {
                        escape = true;
                        continue;
                    }
                    if escape {
                        checksum += (*value ^ 20) as u16;
                        escape = false;
                    } else {
                        checksum += *value as u16;
                    }
                }

                let checksum = checksum as u8;
                push_as_api2(buf, &mut index, 0xFF - checksum);
                index
            }
        }
    }
}

pub enum PacketRecvType {
    LocalAtCommandResponse {
        frame_type: u8,
        frame_id: u8,
        at_command: [u8; 2],
        status: AtCommandResponseError,
        command_data: Vec<u8>,
    },
    ModemStatus {
        frame_type: u8,
        status: ModemStatus,
    },
    ExtendedTransmitStatus {
        frame_type: u8,
        frame_id: u8,
        destination_addr: [u8; 2],
        transmission_count: u8,
        status: DeliveryStatus,
        discovery_status: DiscoveryStatus,
    },
    ReceivePacket {
        frame_type: u8,
        source_addr64: [u8; 8],
        source_addr16: [u8; 2],
        options: u8,
        data: Vec<u8>,
    },
}

impl PacketRecvType {
    pub fn deserialize(data: &[u8]) -> Self {
        let mut buf = [0u8; 255];
        let mut count = 1;
        let mut is_escape = false;
        for c in data {
            if *c == 0x7D {
                is_escape = true;
                continue;
            }
            if is_escape {
                buf[count] = *c ^ 0x20;
                is_escape = false;
            } else {
                buf[count] = *c;
            }
            count += 1;
        }

        match buf[3] {
            0x88 => Self::LocalAtCommandResponse {
                frame_type: 0x88,
                frame_id: buf[4],
                at_command: [buf[5], buf[6]],
                status: AtCommandResponseError::from_value(buf[7]),
                command_data: buf[8..].to_vec(),
            },
            0x8A => Self::ModemStatus {
                frame_type: 0x8A,
                status: ModemStatus::from_value(buf[4]),
            },
            0x8B => Self::ExtendedTransmitStatus {
                frame_type: 0x8B,
                frame_id: buf[4],
                destination_addr: [buf[5], buf[6]],
                transmission_count: buf[7],
                status: DeliveryStatus::from_value(buf[8]),
                discovery_status: DiscoveryStatus::from_value(buf[9]),
            },
            0x90 => Self::ReceivePacket {
                frame_type: 0x90,
                source_addr64: [
                    buf[4], buf[5], buf[6], buf[7], buf[8], buf[9], buf[10], buf[11],
                ],
                source_addr16: [buf[12], buf[13]],
                options: buf[14],
                data: buf[15..].to_vec(),
            },
            _ => todo!(),
        }
        //
    }
}

#[derive(PartialEq, Debug)]
pub enum AtCommandResponseError {
    Ok,
    Error,
    InvalidCommand,
    InvalidParameter,
}

impl AtCommandResponseError {
    pub fn from_value(value: u8) -> Self {
        match value {
            1 => Self::Error,
            2 => Self::InvalidCommand,
            3 => Self::InvalidParameter,
            _ => Self::Ok,
        }
    }
}

// impl AtCommandResponse {
//     pub fn new(packet: RecvPacket) -> Self {
//         Self { packet }
//     }

//     pub fn get_frame_type(&self) -> u8 {
//         self.packet.get_data()[3]
//     }
//     pub fn get_frame_id(&self) -> u8 {
//         self.packet.get_data()[4]
//     }

//     pub fn get_at_command(&self) -> [u8; 2] {
//         [self.packet.get_data()[5], self.packet.get_data()[6]]
//     }

//     pub fn get_status(&self) -> AtCommandResponseError {
//         match self.packet.get_data()[7] {
//             0x00 => AtCommandResponseError::Ok,
//             0x01 => AtCommandResponseError::InvalidCommand,
//             0x02 => AtCommandResponseError::InvalidParameter,
//             _ => AtCommandResponseError::Error,
//         }
//     }
//     pub fn get_command_data(&self) -> &[u8] {
//         &self.packet.get_data()[8..]
//     }
// }

#[derive(PartialEq, Debug)]
pub enum ModemStatus {
    HardwareStart,                      //Hardware reset or power up
    WDTimerReset,                       //Watchdog timer reset
    JoinedNetwork,                      //Joined network
    LeftNetwork,                        //Left network
    CoordinatorStarted,                 //Coordinator started
    NetworkSecurityKeyUpdated,          //Network security key was updated
    NetworkWokeup,                      //Network woke up
    NetworkAsleep,                      //Network went to sleep
    VoltageSupplyLimitExcceded,         //Voltage supply limit exceeded
    DigiRemoteManagerConnected,         //Digi Remote Manager connected
    DigiRemoteManagerDisconnected,      //Digi Remote Manager disconnected
    ConfigurationChangedInJoinProgress, //Modem configuration changed while join in progress
    AccessFault,                        //Access fault
    FatalError,                         //Fatal error
    SecuritySectionEstablished,         //Secure session successfully established
    SecuritySectionEnded,               //Secure session ended
    SecuritySectionAuthFailed,          //Secure session authentication failed
    CoordinatorConflictPanidNoAction,   //Coordinator detected a PAN ID conflict but took no action
    CoordinatorConflictedPanidChanged,  //Coordinator changed PAN ID due to a conflict
    BLEConnected,                       //BLE Connect
    BLEDisconnected,                    //BLE Disconnect
    BandmaskFailed,                     //Bandmask configuration failed
    CellComponentUpdateStarted,         //Cellular component update started
    CellComponentUpdateFailed,          //Cellular component update failed
    CellComponentUpdateCompleted,       //Cellular component update completed
    XbeeFirmwareUpdateStarted,          //XBee firmware update started
    XbeeFirmwareUpdateFailed,           //XBee firmware update failed
    XbeeFirmwareUpdateApplying,         //XBee firmware update applying
    RouterPanidChangedByconflict,       //Router PAN ID was changed by coordinator due to a conflict
    NetworkWatchdogTimeoutExpired,      //Network Watchdog timeout expired
    StackError,                         //0x80 through 0xFF = Stack error
                                        //Refer to the tables below for a filtered list of status codes that are
}

impl ModemStatus {
    pub fn from_value(value: u8) -> Self {
        match value {
            0x01 => Self::WDTimerReset,
            0x02 => Self::JoinedNetwork,
            0x03 => Self::LeftNetwork,
            0x06 => Self::CoordinatorStarted,
            0x07 => Self::NetworkSecurityKeyUpdated,
            0x0B => Self::NetworkWokeup,
            0x0C => Self::NetworkAsleep,
            0x0D => Self::VoltageSupplyLimitExcceded,
            0x0E => Self::DigiRemoteManagerConnected,
            0x0F => Self::DigiRemoteManagerDisconnected,
            0x11 => Self::ConfigurationChangedInJoinProgress,
            0x12 => Self::AccessFault,
            0x13 => Self::FatalError,
            0x3B => Self::SecuritySectionEstablished,
            0x3C => Self::SecuritySectionEnded,
            0x3D => Self::SecuritySectionAuthFailed,
            0x3E => Self::CoordinatorConflictPanidNoAction,
            0x3F => Self::CoordinatorConflictedPanidChanged,
            0x32 => Self::BLEConnected,
            0x33 => Self::BLEDisconnected,
            0x34 => Self::BandmaskFailed,
            0x35 => Self::CellComponentUpdateStarted,
            0x36 => Self::CellComponentUpdateFailed,
            0x37 => Self::CellComponentUpdateCompleted,
            0x38 => Self::XbeeFirmwareUpdateStarted,
            0x39 => Self::XbeeFirmwareUpdateFailed,
            0x3A => Self::XbeeFirmwareUpdateApplying,
            0x40 => Self::RouterPanidChangedByconflict,
            0x42 => Self::NetworkWatchdogTimeoutExpired,
            0x80 => Self::StackError,
            _ => Self::HardwareStart,
        }
    }
}

// impl ModemStatusResponse {
//     pub fn new(packet: RecvPacket) -> Self {
//         Self { packet }
//     }

//     pub fn get_frame_type(&self) -> u8 {
//         self.packet.get_data()[3]
//     }

//     pub fn get_status(&self) -> ModemStatus {
//         match self.packet.get_data()[4] {
//             0x00 => ModemStatus::HardwareStart,
//             0x01 => ModemStatus::WDTimerReset,
//             0x02 => ModemStatus::JoinedNetwork,
//             0x03 => ModemStatus::LeftNetwork,
//             0x06 => ModemStatus::CoordinatorStarted,
//             0x07 => ModemStatus::NetworkSecurityKeyUpdated,
//             0x0B => ModemStatus::NetworkWokeup,
//             0x0C => ModemStatus::NetworkAsleep,
//             0x0D => ModemStatus::VoltageSupplyLimitExcceded,
//             0x0E => ModemStatus::DigiRemoteManagerConnected,
//             0x0F => ModemStatus::DigiRemoteManagerDisconnected,
//             0x11 => ModemStatus::ConfigurationChangedInJoinProgress,
//             0x12 => ModemStatus::AccessFault,
//             0x13 => ModemStatus::FatalError,
//             0x3B => ModemStatus::SecuritySectionEstablished,
//             0x3C => ModemStatus::SecuritySectionEnded,
//             0x3D => ModemStatus::SecuritySectionAuthFailed,
//             0x3E => ModemStatus::CoordinatorConflictPanidNoAction,
//             0x3F => ModemStatus::CoordinatorConflictedPanidChanged,
//             0x32 => ModemStatus::BLEConnected,
//             0x33 => ModemStatus::BLEDisconnected,
//             0x34 => ModemStatus::BandmaskFailed,
//             0x35 => ModemStatus::CellComponentUpdateStarted,
//             0x36 => ModemStatus::CellComponentUpdateFailed,
//             0x37 => ModemStatus::CellComponentUpdateCompleted,
//             0x38 => ModemStatus::XbeeFirmwareUpdateStarted,
//             0x39 => ModemStatus::XbeeFirmwareUpdateFailed,
//             0x3A => ModemStatus::XbeeFirmwareUpdateApplying,
//             0x40 => ModemStatus::RouterPanidChangedByconflict,
//             0x42 => ModemStatus::NetworkWatchdogTimeoutExpired,
//             _ => ModemStatus::StackError,
//         }
//     }
// }

// impl TranmissionReceive {
//     pub fn new(packet: RecvPacket) -> Self {
//         Self { packet }
//     }

//     pub fn get_frame_type(&self) -> u8 {
//         self.packet.get_data()[3]
//     }

//     pub fn get_source_addr64(&self) -> &[u8] {
//         &self.packet.get_data()[4..12]
//     }

//     pub fn get_source_addr16(&self) -> &[u8] {
//         &self.packet.get_data()[12..14]
//     }

//     pub fn get_receive_options(&self) -> u8 {
//         self.packet.get_data()[14]
//     }

//     pub fn get_data(&self) -> &[u8] {
//         let size = self.packet.get_data().len();
//         &self.packet.get_data()[15..size - 1]
//     }
// }

#[derive(PartialEq, Debug)]
pub enum DeliveryStatus {
    Success,                  //Success
    MacAckFailure,            //MAC ACK failure
    CcaOrLbtFailure,          //CCA/LBT failure
    IndirectMessage,          //Indirect message unrequested / no spectrum available
    InvalidDestination,       //Invalid destination endpoint
    NetworkAckFailure,        //Network ACK failure
    NotJointed,               //Not joined to network
    SelfAddressed,            //Self-addressed
    AddressNotFound,          //Address not found
    RouteNotFound,            //Route not found
    RelayMessageFailed,       //Broadcast source failed to hear a neighbor relay the message
    InvalidBindindTableIndex, //Invalid binding table index
    ResourceError,            //Resource error - lack of free buffers, timers, etc.
    ApsBroadcast,             //Attempted broadcast with APS transmission
    ApsUnicast,               //Attempted unicast with APS transmission, but EE = 0
    InternalResourceError,    //Internal resource error
    SessionNotSecure,         //No Secure Session connection
    EncriptionFailure,        //Encryption failure
    PayloadTooLarge,          //Data payload too large
    IndirectRequest,          //Indirect message unrequested
    Failure,
}

impl DeliveryStatus {
    pub fn from_value(value: u8) -> Self {
        match value {
            0x01 => Self::MacAckFailure,
            0x02 => Self::CcaOrLbtFailure,
            0x03 => Self::IndirectMessage,
            0x15 => Self::InvalidDestination,
            0x21 => Self::NetworkAckFailure,
            0x22 => Self::NotJointed,
            0x23 => Self::SelfAddressed,
            0x24 => Self::AddressNotFound,
            0x25 => Self::RouteNotFound,
            0x26 => Self::RelayMessageFailed,
            0x2B => Self::InvalidBindindTableIndex,
            0x2C => Self::ResourceError,
            0x2D => Self::ApsBroadcast,
            0x2E => Self::ApsUnicast,
            0x31 => Self::InternalResourceError,
            0x32 => Self::SessionNotSecure,
            0x34 => Self::EncriptionFailure,
            0x35 => Self::PayloadTooLarge,
            0x74 => Self::IndirectRequest,
            0x75 => Self::Failure,
            _ => Self::Success,
        }
    }
}

#[derive(PartialEq, Debug)]
pub enum DiscoveryStatus {
    NoDiscoveryOverhead,
    ZigbeeAddressDiscovery,
    RouteDiscovery,
    AddressAndRouteDiscovery,
    EndDeviceExtendedTimeout,
}

impl DiscoveryStatus {
    pub fn from_value(value: u8) -> Self {
        match value {
            0x01 => Self::ZigbeeAddressDiscovery,
            0x02 => Self::RouteDiscovery,
            0x03 => Self::AddressAndRouteDiscovery,
            0x40 => Self::EndDeviceExtendedTimeout,
            _ => Self::NoDiscoveryOverhead,
        }
    }
}

// impl TransmissionStatusResponse {
//     pub fn new(packet: RecvPacket) -> Self {
//         Self { packet }
//     }

//     pub fn get_frame_type(&self) -> u8 {
//         self.packet.get_data()[3]
//     }
//     pub fn get_frame_id(&self) -> u8 {
//         self.packet.get_data()[4]
//     }

//     pub fn get_destination_address(&self) -> [u8; 2] {
//         [self.packet.get_data()[5], self.packet.get_data()[6]]
//     }

//     pub fn get_transmission_count(&self) -> u8 {
//         self.packet.get_data()[7]
//     }

//     pub fn get_status(&self) -> TransmissionStatus {
//         match self.packet.get_data()[8] {
//             0x00 => TransmissionStatus::Success,
//             0x01 => TransmissionStatus::MacAckFailure,
//             0x02 => TransmissionStatus::CcaOrLbtFailure,
//             0x15 => TransmissionStatus::InvalidDestination,
//             0x21 => TransmissionStatus::NetworkAckFailure,
//             0x22 => TransmissionStatus::NotJointed,
//             0x23 => TransmissionStatus::SelfAddressed,
//             0x24 => TransmissionStatus::AddressNotFound,
//             0x25 => TransmissionStatus::RouteNotFound,
//             0x26 => TransmissionStatus::RelayMessageFailed,
//             0x2B => TransmissionStatus::InvalidBindindTableIndex,
//             0x2C | 0x32 => TransmissionStatus::ResourceError,
//             0x2D => TransmissionStatus::ApsBroadcast,
//             0x2E => TransmissionStatus::ApsUnicast,
//             0x31 => TransmissionStatus::InternalResourceError,
//             0x34 => TransmissionStatus::SessionNotSecure,
//             0x35 => TransmissionStatus::EncriptionFailure,
//             0x74 => TransmissionStatus::PayloadTooLarge,
//             0x03 | 0x75 => TransmissionStatus::IndirectMessage,
//             _ => TransmissionStatus::Failure,
//         }
//     }

//     pub fn get_discovery_status(&self) -> DiscoveryStatus {
//         match self.packet.get_data()[9] {
//             0x00 => DiscoveryStatus::NoDiscoveryOverhead,
//             0x01 => DiscoveryStatus::ZigbeeAddressDiscovery,
//             0x02 => DiscoveryStatus::RouteDiscovery,
//             0x03 => DiscoveryStatus::AddressAndRouteDiscovery,
//             _ => DiscoveryStatus::EndDeviceExtendedTimeout,
//         }
//     }
// }
