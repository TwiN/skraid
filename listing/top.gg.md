**NOTE:** By default, Skraid is in alert-only mode. Until you configure an alert channel using `s!set_alert_channel` or you disable alert-only mode using `s!set_alert_only false`, the bot will not serve any purpose.

**Please read "Important notes" and "Getting started" before inviting the bot to your server.**


Skraid - a word play on scram, scam and raid - is an attempt to harness the power of large communities in order to put an end, or at least to reduce the blast radius of raids and common scams by ensuring that scammers and users known to take part in raids are banned before they can cause any harm.

This is, however, easier said than done.

This bot is built with several fail-safe mechanisms to prevent abuse.

One of these fail-safe mechanisms is the fact that inviting the bot to a guild does not allow you to modify the global ban list. The staff members of each guild (i.e. users with BAN_MEMBERS permissions) may suggest the addition of users to the global ban list by using `s!suggest_blocklist USER_ID`, but the decision is ultimately up to several factors which will not be disclosed to prevent malicious actors from attempting to circumvent the system. Do not be too worried, though, as one of the measures put in place requires manual action from the maintainer of the bot.

## Features
- Send an alert if a user in the global blocklist has joined the server (or ban said user, if configured to do so). Does not affect users that were already in the server.
- Supports per-server list of "exceptions" (allowlist), in case a guild wishes to let a user in the global ban list (blocklist) join their server anyways. This only really applies if the bot is configured to ban instead of alert.
- Detect messages containing known phishing/scam links and send an alert, or delete said messages if configured to do so.
- Configuration for setting up a channel for alerts, including replacing all actions by alerts sent to said channel.
- Has some utility functions to manage raids


## Important notes
Before providing the instructions to get started with Skraid, there are some things that you must be made aware of.

As good as the promise of a Discord with no scam or raid sounds, having a bot managing a global ban list to which they have no access to may make server owners of large communities worried.

This is completely understandable, and that is why **by default, Skraid is in alert-only mode**. This means that **no automated action will be taken out of the box outside of sending a message to a channel of your choice** (more on this in the Getting started section).

For the same reason, you may invite the bot with no "Ban members" nor "Manage messages" permissions, effectively making the bot harmless to your community.


## Getting started
By default, Skraid will be on alert-only mode.

This means that until you create a channel and configure Skraid to send alerts to that channel, it will not do anything.

Once you've created the alert channel, make sure that Skraid has access to the channel and type the following:
```
s!set_alert_channel <ALERT_CHANNEL_ID>
```
Where `<ALERT_CHANNEL_ID>` is the channel ID of your alert channel (e.g. `860216911907298444`)

If you wish to let Skraid ban members in the global ban list, you may disable alert-only mode by using the following command:
```
s!set_alert_only false
```

If the alert-only mode is set to false, you do not technically need an alert channel, but it is strongly recommended.



## Commands
All commands must be prefixed by the `COMMAND_PREFIX`, or `s!` by default.

**NOTE:** Only users with ban permissions may use these commands.


### Configuration
| Command name      | Description |
|:----------------- |:----------- |
| get_guild_config  | Retrieve the current guild configuration.
| set_alert_channel | Configure an alert channel by passing the desired channel id as argument
| set_alert_only    | Configure Skraid's mode.By default, this is set to true.

**NOTE**: Setting `set_alert_only` to false will cause any user in the global blocklist to be banned as soon as they join the server. Furthermore, it will also cause the deletion of new messages containing one or more forbidden word (e.g. a link known to be related to phishing). In any case, alerts will always be sent as long as the alert channel is configured.


### Allowlist
While each individual guild may only "suggest" the addition of user ids to the global ban list, you have full control over your own server: If you believe that a user has been wrongly added to the global blocklist, you may add them to your guild's allowlist, or later remove them if you changed your mind.

Each guild has their own separate allowlist which, in the case that they chose to enable Skraid's automatic banning capabilities, will override the global blocklist.

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



## FAQ
### Will all users in the global ban list be banned as soon as I invite the bot in my server and set the alert_only mode to false?
No. Not only is alert_only is set to true by default, the bot does not monitor the existing list of users; it only monitors users as they join the server.

### How efficient is Skraid?
What you need to understand is that this bot will do good only if large communities are willing to give it a try, since they are the ones who are the most often targeted by raiders and scammers. As such, the efficiency of Skraid will increase over time.


## Glossary
- **blocklist**: A global ban list maintained by the bot's maintainer.
- **allowlist**: A per-guild list of user ids. Used only in case Skraid has banned a user whom you believe is a legitimate user.
- **forbidden words**: A list of words that, if detected, will send an alert to the configured alert channel and, optionally, delete the message.