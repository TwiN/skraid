# skraid
[Docker Repository](https://hub.docker.com/r/twinproduction/skraid)

To invite the bot in the server: `https://discord.com/oauth2/authorize?client_id=<YOUR_BOT_CLIENT_ID>&scope=bot&permissions=11332`

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
Upon inviting Skraid in your server, you must create an alert channel where Skraid will send its messages to.

Once you've created the alert channel, make sure that Skraid has access to the channel and type the following:
```
s!SetAlertChannel <ALERT_CHANNEL_ID>
```
Where `<ALERT_CHANNEL_ID>` is the channel ID of your alert channel (e.g. `860216911907298444`)

If you wish Skraid to not automatically take actions, you may enable alert-only mode by using the following command:
```
s!SetAlertOnly true
```


## Features
- Detect when a user is spamming (and deletes the messages marked as spam if Skraid is not in alert-only mode).
- Send an alert if a user in the global blocklist has joined the server (or ban said user, if configured to do so). Does not affect users that were already in the server.
- Supports per-server list of "exceptions" (allowlist), in case a guild wishes to let a user in the global ban list (blocklist) join their server anyways. This only really applies if the bot is configured to ban instead of alert.
- Detect messages containing known phishing/scam links and send an alert, or delete said messages if configured to do so.
- Configuration for setting up a channel for alerts, including replacing all actions by alerts sent to said channel.
- Has some utility functions to manage raids

<video width="306" height="204" controls>
  <source src="https://raw.githubusercontent.com/TwinProduction/assets/master/anti-spam.mp4" type="video/mp4">
</video>


## Commands
All commands must be prefixed by the `COMMAND_PREFIX`, or `s!` by default.

### Configuration
| Command                   | Description |
|:------------------------- |:----------- |
| GetGuildConfig            | Retrieve the current guild configuration.
| SetAlertChannel           | Configure an alert channel by passing the desired channel id as argument
| SetAlertOnly              | Configure Skraid's mode. If set to false, if a user in the blocklist joins the server, they will be automatically banned. Likewise, if a user posts a message containing a forbidden word (e.g. a link known to be related to phishing), said message will be deleted. In any case, alerts will be sent as long as the alert channel is configured.
| SetBanNewUserOnJoin       | Configure whether Skraid should automatically ban users that were created less than two hours ago when they join the guild.
| SetBanNewUserOnJoin          | Configure whether Skraid should automatically ban every user that joins the guild. Used for when your guild is actively being raided.

### User Allowlist
Each guild has their own separate allowlist which, in the case that they chose to enable Skraid's automatic banning capabilities,
can allow a user they believe was wrongly blocklisted to join the server. 

If `alert_only` is set to `true`, there's no need to use the allowlist as the only thing you'd be
preventing is the creation of an alert.

| Command               | Description |
|:--------------------- |:----------- |
| UserAllowList add     | Add user ID to the guild's user allowlist
| UserAllowList remove  | Remove user ID from the guild's user allowlist
| UserAllowList search  | Check if a user ID is in the guild's user allowlist
| UserAllowList list    | Retrieve a list of all allowlisted user ids for this guild

### Suggestion
Suggestions are the mean by which guild staff members may communicate with the maintainer.

| Command                | Description |
|:---------------------- |:----------- |
| Suggest WordBlocklist  | Suggest a word to add to the list of global forbidden words to the maintainer
| Suggest UserBlocklist  | Suggest the addition of a user ID to the global blocklist to the maintainer.

### Utilities
| Command      | Description |
|:------------ |:----------- |
| Clear        | Clear N messages from the current channel
| Status       | Check the status of the bot

### Maintainer
| Command                  | Description |
|:------------------------ |:----------- |
| UserBlocklist add        | Add user ID to the user blocklist
| UserBlocklist remove     | Remove user ID to the user blocklist
| UserBlocklist search     | Check if user ID is in the user blocklist
| WordBlocklist add        | Add word in the word blocklist
| WordBlocklist remove     | Remove word in the word blocklist
| WordBlocklist search     | Checks if a string contains a word from the word blocklist
| WordBlocklist list       | Retrieve a list of all words in word blocklist


## Glossary
- **user blocklist**: A global list maintained by the bot's maintainer.
- **user allowlist**: A per-guild list of user ids. Used only in case Skraid has flagged or banned a user whom you believe is a legitimate user.
- **word blocklist**: A list of words that, if detected, will send an alert to the configured alert channel and, optionally, delete the message. These are mostly links of known scam websites.