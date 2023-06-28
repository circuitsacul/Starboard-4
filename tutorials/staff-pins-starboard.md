# Staff Pins Starboard

A staff-pins starboard is a starboard that only staff can send messages to. If someone without the @Staff role tries to react with 📌, it'll just be removed - but when a staff reacts with it, Starboard will repost the message to your staff-pins starboard.

1. Create a channel called "staff-pins".
2. If you don't have one already, create a @Staff role for all the staff of your server.
3. Run "/starboards create channel: #staff-pins name: staff-pins".
4. Run "/starboards edit requirements starboard: staff-pins self-vote: True required: 1 upvote-emojis: 📌"
5. Run "/permroles create role: @everyone".
6. Run "/permroles edit-starboard permrole: @everyone starboard: staff-pins vote: False".
7. Run "/permroles create role: @Staff".
8. Run "/permroles edit-starboard permrole: @Staff starboard: staff-pins vote: True".
