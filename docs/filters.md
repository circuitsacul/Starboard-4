# Filters

{% hint style="info" %}
Filters are a beta feature and may be buggy. Things may change at any time, including conditions being dropped or filters being removed all together.
{% endhint %}

{% hint style="info" %}
If you have any problems with filters, or any suggestions, please open a thread in #feedback or #support in the [support server](https://discord.gg/3gK8mSA).
{% endhint %}

Filters are a new, more flexible way to control what messages can be starred or sent to autostar channels. Before reading, it's a good idea to familiarize yourself with the terminology used:

* Filter Group: A filter group is a collection of filters. A single filter group can be applied to a starboard or an autostar channel. A filter group is identified by its name.
* Filter: A filter is a collection of conditions. All filters are inside a filter group (they cannot be by themselves), and are identified by their position within that group (e.g. 1 or 2).
* Filter Condition: A single condition of a filter, such as `user-has-all-of` (meaning that the user, or message author, must have all of a set of roles).

Currently, filters can only be applied to starboards and autostar channels. In the future, you may be able to use filters as a condition for whether an override should apply (rather than just applying overrides on a channel basis), as well as for custom leaderboards (another planned feature).

## Commands

| /filters view          | View the filter groups in the current server.          |
| ---------------------- | ------------------------------------------------------ |
| /filters create-group  | Create a new filter group.                             |
| /filters delete-group  | Delete a filter group.                                 |
| /filters rename-group  | Rename a filter group.                                 |
| /filters create-filter | Create a filter inside a filter group.                 |
| /filters delete-filter | Delete a filter inside a filter group.                 |
| /filters move-filter   | Change the position of a filter inside a filter group. |
| /filters edit          | Edit the conditions of a filter.                       |

## Conditions

The conditions for a filter are split into three categories (called contexts): Default, Message, and Vote. The reason for this is that some conditions (such as voter-has-all-of) make no sense in some situations, like autostar channels.

Currently, there are only two contexts that occur: starboards and autostar channels.

A starboard utilizes all of the contexts (Default, Message, and Vote). Autostar channels utilize only the first two (Default and Message). If a filter has conditions inside of a context that isn't considered for a certain situation, then it will be assumed to pass.

### Default Context

These are conditions that are always valid. These conditions concern a "user" - for autostar channels and starboards, this is the message author.

| user-has-all-of      | A list of roles that the user must have                                                                   |
| -------------------- | --------------------------------------------------------------------------------------------------------- |
| user-has-some-of     | A list of roles that the user must have at least one of                                                   |
| user-missing-all-of  | A list of roles that the user must not have (the user cannot have _any_ of these roles)                   |
| user-missing-some-of | A list of roles that the user must be missing at least one of (the user cannot have _all_ of these roles) |
| user-is-bot          | A condition that either requires the user to be a bot or be a human.                                      |

### Message Context

These are conditions that are valid when a message is being considered. Both autostar channels and starboards consider this context.

| in-channel                     | A list of channels that the message must be in.                                         |
| ------------------------------ | --------------------------------------------------------------------------------------- |
| not-in-channel                 | A list of channels that the message cannot be in.                                       |
| in-channel-or-sub-channels     | A list of channels that the message must be in, including that channels sub-channels.   |
| not-in-channel-or-sub-channels | A list of channels that the message cannot be in, including that channels sub-channels. |
| min-attachments                | The message must have at least this many attachments.                                   |
| max-attachments                | The message must have at most this many attachments.                                    |
| min-length                     | The message must be at least this long.                                                 |
| max-length                     | The message must be at most this long.                                                  |
| matches                        | (Premium) the message must match this regex.                                            |
| not-matches                    | (Premium) the message must not match this regex.                                        |

### Vote Context

This context is considered for votes (reactions). Only starboards consider this context, everything else just assumes that these conditions pass.

| voter-has-all-of      | A list of roles that the voter must have                                                                    |
| --------------------- | ----------------------------------------------------------------------------------------------------------- |
| voter-has-some-of     | A list of roles that the voter must have at least one of                                                    |
| voter-missing-all-of  | A list of roles that the voter must not have (the voter cannot have _any_ of these roles)                   |
| voter-missing-some-of | A list of roles that the voter must be missing at least one of (the voter cannot have _all_ of these roles) |
| older-than            | The message must be older than this, at the time that the vote was added.                                   |
| newer-than            | The message must be newer than this, at the time that the vote was added.                                   |
