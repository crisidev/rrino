[![Build Status](https://travis-ci.org/crisidev/rrino.svg?branch=master)](https://travis-ci.org/crisidev/rrino)<Paste>

# rRino - Remote IRC Notifier OSX - Weechat

* [Under the hood](#under-the-hood)
* [Installation and usage](#installation-and-usage)
  + [Prerequisites](#prerequisites)
  + [Install](#install)
  + [Start rRino](#start-rrino)
  + [Stop rRino](#stop-rrino)
* [Autossh script](#autossh-script)
* [SSH configs](#ssh-configs)

IRC SSH notifier for MacOSX. It uses terminal-notifier to talk with
the notification center, SSH to publish its port on the remote host
and an Weechat plugin to push notification to the listener.

## Under the hood
This is a small Rust/Python/Bash application that allows you to send
notification on private messages and mentions from your remote Weechat
to your local MacOSX notification center.

To achieve such awesomeness rRino spawns a rust server on a local port. 
This port need to be forwarded to your remote Weechat box, 
where a Python plugins will send notification for privmsg / mentions
to the forwarded port. This will trigger a run of [terminal-notifier](https://github.com/julienXX/terminal-notifier)
and a nice notification with the message will pop on you screen.

rRino has only one mandatory argument (--link) which specifies the server
will run on and a tag to identify the notification.
```shell
rrino -l crisidev:4223
```
To avoid problems, this tag need to be "whitelisted" for the Weechat python
plugin. The whitelisting can be done creating an empty file, named as
the link into /$HOME/.weechat/rrino inside your remote Weechat box.

If your Weechat is on localhost, you just have to avoid the port
forwarding.

With rRino running as
```shell
rino -l crisidev:4223
```
a message from <b>paul</b> with content <b>hello matteo</b> will become
a notification with
* <b>title:</b> crisidev: paul
* <b>message:</b> hello matteo

## Installation and usage
### Prerequisites
* MacOSX
* [weechat](https://weechat.org)
* [terminal-notifier](https://github.com/julienXX/terminal-notifier)
* [rust](https://www.rust-lang.org)
* [OpenSSH](http://www.openssh.com)
* Nohup

### Install
* On your local machine
```shell
$ git clone https://github.com/crisidev/rrino
$ cd rrino
$ cargo build --release
$ cargo install
```

* On your Weechat box
```shell
$ git clone https://github.com/crisidev/rrino
$ cp rrino/weechat_plugins/rrino.py /$HOME/.weechat/python/
$ (from Weechat) /script load rrino.py
```

### Start rRino
```shell
$ nohup rrino -l irc:4223 >> /tmp/irc:4223.log &
$ ssh -R 4223:localhost:4223 $USER@ircbox
```

### Stop rRino
```shell
$ rrino -l irc:4223 -s
$ rrino -l irc:4223 -sf  # force stop if the above fails
```

## Autossh script
There is an utility script inside ```bin/irc``` which allows you to keep weechat and rino
running over tmux using autossh. Open the file, change the configuration variables and enjoy.

```
irc crisidev
```

## SSH configs
See ```ssh_configs``` folder for an example config with remote port forwarding.
