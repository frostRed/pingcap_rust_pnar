use clap::{App, Arg, SubCommand};
use std::process::exit;

fn main() {
    let matches = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .subcommand(
            SubCommand::with_name("set")
                .about("Set the value of a string key to a string")
                .version(env!("CARGO_PKG_VERSION"))
                .author(env!("CARGO_PKG_AUTHORS"))
                .arg(
                    Arg::with_name("key")
                        .value_name("KEY")
                        .short("k")
                        .required(true)
                        .index(1)
                        .help("key"),
                )
                .arg(
                    Arg::with_name("value")
                        .value_name("VALUE")
                        .short("v")
                        .required(true)
                        .index(2)
                        .help("value"),
                ),
        )
        .subcommand(
            SubCommand::with_name("get")
                .about("Get the string value of a given string key")
                .version(env!("CARGO_PKG_VERSION"))
                .author(env!("CARGO_PKG_AUTHORS"))
                .arg(
                    Arg::with_name("key")
                        .value_name("KEY")
                        .short("k")
                        .required(true)
                        .index(1)
                        .help("key"),
                ),
        )
        .subcommand(
            SubCommand::with_name("rm")
                .about("Remove a given key")
                .version(env!("CARGO_PKG_VERSION"))
                .author(env!("CARGO_PKG_AUTHORS"))
                .arg(
                    Arg::with_name("key")
                        .value_name("KEY")
                        .short("k")
                        .required(true)
                        .index(1)
                        .help("key"),
                ),
        )
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("get") {
        let _key = matches.value_of("key").unwrap();
        eprintln!("unimplemented");
        exit(-1);
    }

    if let Some(matches) = matches.subcommand_matches("set") {
        let _key = matches.value_of("key").unwrap();
        let _value = matches.value_of("value").unwrap();
        eprintln!("unimplemented");
        exit(-1);
    }

    if let Some(matches) = matches.subcommand_matches("rm") {
        let _key = matches.value_of("key").unwrap();
        eprintln!("unimplemented");
        exit(-1);
    }

    exit(-1);
}
