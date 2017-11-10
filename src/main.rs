#![cfg_attr(feature = "clippy", feature(plugin))]
#![cfg_attr(feature = "clippy", plugin(clippy))]
#![feature(plugin, decl_macro)]
#![plugin(rocket_codegen)]

#[macro_use]
extern crate log;
#[macro_use]
extern crate chan;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate rocket_contrib;

extern crate rocket;
extern crate reqwest;
extern crate chan_signal;
extern crate pretty_env_logger;

mod msg;
mod http;
mod args;
mod notify;

fn handle_stop(args: args::cmd::Args) {
    let mut map = std::collections::HashMap::new();
    if args.force {
        map.insert("force", true);
    } else {
        map.insert("force", false);
    }
    let client = reqwest::Client::new();
    let json = client
        .post(&format!("http://{}:{}/shutdown", args.address, args.port))
        .json(&map)
        .send();
    match json {
        Ok(res) => {
            info!(
                "stopped rRino HTTP server for tag {} on {}:{}, {:#?}",
                args.tag,
                args.address,
                args.port,
                res
            )
        }
        Err(error) => {
            error!(
                "unable to stop rRino HTTP server for tag {} on {}:{}, {}",
                args.tag,
                args.address,
                args.port,
                error
            )
        }
    }
}

fn handle_start(args: args::cmd::Args) {
    let signal = chan_signal::notify(&[chan_signal::Signal::INT, chan_signal::Signal::TERM]);
    let (sdone, rdone) = chan::sync::<usize>(0);

    std::thread::spawn(move || http::server::start(args, &sdone));

    chan_select! {
        signal.recv() -> signal => {
            warn!("received signal: {:?}", signal);
        },
        rdone.recv() => {
            info!("rRino completed normally");
        }
    }

}

fn main() {
    pretty_env_logger::init().unwrap();

    let args = args::cmd::Args::new();

    if args.stop {
        handle_stop(args);
        std::process::exit(0);
    } else {
        handle_start(args);
        std::process::exit(0);
    }
}
