version  0- 4
length   5- 8
data     9- 9+length
this would yield:
pub struct MyPacketReader<'a> {
    raw: &'a [u8],
}

impl<'a> MyPacketReader<'a> {
    pub fn new(raw: &'a [u8]) -> Option<MyPacketReader<'a>> {
        if raw.len() < 8 {
            None
        } else {
            Some(MyPacketReader { raw: raw })
        }
    }

    pub fn get_version(&self) -> u32 {
        read_u32(self.raw)
    }

    pub fn get_data(&self) -> &'a [u8] {
        let length = read_u32(self.raw[4..]);
        self.raw[8..cmp::min(8 + length, self.raw.len())]
    }
}

fn read_u32(data: &[u8]) -> u32 {
    (data[0] as u32 << 24) +
    (data[1] as u32 << 16) +
    (data[2] as u32 <<  8) + 
    (data[3] as u32 <<  0)
}