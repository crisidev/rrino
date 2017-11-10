pub mod osx {
    use std;
    use msg;
    use args;
    use rocket;

    pub fn run(message: msg::kind::NotifyMsg, args: rocket::State<args::cmd::Args>) -> bool {
        debug!("got new notify message: {:#?}", message);
        debug!("{:#?}", args);
        let mut the_process = std::process::Command::new(args.notifier.to_owned())
            .args(
                [
                    "-sender",
                    &args.sender,
                    "-title",
                    message.get_title(),
                    "-message",
                    message.get_message(),
                ].iter(),
            )
            .spawn()
            .ok()
            .expect("Failed to execute.");
        debug!("terminal-notifier PID is: {}", the_process.id());
        match the_process.wait() {
            Ok(_) => true,
            Err(_) => false,
        }
    }
}
