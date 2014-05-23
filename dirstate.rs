/// Dirstate entry record.
pub struct DirstateEntry {
    pub state: u8,
    pub mode: u32,
    pub size: u32,
    pub mtime: u32,
    pub namelen: u32,
    pub name: Vec<u8>,
}

/// Dirstate.
pub struct Dirstate {
    pub fphash: Vec<u8>,
    pub sphash: Vec<u8>,
    pub entries: Vec<DirstateEntry>,
}

impl Dirstate {
    pub fn from_reader(f: &mut Reader) -> Dirstate {
        let mut entries = Vec::new();
        let fphash = f.read_exact(20).unwrap();
        let sphash = f.read_exact(20).unwrap();
        loop {
            let state = match f.read_u8() {
                Ok(x)   => x,
                Err(_)  => break,
            };
            let mode = f.read_be_u32().unwrap();
            let size = f.read_be_u32().unwrap();
            let mtime = f.read_be_u32().unwrap();
            let namelen = f.read_be_u32().unwrap();
            let name = f.read_exact(namelen as uint).unwrap();
            let entry = DirstateEntry {
                state: state,
                mode: mode,
                size: size,
                mtime: mtime,
                namelen: namelen,
                name: name,
            };
            entries.push(entry);
        };
        Dirstate {
            fphash: fphash,
            sphash: sphash,
            entries: entries,
        }
    }
}
