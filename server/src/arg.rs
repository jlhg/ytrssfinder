use clap::{Arg, Command};

pub fn parse_args() -> Command {
    Command::new("server")
        .about("ytrssfinder server")
        .arg(
            Arg::new("log")
                .short('l')
                .default_value("app.log")
                .help("The log file path."),
        )
        .arg(
            Arg::new("bind")
                .short('b')
                .alias("host")
                .default_value("127.0.0.1")
                .help("The IP address where the server is hosted."),
        )
        .arg(
            Arg::new("port")
                .short('p')
                .default_value("6000")
                .help("The port number where the server is listening."),
        )
}
