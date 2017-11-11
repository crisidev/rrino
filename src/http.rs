pub mod server {
    use std;
    use args;
    use notify;
    use chan;
    use chan_signal;
    use rocket;
    use rocket::config::{Config, Environment};
    use rocket_contrib::{Json, JsonValue};
    use msg::kind::{NotifyMsg, ShutdownMsg};

    #[post("/notify", format = "application/json", data = "<message>")]
    fn notify(message: Json<NotifyMsg>, args: rocket::State<args::cmd::Args>) -> JsonValue {
        debug!("got new notify message: {:#?}", message);
        let success = notify::osx::run(message.0, args);
        if success {
            json!({"code": 200, "reason": "notification sent"})
        } else {
            json!({"code": 500, "reason": "error sending notification to osx"})
        }
    }

    #[post("/shutdown", format = "application/json", data = "<message>")]
    fn shutdown(message: Json<ShutdownMsg>) -> JsonValue {
        debug!("requested rRino HTTP server shutdown: {:#?}", message);
        if message.force {
            std::thread::spawn(move || stop(chan_signal::Signal::KILL));
        } else {
            std::thread::spawn(move || stop(chan_signal::Signal::TERM));
        }
        json!({ "code": 200, "reason": "server shutdown started" })
    }

    #[catch(404)]
    fn not_found() -> JsonValue {
        json!({"code": 404, "reason": "not found"})
    }

    #[catch(400)]
    fn bad_request() -> JsonValue {
        json!({"code": 400, "reason": "bad request"})
    }

    fn stop(signal: chan_signal::Signal) {
        let sleep_time = std::time::Duration::from_millis(200);
        std::thread::sleep(sleep_time);
        warn!("sending {:#?} signal to rRino HTTP server", signal);
        chan_signal::kill_this(signal);
    }

    pub fn start(args: args::cmd::Args, _sdone: &chan::Sender<usize>) {
        info!("starting rRino HTTP server for tag {} on {}:{}", args.tag, args.address, args.port);
        let config = Config::build(Environment::Production)
            .address(args.address.to_owned())
            .port(args.port)
            .workers(2)
            .unwrap();
        let app = rocket::custom(config, true);
        app.mount("/", routes![notify, shutdown])
            .catch(catchers![not_found, bad_request])
            .manage(args)
            .launch();
    }
}
