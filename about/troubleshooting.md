# Troubleshooting

Below is a list of common problems and questions we get in the support server. **Before going through this list** please make sure you have the right bot/docs. If the bot you're using is "Starboard#9387", then you're in the right place. Note that there are other bots with the name "Starboard."

If none of this helps, feel free to join the [support server](https://discord.gg/3gK8mSA).

## Starboard is offline in my server, but not others

There seems to be an issue where this happens frequently to some servers. I'm not sure if it's my fault or Discord's, but I'm not the only developer running into this.

If you join the [support server](https://discord.gg/3gK8mSA), I can restart the bot, which usually fixes the problem.

It would also be nice to get a list of servers that this is happening in, so if you're willing, it would be nice if you could send me the ID of your server as well.

## Starboard isn't responding

First, try pinging the bot. If it responds, it should tell you it's prefix. If it doesn't, then it probably doesn't have the "View Channel" permission in that channel. An easy way to check if the problem is because of broken permissions is to (temporarily) give the bot admin. If it still doesn't work, there's like something else going.

## Messages aren't showing up on the starboard

1. Make sure that the message that isn't showing up isn't in an NSFW channel. If it is, note that the message can **only** show up on a starboard if that starboard is also marked as NSFW.
2. Check that the bot has permission to send messages in the starboard. You can test this by mentioning the bot inside the starboard channel.
3. Try running `/utils info` with a link to the message that isn't working.
   1. If the bot says it can't find the message, that means it doesn't have permission to view the channel or its history.
   2. Otherwise, look at the stats for the starboard it isn't working on. It should say something like "**2**/3", which means that the bot has recorded 2 upvotes, but 3 are required.
