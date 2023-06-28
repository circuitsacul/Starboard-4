# Setting Overrides

Overrides allow you to have different settings for a starboard based on the channel. For example, you can change the number of stars required for only one channel, instead of for the whole starboard.

All starboard options can be overwritten with overrides, except for private. A list of options can be found at [options.md](starboards/options.md "mention").

## Commands

| /overrides view                                          | View all overrides or the settings for a specific override.                                                                                  |
| -------------------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------- |
| /overrides create                                        | Create an override for a starboard in a specific channel set. The copy-from option let's you specify another override to copy settings from. |
| /overrides delete                                        | Delete an override.                                                                                                                          |
| /overrides edit \[behavior\|embed\|style\|requirements\| | Change the settings for an override.                                                                                                         |
| /overrides edit reset                                    | Reset certain options to their defaults for the starboard.                                                                                   |
| /overrides rename                                        | Rename an override.                                                                                                                          |
| /overrides channels set                                  | Set the channels that an override applies to.                                                                                                |
| /overrides channels remove                               | Remove channels from an override.                                                                                                            |
| /overrides channels add                                  | Add channels to an override.                                                                                                                 |
