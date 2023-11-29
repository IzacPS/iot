pub fn encode(mut value: u64, buf: &mut [u8]) -> usize {
    // if value > 268435455 {
    //     return 0;
    // }
    //limit to only four Bytes
    // let mut data: Vec<u8> = Vec::with_capacity(4);
    //loop until the value is les then 127 that represents the seven lsb bits
    let mut index = 0;
    while value > 127 {
        buf[index] = (value as u8 & 127) | 128;
        index += 1;
        value >>= 7;
    }
    buf[index] = value as u8 & 127;
    index += 1;
    //return in little endian
    index
}

pub fn decode(data: &[u8], limit: usize) -> Option<(u64, usize)> {
    //receives data in little endian
    let mut value: u64 = 0;
    let mut len = 0;
    for (i, b) in data.iter().enumerate() {
        if len > limit {
            return None;
        }
        value |= (*b as u64 & 127) << (7 * i);
        // if value > 268435455 {
        //     return None;
        // }
        len += 1;
        if (b & 128) == 0 {
            break;
        }
    }
    Some((value, len))
}
