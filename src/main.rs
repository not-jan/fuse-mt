// Main Entry Point :: FUSE-MT test program.
//
// Copyright (c) 2016 by William R. Fraser
//

use std::env;
use std::ffi::{OsStr, OsString};

extern crate daemonize;
extern crate fuse;
extern crate libc;
extern crate syslog;
extern crate time;

#[macro_use]
extern crate log;

mod fs;
mod libc_wrappers;

struct ConsoleLogger;

impl log::Log for ConsoleLogger {
    fn enabled(&self, _metadata: &log::LogMetadata) -> bool {
        //metadata.level() >= log::LogLevel::Debug
        true
    }

    fn log(&self, record: &log::LogRecord) {
        println!("{}: {}: {}", record.target(), record.level(), record.args());
    }
}

fn main() {
    log::set_logger(|max_log_level| {
        max_log_level.set(log::LogLevelFilter::Debug);
        Box::new(ConsoleLogger)
    }).unwrap();

    let args: Vec<OsString> = env::args_os().collect();

    if args.len() != 3 {
        println!("usage: {} <target> <mountpoint>", &env::args().next().unwrap());
        ::std::process::exit(-1);
    }

    let filesystem = fs::Passthrough {
        target: args[1].clone(),
    };

    let translator = fs::InodeTranslator::new(filesystem);

    let fuse_args: Vec<&OsStr> = vec![&OsStr::new("-o"), &OsStr::new("auto_unmount")];

    fuse::mount(translator, &args[2], &fuse_args);
}
