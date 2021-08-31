# skraid
Skraid -- a word play on scram, scam and raid -- is a bot whose only purpose is to put an end to the growing amount of
raids and scams in Discord.


## Usage
| Environment variable | Description                           | Required | Default |
|:-------------------- |:------------------------------------- |:-------- |:------- |
| DISCORD_BOT_TOKEN    | Discord bot token                     | yes      | `""`    |
| MAINTAINER_ID        | User ID of the maintainer of the bot  | yes      | `""`    |
| COMMAND_PREFIX       | String prepending all bot commands.   | no       | `s!`    |
| DATABASE_PATH        | Path to the SQLite database file      | no       | `""`    |


## Getting started
To invite the bot in the server: `https://discord.com/oauth2/authorize?client_id=<YOUR_BOT_CLIENT_ID>&scope=bot&permissions=11332`

By default, Skraid will be on alert-only mode. This means that until you create a channel and configure Skraid to send
alerts to that channel, it will not do anything.

Once you've created the alert channel, make sure that Skraid has access to the channel and type the following:
```
s!set_alert_channel <ALERT_CHANNEL_ID>
```
Where `<ALERT_CHANNEL_ID>` is the channel ID of your alert channel (e.g. `860216911907298444`)


## Features
- Bans new users if they're part of the global ban list. Does not affect users that were already in the server.
- Supports per-server list of "exceptions" (allowlist), in case a guild wishes to let a user in the global ban list (blocklist) join their server anyways.
- Detect messages containing known phishing/scam links, delete said messages and send an alert.
- Configuration for setting up a channel for alerts, including replacing all actions by alerts sent to said channel.
- Has some utility functions to manage raids


## Glossary
- **blocklist**: A global ban list maintained by the bot's maintainer.
- **allowlist**: A per-guild list of user ids. Used only in case Skraid has banned a user whom you believe is a legitimate user. 
- **forbidden words**: A list of words that, if detected, will alert or act on the user who sent the message.