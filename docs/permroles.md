# PermRoles

PermRoles work very similarly to Discord's role permissions system. For each PermRole, you can change their "permissions". Each permission can either be True (enabled), False (disabled), or None (default). If a permission is None, it simply carries from the last PermRole.

PermRoles are ordered in the same way your Discord roles are ordered, and are applied starting with the @everyone role and ending with the highest up role.

In addition to setting global permissions, you can also set per-starboard permissions for each role.

## Commands

| /permroles view           | View the PermRoles for a server.                             |
| ------------------------- | ------------------------------------------------------------ |
| /permroles create         | Create a PermRole.                                           |
| /permroles delete         | Delete a PermRole.                                           |
| /permroles clear-deleted  | Delete PermRoles where the role was deleted from the server. |
| /permroles edit           | Edit the permissions for a PermRole.                         |
| /permroles edit-starboard | Edit a PermRoles starboard-specific permissions.             |

## Permissions

| vote          | Whether to allow members with this role to vote on messages.     |
| ------------- | ---------------------------------------------------------------- |
| receive-votes | Whether messages sent by members with this role can be voted on. |
| gain-xproles  | Whether mebers with this role can gain XPRoles.                  |
