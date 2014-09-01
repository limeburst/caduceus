//! Hematite; Mercurial implementation in Rust.

#![crate_name = "hematite"]
#![crate_type = "bin"]

#![comment = "Hematite"]
#![license = "GPLv2+"]

extern crate serialize;
extern crate time;

use std::io::File;
use std::path::BytesContainer;

mod dirstate;
mod revlog;

fn usage() {
    //! Print usage for the `hg` command.
    print!("\
Hematite Distributed SCM

 debugdirstate   show the contents of the current dirstate
 debugindex      dump the contents of an index file
")
}

fn debugdirstate() {
    //! Show the contents of the current dirstate.
    let path = &Path::new(".hg/dirstate");
    let mut f = File::open(path);
    let mut dirstate = dirstate::Dirstate::from_reader(&mut f);
    dirstate.entries.sort_by(|a, b| a.name.cmp(&b.name));
    for entry in dirstate.entries.iter() {
        println!("{:} {:o} {:>10} {:<19} {}", entry.state.to_ascii(),
                 entry.mode & 0o0777, entry.size,
                 {
                     if entry.mtime == -1 {
                         "unset".to_string()
                     } else {
                         time::at(time::Timespec {
                             sec:entry.mtime as i64,
                             nsec: 0,
                         }).strftime("%Y-%m-%d %H:%M:%S").to_string()
                     }
                 },
                 entry.name.container_as_str().unwrap());
    }
}

fn debugindex(args: Vec<String>) {
    //! Dump the contents of an index file.
    let index_filename = match args[2].as_slice() {
        "-c" => ".hg/store/00changelog.i",
        "-m" => ".hg/store/00manifest.i",
        _    => args[2].as_slice(),
    };
    let path = &Path::new(index_filename);
    let mut f = File::open(path);
    let index = revlog::Revlog::from_reader(&mut f);
    let nullrev = "000000000000";
    println!("{:>6}{:>10}{:>8}{:>7}{:>8} {:<12} {:<12} {:<12}", "rev",
             "offset", "length", "base", "linkrev", "nodeid", "p1", "p2");
    for (rev, record) in index.records.iter().enumerate() {
        println!("{:>6}{:>10}{:>8}{:>7}{:>8} {:>12} {:>12} {:>12}", rev,
                 record.offset, record.clen, record.base, record.link,
                 record.shortid(),
                 {
                     if record.p1 == -1 {
                         nullrev.to_string()
                     } else {
                         index.records.get(record.p1 as uint).shortid()}
                 },
                 {
                     if record.p2 == -1 {
                         nullrev.to_string()
                     } else {
                         index.records.get(record.p2 as uint).shortid()}
                 });
    };
}

fn main() {
    //! Main entry point.
    let args = std::os::args();
    if args.len() < 2 {
        usage();
    } else {
        match args[1].as_slice() {
            "debugdirstate" => debugdirstate(),
            "debugindex"    => debugindex(args),
            _               => usage(),
        }
    }
}
