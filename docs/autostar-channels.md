# AutoStar Channels

Autostar channels are channels where Starboard will automatically react to any message sent there.

## Commands

| /autostar view   | View all of your autostar channels, or the settings for a specific one. |
| ---------------- | ----------------------------------------------------------------------- |
| /autostar create | Create an autostar channel.                                             |
| /autostar delete | Delete an autostar channel.                                             |
| /autostar edit   | Edit the settings for an autostar channel.                              |
| /autostar rename | Rename an autostar channel                                              |

## Options

| emojis         | A list of emoijs that Starboard will react with when messages are sent.                  |
| -------------- | ---------------------------------------------------------------------------------------- |
| min-chars      | The minimum number of characters a message needs.                                        |
| max-chars      | The maximum number of characters a message can have. Set to -1 to disable.               |
| require-image  | Whether to require an image on messages.                                                 |
| delete-invalid | Whether to delete messages that don't meet requirements, rather than just ignoring them. |
