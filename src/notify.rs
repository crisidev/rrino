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
        let chunk = message.message.chars().take(args.max_length).collect();
        notify(&args.notifier, &args.sender, &title, &chunk, &url);
        true
    }

    fn notify(notifier: &String, sender: &String, title: &String, message: &String, url: &String) {
        let mut the_process: std::process::Child;
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
        the_process.wait().expect("failed to wait on child");
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
