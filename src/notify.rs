pub mod osx {
    extern crate regex;

    use std;
    use msg;
    use args;
    use rocket;

    pub fn run(message: msg::kind::NotifyMsg, args: rocket::State<args::cmd::Args>) -> bool {
        let title = message.from;
        let url = match_url(&message.message);
        if url != "" {
            notify(&args.notifier, &args.sender, &title, &message.message, &url);
            return true;
        }
        let mut peekable = message.message.chars().peekable();
        while peekable.peek().is_some() {
            let chunk: String = peekable.by_ref().take(args.max_length).collect();
            notify(&args.notifier, &args.sender, &title, &chunk, &url);
            if message.message.chars().count() > args.max_length {
                let sleep_time = std::time::Duration::from_millis(1500);
                std::thread::sleep(sleep_time);
            }
        }
        true
    }

    fn notify(notifier: &String, sender: &String, title: &String, message: &String, url: &String) {
        let the_process: std::process::Child;
        if url != "" {
            the_process = std::process::Command::new(notifier)
                .args(
                    ["-sender", sender, "-title", title, "-message", message, "-open", url].iter(),
                )
                .spawn()
                .ok()
                .expect("failed to execute");
        } else {
            the_process = std::process::Command::new(notifier)
                .args(
                    ["-sender", sender, "-title", title, "-message", message].iter(),
                )
                .spawn()
                .ok()
                .expect("failed to execute");
        }
        debug!("terminal-notifier PID is: {}", the_process.id());
    }

    fn match_url(message: &String) -> String {
        let chunks = message.split(" ");
        for chunk in chunks {
            let re = regex::Regex::new(r"https?://(www\.)?[-a-zA-Z0-9@:%._\+~#=]{2,256}\.[a-z]{2,6}\b([-a-zA-Z0-9@:;%()\[\]{}_\+.*~#?,&//=]*)").unwrap();
            if re.is_match(chunk) {
                debug!("matched url: {}", chunk);
                return String::from(chunk)
            }
        }
        String::from("")
    }
}
