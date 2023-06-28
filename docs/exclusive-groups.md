# Exclusive Groups

Exclusive groups are a way to make starboards _exclusive_ - meaning that a message can only be on one starboard at a time. Example use-cases:

* You have multiple starboards to categorize messages, and a message should only be on the first starboard it reaches
* A reporting channel - if a message receives enough ⚠️ (or whatever emojis you want) reactions, then it will be removed from all starboards and sent to the reports starboard
* Leveling starboards - you have multiple tiers of starboards. Messages with 5-10 stars are on starboard-low, messages with 10-20 stars are on starboard-med, and so on.

Exclusive works have to "settings".

* Starboards in the group
* The priority of each starboard

The priority of a starboard determines what happens to posts on other starboards in the same group. If a starboard has a higher priority than another, it will override lower-priority starboards. If your starboards have equal priorities, then the message will stay on the first starboard it is sent to.

## Commands

| /exclusive-groups create | Create a group |
| ------------------------ | -------------- |
| /exclusive-groups delete | Delete a group |
| /exclusive-groups rename | Rename a group |

To add starboards to a group and change its priority, use the `/starboards edit behavior` command (or `/overrides edit behavior` command for overrides).
