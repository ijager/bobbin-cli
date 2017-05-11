#![allow(dead_code, unused_variables)]
#![recursion_limit = "1024"]

#[macro_use]
extern crate error_chain;
extern crate clap;
extern crate toml;
extern crate sha1;
extern crate plist;
extern crate termcolor;

mod app;
mod cmd;
mod config;
mod device;
mod ioreg;
mod printer;


use errors::*;
mod errors {
    // Create the Error, ErrorKind, ResultExt, and Result types
    error_chain! {
        links {            
        }
        foreign_links {
            Io(::std::io::Error);            
            PList(::plist::Error);
            Toml(::toml::de::Error);
        }
    }
}

fn main() {
    if let Err(ref e) = run() {
        use ::std::io::Write;
        let stderr = &mut ::std::io::stderr();
        let errmsg = "Error writing to stderr";

        writeln!(stderr, "error: {}", e).expect(errmsg);

        for e in e.iter().skip(1) {
            writeln!(stderr, "caused by: {}", e).expect(errmsg);
        }

        // The backtrace is not always generated. Try to run this example
        // with `RUST_BACKTRACE=1`.
        if let Some(backtrace) = e.backtrace() {
            writeln!(stderr, "backtrace: {:?}", backtrace).expect(errmsg);
        }

        ::std::process::exit(1);
    }
}

fn run() -> Result<()> {
    let args = app::app().get_matches();
    let cfg = config::config(&args)?;
    let mut out = printer::printer().with_verbose(args.is_present("verbose"));

    if let Some(_) = args.subcommand_matches("list") {        
        cmd::list(&cfg, &args, &mut out)
    } else if let Some(_) = args.subcommand_matches("info") {        
        cmd::info(&cfg, &args, &mut out)
    } else {
        println!("{}", args.usage());
        Ok(())
    }
    
    // if let Some(cmd_args) = args.subcommand_matches("list") {        
    //     try!(cmd_list(&args, cmd_args));
    // } else if let Some(cmd_args) = args.subcommand_matches("load") {
    //     try!(cmd_device(&args, cmd_args));
    // } else if let Some(cmd_args) = args.subcommand_matches("run") {
    //     try!(cmd_device(&args, cmd_args));
    // } else if let Some(cmd_args) = args.subcommand_matches("halt") {
    //     try!(cmd_device(&args, cmd_args));
    // } else if let Some(cmd_args) = args.subcommand_matches("resume") {
    //     try!(cmd_device(&args, cmd_args));
    // } else if let Some(cmd_args) = args.subcommand_matches("reset") {
    //     try!(cmd_device(&args, cmd_args));
    // } else if let Some(cmd_args) = args.subcommand_matches("console") {
    //     try!(cmd_console(&args, cmd_args));
    // } else if let Some(cmd_args) = args.subcommand_matches("debug") {
    //     try!(cmd_debug(&args, cmd_args));
    // } else {
    //     println!("{}", args.usage());
    // }    
}