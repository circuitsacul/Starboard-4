# Position-based Award Roles

Position-based Award Roles (referred to as PosRoles) are roles that have a limited "membership." For each role, you can set a number of members, and Starboard will automatically assign these roles based on a members position on the leaderboard. For example, you might have something like this:

* @Super Star: 1 member
* @Bright Star: 10 members
* @Star: 100 members

This would mean that the first person on the leaderboard would receive the @Super Star role, the next 10, the @Bright Star role, and the next 100, the @Star role.

## Commands

| /posroles set-max-members | Create a PosRole, and/or set the maximum members for it. |
| ------------------------- | -------------------------------------------------------- |
| /posroles delete          | Delete a PosRole.                                        |
| /posroles clear-deleted   | Delete all PosRoles where the Discord role was deleted.  |
| /posroles view            | View all your PosRoles.                                  |
| /posroles refresh         | Refresh the PosRole assignments for your server.         |
