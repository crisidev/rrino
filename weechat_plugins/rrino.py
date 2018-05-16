import os
import json
import urllib2
import datetime
import threading
from httplib import BadStatusLine

import weechat

SCRIPT_NAME = 'rrino'
SCRIPT_AUTHOR = 'Matteo Bigoi <bigo@crisidev.org>'
SCRIPT_VERSION = '0.1'
SCRIPT_LICENSE = 'MIT'
SCRIPT_DESC = 'Pass highlights and private messages to the OS X 10.8+ Notification Center'

weechat.register(SCRIPT_NAME, SCRIPT_AUTHOR, SCRIPT_VERSION, SCRIPT_LICENSE, SCRIPT_DESC, '', '')

DEFAULT_OPTIONS = {
    'show_highlights': 'on',
    'show_private_message': 'on',
    'show_message_text': 'on',
    'ignore_old_messages': 'off',
    'server_addr': '127.0.0.1',
}

for key, val in DEFAULT_OPTIONS.items():
    if not weechat.config_is_set_plugin(key):
        weechat.config_set_plugin(key, val)

weechat.hook_print('', 'irc_privmsg', '', 1, 'notify', '')


def push(data, tag, port):
    try:
        req = urllib2.Request('http://{}:{}/notify'.format(weechat.config_get_plugin('server_addr'), port))
        req.add_header('Content-Type', 'application/json')
        resp = urllib2.urlopen(req, json.dumps(data), timeout=0.5)
        if resp.getcode() != 200:
            weechat.prnt(
                "",
                "%srrino http server %s:%s error, status code %s" % (weechat.prefix("error"), tag, port, resp.getcode())
            )
    except Exception:
        pass


def push_notification(user, message):
    rrino_dir = os.path.join(weechat.info_get('weechat_dir', ''), 'rrino')
    for client in os.listdir(rrino_dir):
        client_split = client.split(":")
        if len(client_split) == 2:
            tag, port = client_split
            data = {'from': '{}: {}'.format(tag, user), 'message': message}
            t = threading.Thread(target=push, args=(data, tag, port))
            t.daemon = True
            t.start()


def notify(data, buffer, date, tags, displayed, highlight, user, message):
    # ignore if it's yourself
    own_nick = weechat.buffer_get_string(buffer, 'localvar_nick')
    if user == own_nick or user == ('@%s' % own_nick):
        return weechat.WEECHAT_RC_OK

    if weechat.config_get_plugin('ignore_old_messages') == 'on':
        message_time = datetime.datetime.utcfromtimestamp(int(date))
        now_time = datetime.datetime.utcnow()

        # ignore if the message is greater than 5 seconds old
        if (now_time - message_time).seconds > 5:
            return weechat.WEECHAT_RC_OK

    if weechat.config_get_plugin('show_message_text') == 'off':
        message = 'Private message'

    if weechat.config_get_plugin('show_highlights') == 'on' and int(highlight):
        channel = weechat.buffer_get_string(buffer, 'localvar_channel')
        user = '{}@{}'.format(user, channel)
        push_notification(user, message)
    elif weechat.config_get_plugin('show_private_message') == 'on' and 'notify_private' in tags:
        push_notification(user, message)

    return weechat.WEECHAT_RC_OK
