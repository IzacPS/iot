use std::io::Error;

use crate::device_address::DeviceAddress;
use mio::event::Source;
use protocol::PacketSerializer;

pub enum SendType<'a> {
    None,
    Unicast(&'a [u8]),
    Broadcast,
}

pub trait NetworkStreamSerializer {
    fn serialize(&self, send_type: SendType<'_>, data: &impl PacketSerializer) -> Vec<u8>;
}
pub trait NetworkStreamDeserializer {
    fn deserialize(&self, data: &[u8]) -> Option<(Vec<u8>, DeviceAddress)>;
}

pub trait StreamReader {
    fn read(&mut self, data: &mut [u8]) -> Result<usize, Error>;
}

pub trait StreamWriter {
    fn write(&mut self, data: &[u8]) -> Result<usize, Error>;
}

pub trait StreamSource {
    fn mut_source(&mut self) -> &mut impl Source;
}
