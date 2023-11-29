pub mod mqtt;
pub mod mqtt_sn;
pub mod zigbee;

pub trait PacketSerializer {
    fn serialize(&self, buf: &mut [u8]) -> usize;
}

pub trait PacketDeserializer<'a> {
    fn deserialize(data: &'a [u8]) -> Self;
}
