use clap::{App, Arg, ArgMatches};

pub fn get_args<'a>() -> ArgMatches<'a> {
    App::new("deedoo")
        .version("0.1")
        .author("versbinarii <versbinarii@gmail.com>")
        .about("File deduplicator")
        .arg(
            Arg::with_name("directory")
                .required(true)
                .help("Directory to scan.")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("out_directory")
                .help("Directory for duplicated files.")
                .short("o")
                .long("output")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("ensure")
                .help("Runs additional check to verify duplicate.")
                .short("E")
                .long("ensure"),
        )
        .arg(
            Arg::with_name("show-only")
                .help("Dont move, just display duplicate location.")
                .short("s")
                .long("show-only"),
        )
        .get_matches()
}
