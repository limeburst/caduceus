use serialize::hex::ToHex;

/// The revlog record.
struct RevlogRecord {
    pub offset: u64,
    pub clen: u32,
    pub ulen: u32,
    pub base: i32,
    pub link: i32,
    pub p1: i32,
    pub p2: i32,
    pub nodeid: Vec<u8>,
    pub data: Vec<u8>,
}

/// The revlog.
pub struct Revlog {
    pub records: Vec<RevlogRecord>,
    pub version: u32,
}

impl Revlog {
    pub fn from_reader(f: &mut Reader) -> Revlog {
        let mut records = Vec::new();
        let mut version = 0;
        loop {
            let header = match f.read_be_u64() {
                Ok(x)   => x,
                Err(_)  => break,
            };
            let offset = {
                if records.len() == 0 {
                    version = (header >> 32 & 0x0000ffff) as u32;
                    0
                } else {
                    header >> 16
                }
            };
            let clen = f.read_be_u32().unwrap();
            let ulen = f.read_be_u32().unwrap();
            let base = f.read_be_i32().unwrap();
            let link = f.read_be_i32().unwrap();
            let p1 = f.read_be_i32().unwrap();
            let p2 = f.read_be_i32().unwrap();
            let nodeid = f.read_exact(32).unwrap();
            let data = f.read_exact(clen as uint).unwrap();
            let record = RevlogRecord {
                offset: offset,
                clen: clen,
                ulen: ulen,
                base: base,
                link: link,
                p1: p1,
                p2: p2,
                nodeid: nodeid,
                data: data,
            };
            records.push(record);
        };
        Revlog {
            records: records,
            version: version,
        }
    }
}

impl RevlogRecord {
    pub fn shortid(&self) -> ~str {
        self.nodeid.slice(0, 6).to_hex().to_str()
    }
}
