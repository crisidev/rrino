#![cfg_attr(feature = "clippy", feature(plugin))]
#![cfg_attr(feature = "clippy", plugin(clippy))]

#[macro_use]
extern crate log;
#[macro_use]
extern crate chan;

extern crate clap;
extern crate pretty_env_logger;
extern crate hyper;
extern crate futures;
extern crate serde;
extern crate serde_json;
extern crate chan_signal;
extern crate tokio_core;

use std::str::FromStr;
use futures::{Future, Stream};

struct Webhook;

impl hyper::server::NewService for Webhook {
    type Request = hyper::server::Request;
    type Response = hyper::server::Response;
    type Error = hyper::error::Error;
    type Instance = Webhook;
    fn new_service(&self) -> Result<Self::Instance, std::io::Error> {
        Ok(Webhook)
    }
}

impl hyper::server::Service for Webhook {
    type Request = hyper::server::Request;
    type Response = hyper::server::Response;
    type Error = hyper::error::Error;
    type Future = Box<Future<Item = Self::Response, Error = Self::Error>>;
    fn call(&self, req: hyper::server::Request) -> Self::Future {
        let (method, path, _, _headers, body) = req.deconstruct();
        // Make sure we only recieve POST requests from Github
        if method == hyper::Method::Post {
            if path == "/shutdown" {
                chan_signal::kill_this(chan_signal::Signal::TERM);
            }
            // Get all of the chunks streamed to us in our request
            // GitHub gives us a lot of data so there might be
            // more than one Chunk
            Box::new(
                body.collect()
                // Then put them all into a single buffer for parsing
                .and_then(move |chunk| {
                    let mut buffer: Vec<u8> = Vec::new();
                    for i in chunk {
                        buffer.append(&mut i.to_vec());
                    }
                    Ok(buffer)
                })
                // If there is JSON do things with it
                // Send to the server that we got the data
                .map(move |buffer| {
                    if !buffer.is_empty() {
                        match serde_json::from_slice::<serde_json::Value>(&buffer) {
                            Ok(val) => info!("{:#?}", val),
                            Err(error) => error!("{:#?}", error)
                        }
                    }
                    hyper::server::Response::new()
                }),
            )
        } else {
            let mut res = hyper::server::Response::new();
            res.set_status(hyper::StatusCode::MethodNotAllowed);
            Box::new(futures::finished(res))
        }
    }
}

fn main() {
    pretty_env_logger::init().unwrap();

    let args = clap::App::new("rRino")
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
        .arg(clap::Arg::with_name("service-dir")
             .short("d")
             .long("service-dir")
             .value_name("PATH")
             .help("path for service dir containing lock and pid files (default: $HOME/.rrino)")
             .takes_value(true))
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
             .value_name("ADDREss")
             .takes_value(true))
        .get_matches();

    let link = args.value_of("link").unwrap();
    let stop = args.is_present("stop");
    let service_dir: String;
    if args.is_present("service-dir") {
        service_dir = args.value_of("service-dir").unwrap().to_owned();
    } else {
        match std::env::home_dir() {
            Some(path) => service_dir = format!("{}/.rrino", path.display()),
            None => panic!("Impossible to get your home dir!"),
        }
    };
    let notifier = args.value_of("notifier").unwrap_or(
        "/usr/local/bin/terminal-notifier",
    );
    let sender = args.value_of("sender").unwrap_or("com.apple.Terminal");
    let address = args.value_of("address").unwrap_or("127.0.0.1");
    let port: u16;
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

    debug!("command line arg link: {}", link);
    debug!("command line arg stop: {}", stop);
    debug!("command line arg service-dir: {}", service_dir);
    debug!("command line arg notifier: {}", notifier);
    debug!("command line arg sender: {}", sender);
    debug!("command line arg tag: {}", tag);
    debug!("command line arg address: {}", address);
    debug!("command line arg port: {}", port);

    if stop {
        let mut core = tokio_core::reactor::Core::new().unwrap();
        let client = hyper::client::Client::new(&core.handle());
        let uri = format!("http://{}:{}/shutdown", address, port)
            .parse()
            .unwrap();
        let req: hyper::client::Request = hyper::client::Request::new(hyper::Method::Post, uri);
        let post = client.request(req).and_then(|res| res.body().concat2());
        match core.run(post) {
            Ok(_) => info!("rRino shudown successful"),
            Err(_) => info!("rRino shudown successful"),
        }
    } else {
        let signal = chan_signal::notify(&[chan_signal::Signal::INT, chan_signal::Signal::TERM]);
        let (sdone, rdone) = chan::sync::<usize>(0);

        let address_clone = address.to_owned();
        std::thread::spawn(move || {
            run(
                address_clone.as_ref(),
                &port.to_owned(),
                tag.to_owned().as_ref(),
                &sdone,
            )
        });

        chan_select! {
        signal.recv() -> signal => {
            warn!("received signal: {:?}", signal);
        },
        rdone.recv() => {
            info!("rRino completed normally.");
        }
    }
    }
}

fn run(address: &str, port: &u16, tag: &str, _sdone: &chan::Sender<usize>) {
    info!(
        "starting rRino HTTP server for tag {} on {}:{}",
        tag,
        address,
        port
    );
    let socket = std::net::SocketAddr::new(
        std::net::IpAddr::V4(std::net::Ipv4Addr::from_str(address).unwrap()),
        *port,
    );
    let _ = hyper::server::Http::new()
        .bind(&socket, Webhook)
        .map(|server| server.run())
        .map_err(|e| error!("Server failed to setup: {}", e));
}
