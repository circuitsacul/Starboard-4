# Options

These are all the settings that can be configured for each starboard and override.

## Style

| display-emoji       | The emoji shown next to the number of points on a starboard post. ⭐️ by default.                                                                                                                                                                                                               |
| ------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| ping-author         | Whether to mention the author of the original message when it appears on the starboard. False by default.                                                                                                                                                                                      |
| user-server-profile | Whether to use a users per-server avatar and nickname, rather than their default avatar and username. True by default.                                                                                                                                                                         |
| extra-embeds        | Whether to include extra embeds when the original message also has embeds. True by default.                                                                                                                                                                                                    |
| go-to-message       | <p>The style of the message link.</p><ul><li>None: no message link.</li><li>Link: puts a hyperlink inside the embed.</li><li>Button: puts the link in a button below the message.</li><li>Mention: replaces the channel mention with a message mention. This is the default setting.</li></ul> |
| use-webhook         | Whether to use a webhook to send starboard posts, rather than the bot account. Doing this will allow you to customize the avatar and username of the starboard posts. False by default.                                                                                                        |

## Embed

| color           | The color of the embed on starboard posts. #FFE19C by default.                                   |
| --------------- | ------------------------------------------------------------------------------------------------ |
| attachment-list | Whether to list the names (as hyperlinks) of uploaded attachments. True by default.              |
| replied-to      | Whether to include the message that was replied to, if any, in starboard posts. True by default. |

## Requirements

| required        | The minimum number of upvotes a message needs to be sent to the starboard. 3 by default.             |
| --------------- | ---------------------------------------------------------------------------------------------------- |
| required-remove | The minimum number of upvotes a post can have before it is removed from the starboard. 0 by default. |
| upvote-emojis   | The emojis that can be used to upvote a message. ⭐️ by default.                                      |
| downvote-emojis | The emojis that can be used to downvote a post. None by default.                                     |
| self-vote       | Whether to allow users to vote on their own posts. False by default.                                 |
| allow-bots      | Whether to allow bot messages to be voted on. True by default.                                       |
| require-image   | Whether to require messages to have an image to be voted on. False by default.                       |
| older-than      | Only messages older than this can be voted on (e.g. 1 hour). Disabled by default.                    |
| newer-than      | Only messages newer than this can be voted on (e.g. 1 week). Disabled by default.                    |

## Behavior

| autoreact-upvote         | Whether to automatically react to posts on the starboard with the upvote-emojis. True by default.                                                                                                                            |
| ------------------------ | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| autoreact-downvote       | Whether to automatically react to posts on the starboard with the downvote-emojis. True by default.                                                                                                                          |
| remove-invalid-reactions | Whether to remove votes that don't meet requirements (e.g. self votes). True by default.                                                                                                                                     |
| link-deletes             | Whether to delete a starboard post if the original message was deleted. False by default.                                                                                                                                    |
| link-edits               | Whether to update the content of a starboard post with new content if the original is edited. True by default.                                                                                                               |
| on-delete                | What to do if a moderator deletes a message from the starboard. Can be "Refresh" (default), "Ignore", "Trash All", or "Freeze All".                                                                                          |
| xp-multiplier            | How much XP to give someone fro every vote they receive (1 for upvotes, -1 for downvotes). If you set this to negative, then downvotes will cause you to _gain_ XP, while upvotes will cause you to _lose_ XP. 1 by default. |
| cooldown-enabled         | Whether to enable the vote cooldown for this starboard.                                                                                                                                                                      |
| cooldown                 | The value of the cooldown. Represents how many times you can vote per a certain number of seconds (e.g. "3/10", which means 3 votes per 10 seconds).                                                                         |
| private                  | Whether to hide this starboard's posts from /moststarred and /random. False by default.                                                                                                                                      |
| exclusive-group          | The exclusive group this starboard belongs to.                                                                                                                                                                               |
| exclusive-group-priority | The priority this starboard has inside the exclusive group, if any.                                                                                                                                                          |
