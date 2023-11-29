#[derive(Clone, Debug)]
pub struct DeviceAddress {
    data: Vec<u8>,
    hash: usize,
}

impl DeviceAddress {
    pub fn from_bytes_array(data: &[u8]) -> Self {
        Self {
            data: data.to_vec(),
            hash: Self::simple_hash(data),
        }
    }
    fn simple_hash(data: &[u8]) -> usize {
        let mut value = 7;
        for b in data {
            value = 31usize.wrapping_mul(value).wrapping_add(*b as usize);
        }
        value
    }
    pub fn new() -> Self {
        Self {
            data: vec![],
            hash: 0,
        }
    }
    pub fn as_bytes(&self) -> &[u8] {
        &self.data
    }
    pub fn as_hash(&self) -> usize {
        self.hash
    }
}

pub trait GetDeviceAddress {
    //TODO: if its self is needed
    fn get_device_address(&self) -> DeviceAddress;
}
