use std::io::Read;

use byteorder::{ReadBytesExt, BigEndian};

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
    pub fn from_reader(f: &mut Read) -> Dirstate {
        let mut entries = Vec::new();
        let mut fphash = Vec::with_capacity(20);
        let _ = f.read_exact(&mut fphash);
        let mut sphash = Vec::with_capacity(20);
        let _ = f.read_exact(&mut sphash);
        loop {
            let state = match f.read_u8() {
                Ok(x)   => x,
                Err(_)  => break,
            };
            let mode = f.read_u32::<BigEndian>().unwrap();
            let size = f.read_u32::<BigEndian>().unwrap();
            let mtime = f.read_u32::<BigEndian>().unwrap();
            let namelen = f.read_u32::<BigEndian>().unwrap();
            let mut name = Vec::with_capacity(namelen as usize);
            let _ = f.read_exact(&mut name);
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
