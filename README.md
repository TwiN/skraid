# skraid
Skraid - a word play on scram, scam and raid - is an attempt to harness the power of large communities in order
to put an end, or at least to reduce the blast radius of raids and common scams by ensuring that scammers and
users known to take part in raids are banned before they can cause any harm.

This is, however, easier said than done.

This bot is built with several fail-safe mechanisms to prevent abuse.

One of these fail-safe mechanisms is the fact that inviting the bot to a guild does not allow you to modify the global
ban list. The staff members of each guild (i.e. users with BAN_MEMBERS permissions) may suggest the addition of users
to the global ban list by using s!suggest_blocklist USER_ID, but the decision is ultimately up to several factors
which will not be disclosed to prevent malicious actors from attempting to circumvent the system. Do not be too
worried, though, as one of the measures put in place requires manual action from the maintainer of the bot.


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
- Send an alert if a user in the global blocklist has joined the server (or ban said user, if configured to do so). Does not affect users that were already in the server.
- Supports per-server list of "exceptions" (allowlist), in case a guild wishes to let a user in the global ban list (blocklist) join their server anyways. This only really applies if the bot is configured to ban instead of alert.
- Detect messages containing known phishing/scam links and send an alert, or delete said messages if configured to do so.
- Configuration for setting up a channel for alerts, including replacing all actions by alerts sent to said channel.
- Has some utility commands to manage raids


## Commands
All commands must be prefixed by the `COMMAND_PREFIX`, or `s!` by default.

### Configuration
| Command name      | Description |
|:----------------- |:----------- |
| get_guild_config  | Retrieve the current guild configuration.
| set_alert_channel | Configure an alert channel by passing the desired channel id as argument
| set_alert_only    | Configure Skraid's mode.By default, this is set to true. If set to false, if a user in the blocklist joins the server, they will be automatically banned. Likewise, if a user posts a message containing a forbidden word (e.g. a link known to be related to phishing), said message will be deleted. In any case, alerts will be sent as long as the alert channel is configured.

### Allowlist
Each guild has their own separate allowlist which, in the case that they chose to enable Skraid's automatic banning capabilities,
can allow a user they believe was wrongly blocklisted to join the server. 

If `alert_only` is set to `true` (by default, it is), there's no need to use the allowlist as the only thing you'd be 
preventing is the creation of an alert.

| Command name          | Description |
|:--------------------- |:----------- |
| allowlist             | Add user ID to the guild's list of exceptions
| unallowlist           | Remove user ID from the guild's list of exception
| is_allowlisted        | Check if a user ID is in the guild's list of exception
| get_allowlisted_users | Retrieves a list of all allowlisted user ids for this guild

### Suggestion
Suggestions are the mean by which guild staff members may communicate with the maintainer.

| Command name           | Description |
|:---------------------- |:----------- |
| suggest_forbidden_word | Suggest a word to add to the list of global forbidden words to the maintainer
| suggest_blocklist      | Suggest the addition of a user ID to the global blocklist to the maintainer.

### Utilities
| Command name | Description |
|:------------ |:----------- |
| clear        | Clear N messages from the current channel
| status       | Check the status of the bot

### Maintainer
| Command name             | Description |
|:------------------------ |:----------- |
| blocklist                | Add user ID to the blocklist
| unblocklist              | Remove user ID to the blocklist
| is_blocklisted           | Check if user ID is in the blocklist
| forbid_word              | Add word in the list of forbidden words
| unforbid_word            | Remove word in the list of forbidden words
| contains_forbidden_word  | Checks if a string contains a forbidden word
| get_forbidden_words      | Retrieve a list of all forbidden words


## Glossary
- **blocklist**: A global ban list maintained by the bot's maintainer.
- **allowlist**: A per-guild list of user ids. Used only in case Skraid has banned a user whom you believe is a legitimate user. 
- **forbidden words**: A list of words that, if detected, will alert or act on the user who sent the message.