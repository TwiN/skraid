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


## Features
- Bans new users if they're part of the global ban list. Does not affect users that were already in the server.
- Supports per-server list of "exceptions" (allowlist), if a legitimate user is in the blocklist (global ban list)
- Detect messages containing known phishing/scam links, ban them and add them to the global ban list. **(TODO)**
- Command to enable/disable
- Has some utility functions to manage raids


## Glossary
- **blocklist**: A global ban list maintained by the bot's maintainer.
- **allowlist**: A per-guild list of user ids. Used only in case Skraid has banned a user whom you believe is a legitimate user. 