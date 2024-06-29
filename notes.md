
# Undocumented gateway updates (user tokens)

## PASSIVE_UPDATE_V2
```json
{
  "d": {
    "guild_id": "...",
    "removed_voice_states": [],
    "updated_channels": [
      {
        "id": "...",
        "last_message_id": "...",
        "last_pin_timestamp": "iso8601 format"
      },
      {
        "id": "...",
        "last_message_id": "..."
      }
    ],
    "updated_members": [],
    "updated_voice_states": []
  },
  "op": 0,
  "s": 3,
  "t": "PASSIVE_UPDATE_V2"
}
```

## READY_SUPPLEMENTAL
```json
just dont even worry about it its not worth it. marked as an empty event for now.
```

## SESSIONS_REPLACE
["essentially sends an update to your client of your other connected clients and their statuses."](https://github.com/discord/discord-api-docs/discussions/6210)
```json
{
  "d": [
    {
      "activities": [],
      "client_info": {
        "client": "desktop",
        "os": "windows",
        "version": 0
      },
      "session_id": "43...2e",
      "status": "idle"
    },
    {
      "activities": [],
      "client_info": {
        "client": "web",
        "os": "windows",
        "version": 0
      },
      "session_id": "8e...bc",
      "status": "idle"
    }
  ],
  "op": 0,
  "s": 3,
  "t": "SESSIONS_REPLACE"
}
```

## CONVERSATION_SUMMARY_UPDATE
What does this do.
```json
{
  "d": {
    "channel_id": "...",
    "guild_id": "...",
    "summaries": []
  },
  "op": 0,
  "s": 8,
  "t": "CONVERSATION_SUMMARY_UPDATE"
}
```