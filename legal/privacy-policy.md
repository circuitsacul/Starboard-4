# Privacy Policy

Starboard tracks the least amount of information necessary to function. This is a list of what data is stored and why.

Only a select few have access to the bots database and cache. Note that data is not encrypted, and so is accessible by the VPS provider. Right now, the VPS provider is [https://netcup.eu](https://netcup.eu), located in Germany. Daily backups are also sent to a VPS from [https://alphavps.com](https://alphavps.com).

You can see a more detailed list of stored data, if you want, by looking at the source code for Starboard. The database structure can be viewed here: [https://github.com/CircuitSacul/Starboard-4/tree/main/src/database/models](https://github.com/CircuitSacul/Starboard-4/tree/main/src/database/models)

## Stored in Database

* Server IDs: Any server that uses or has used Starboard will have its ID stored. Necessary to track configuration.
* User IDs: Any user who has voted on a message, or has sent a message that was voted on, will have their ID stored. Necessary to track message ownership and vote ownership. A user's ID will also be stored if one of their messages is forced/frozen/trashed by a moderator.
* Message IDs: Any message that has been voted on, or has been trashed/frozen/forced. Necessary to track the origin of messages, so that the bot can find message content when it is needed.
* Channel IDs: Stored if the channel has a starboard or autostar channel in it. Also stored if the ID of a message in that channel is stored (Discord requires both the channel ID and the message ID to fetch message data).
* Reactions: If a reaction is added to a message, and that reaction is an upvote/downvote emoji for a starboard in that server, the reaction is stored. The emoji itself isn't stored - rather, the bot stores the reactor's ID, the message ID, and the message's author's ID. Necessary to track the votes a message has.
* Any data/settings that you explicitly give to Starboard via configuration (True/False settings, emojis, etc.)

## Stored Elsewhere

* Server Count: Starboard periodically provides the total number of servers Starboard is in to different bot lists. It does not provide specific servers.
* Approximate Server Member Count: This is data provided directly by Discord. Starboard periodically takes the approximate member count from all servers, adds them together, and posts this to bot lists. This is never stored per-server.

## Cached Data

Starboard stores info in-memory that it needs to access, to avoid unnecessary API calls. The cache was written from scratch to only store information that the bot really needs. In addition to the data normally stored in the database (IDs, for the most part), Starboard also caches:

* Guilds: ID, name
  * Channels: ID, parent channel ID (if any), type
  * Roles: ID, position, name
  * Emojis: ID, is-animated
* Messages (up to 50k): ID, content (including file URLs), author, replied-to message
* Users (up to 50k): is-bot, name, avatar URL
* Members (up to 50k): user ID, guild ID, nickname, server avatar URL, roles

The cache structure and data stored can also be viewed in the source code here: [https://github.com/CircuitSacul/Starboard-4/blob/main/src/cache/cache\_struct.rs](https://github.com/CircuitSacul/Starboard-4/blob/main/src/cache/cache\_struct.rs)
