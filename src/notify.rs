pub mod osx {
    use std;
    use msg;
    use args;
    use rocket;

    pub fn run(message: msg::kind::NotifyMsg, args: rocket::State<args::cmd::Args>) -> bool {
        debug!("got new notify message: {:#?}", message);
        let mut peekable = message.get_message().chars().peekable();
        while peekable.peek().is_some() {
            let chunk: String = peekable.by_ref().take(args.max_length).collect();
            let the_process = std::process::Command::new(args.notifier.to_owned())
                .args(
                    [
                        "-sender",
                        &args.sender,
                        "-title",
                        &format!("{}: {}", args.tag, message.get_title()),
                        "-message",
                        &chunk,
                    ].iter(),
                )
                .spawn()
                .ok()
                .expect("failed to execute");
            debug!("terminal-notifier PID is: {}", the_process.id());
            let sleep_time = std::time::Duration::from_millis(1500);
            std::thread::sleep(sleep_time);
        }
        true
    }
}
