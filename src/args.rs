pub mod cmd {
    extern crate clap;

    use std;

    #[derive(Debug)]
    pub struct Args {
        pub link: String,
        pub stop: bool,
        pub force: bool,
        pub notifier: String,
        pub sender: String,
        pub address: String,
        pub port: u16,
        pub tag: String,
        pub max_length: usize,
    }

    impl Args {
        pub fn new() -> Args {
            let matches = clap::App::new("rRino")
                .version("0.1")
                .author("Matteo Bigoi <bigo@crisidev.org>")
                .about("Remote IRC (weechat) Notifier OSX")
                .arg(clap::Arg::with_name("link")
                    .short("l")
                    .long("link")
                    .help("link rRino with a SSH exposed port using a tag like \"BIGO:4223\"")
                    .value_name("LINK")
                    .required(true)
                    .takes_value(true))
                .arg(clap::Arg::with_name("stop")
                    .short("s")
                    .long("stop")
                    .help("tell rRino to stop serving the current link"))
                .arg(clap::Arg::with_name("force")
                    .short("f")
                    .long("force")
                    .help("force stop using SIGKILL"))
                .arg(clap::Arg::with_name("notifier")
                    .short("n")
                    .long("notifier")
                    .help("path for terminal-notifier command (default: /usr/local/bin/terminal-notifier)")
                    .value_name("PATH")
                    .takes_value(true))
                .arg(clap::Arg::with_name("sender")
                    .short("S")
                    .long("sender")
                    .help("sender for terminal-notifier command, aka the icon (default: com.apple.Terminal)")
                    .value_name("SENDER")
                    .takes_value(true))
                .arg(clap::Arg::with_name("address")
                    .short("a")
                    .long("address")
                    .help("HTTP server bind address (default: 127.0.0.1)")
                    .value_name("ADDRESS")
                    .takes_value(true))
                .arg(clap::Arg::with_name("max-length")
                    .short("m")
                    .long("max-length")
                    .help("max message lenght before splitting it into multiple notification (default: 92)")
                    .value_name("LENGTH")
                    .takes_value(true))
                .get_matches();

            let link = matches.value_of("link").unwrap();
            let stop = matches.is_present("stop");
            let force = matches.is_present("force");
            let notifier = matches.value_of("notifier").unwrap_or(
                "/usr/local/bin/terminal-notifier",
            );
            let sender = matches.value_of("sender").unwrap_or("com.apple.Terminal");
            let address = matches.value_of("address").unwrap_or("127.0.0.1");
            let port: u16;
            let max_length: usize;
            let tag: String;

            let link_split: Vec<&str> = link.split(':').collect();
            if link_split.len() == 2 {
                tag = link_split[0].to_string();
                match link_split[1].to_string().parse::<u16>() {
                    Ok(val) => port = val,
                    Err(_) => {
                        error!("link {} looks not valid, port should be an u16", link);
                        std::process::exit(1)
                    }
                }
            } else {
                error!("link {} looks not valid, should be tag:port", link);
                std::process::exit(1)
            }

            match matches
                .value_of("max_length")
                .unwrap_or("90")
                .to_string()
                .parse::<usize>() {
                Ok(val) => max_length = val,
                Err(_) => {
                    error!("max length needs to be a integer number");
                    std::process::exit(1)
                }
            }

            let args = Args {
                link: link.to_string(),
                stop,
                force,
                notifier: notifier.to_string(),
                sender: sender.to_string(),
                address: address.to_string(),
                port,
                tag,
                max_length,
            };

            debug!("command line arguments: {:#?}", args);

            return args;
        }
    }
}
