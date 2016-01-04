//! Caduceus; Mercurial implementation in Rust.

#![feature(negate_unsigned)]

extern crate ascii;
extern crate byteorder;
extern crate rustc_serialize;
extern crate time;

use std::fs::File;
use std::path::Path;

use ascii::AsciiCast;
use time::Timespec;

mod dirstate;
mod revlog;

fn usage() {
    //! Print usage for the `hg` command.
    print!("\
Caduceus Distributed SCM

 debugdirstate   show the contents of the current dirstate
 debugindex      dump the contents of an index file
")
}

fn debugdirstate() {
    //! Show the contents of the current dirstate.
    let path = &Path::new(".hg/dirstate");
    let mut f = File::open(path).unwrap();
    let mut dirstate = dirstate::Dirstate::from_reader(&mut f);
    dirstate.entries.sort_by(|a, b| a.name.cmp(&b.name));
    for entry in dirstate.entries.iter() {
        print!("{} {:o} {:>10} ", entry.state.to_ascii().unwrap(), entry.mode & 0o0777,
               entry.size);
        if entry.mtime == -1 {
            print!("{:<19}", "unset")
        } else {
            print!("{:<19}", time::at(Timespec {
                sec: entry.mtime as i64,
                nsec: 0}).strftime("%Y-%m-%d %H:%M:%S").unwrap().to_string())
        }
        println!(" {}", String::from_utf8(entry.name.clone()).unwrap())
    }
}

fn debugindex(args: Vec<String>) {
    //! Dump the contents of an index file.
    let index_filename = match args[2].as_ref() {
        "-c" => ".hg/store/00changelog.i",
        "-m" => ".hg/store/00manifest.i",
        _    => args[2].as_ref(),
    };
    let path = &Path::new(index_filename);
    let mut f = File::open(path).unwrap();
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
                         index.records.get(record.p1 as usize)
                                      .unwrap()
                                      .shortid()
                     }
                 },
                 {
                     if record.p2 == -1 {
                         nullrev.to_string()
                     } else {
                         index.records.get(record.p2 as usize)
                                      .unwrap()
                                      .shortid()
                     }
                 });
    };
}

fn main() {
    //! Main entry point.
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        usage();
    } else {
        match args[1].as_ref() {
            "debugdirstate" => debugdirstate(),
            "debugindex"    => debugindex(args),
            _               => usage(),
        }
    }
}
