# Premium

Starboard premium works with "credits". Each US $ you spend is equivalent to 1 credit for you. Once you reach 3 credits, you can redeem 1 month of premium in 1 server. So, to have premium in one server, it would cost $3/month.

Currently, the only way to get credits is by becoming a patron: [https://patreon.com/CircuitSacul](https://patreon.com/CircuitSacul)

## Perks

* Up to 20 starboards instead of 3.
* Up to 50 autostar channels instead of 3.
* Up to 20 emojis per starboard instead of 3.
* Up to 20 emojis per auotstar channel instead of 3.
* Access to regex matching for starboards/filters
* Autostar 100 messages per 10 seconds (rather than 4 messages per 10 seconds)
* Access to award roles (XPRoles and PosRoles)
* Starboard will upload attachments that can't be embedded (assuming they're under 8mb)
* Remove "Powered by starboard.best" footer on webhook starboards
* @Supporter and @Patron roles in Discord serve

## Setup

Once you've become a patron, Starboard will add premium credits to your discord account (this may take up to 30 minutes).

| /premium info   | Show info on premium, how much premium is left for a server, and how many credits you have. |
| --------------- | ------------------------------------------------------------------------------------------- |
| /premium redeem | Redeem premium for a server.                                                                |

### AutoRedeem

If you don't want to run the command every month, you can enable autoredeem `/premium autoredeem enable`. If this is enabled, then whenever your server runs out of premium, Starboard will automatically try to redeem another month of premium using your credits.

Autoredeem will only take credits from one member who has autoredeem enabled per server, and will only work if you are actually in the server.

| /premium autoredeem enable  | Enable autoredeem for a server.  |
| --------------------------- | -------------------------------- |
| /premium autoredeem disable | Disable autoredeem for a server. |

## Locks

Locks are added to starboards and autostar-channels if premium expires on a server, and that server has more starboards/autostar-channels than the non-premium limit.

Locks can be moved between starboards/autostar-channels, and will automatically disappear if premium is re-enabled, or if a starboard/autostar-channel is deleted.

| /premium-locks refresh        | Refresh the locks for your server.                |
| ----------------------------- | ------------------------------------------------- |
| /premium-locks move-autostar  | Move a lock from one autostar channel to another. |
| /premium-locks move-starboard | Move a lock from one starboard to another.        |

