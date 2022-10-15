## Twitch webhook message examples

```json
// Webhook subscription verification challenge example
{
  "challenge": "pogchamp-kappa-360noscope-vohiyo",
  "subscription": {
    "id": "f1c2a387-161a-49f9-a165-0f21d7a4e1c4",
    "status": "webhook_callback_verification_pending",
    "type": "channel.follow",
    "version": "1",
    "cost": 1,
    "condition": {
      "broadcaster_user_id": "12826"
    },
    "transport": {
      "method": "webhook",
      "callback": "https://example.com/webhooks/callback"
    },
    "created_at": "2019-11-16T10:11:12.123Z"
  }
}
```

```json
// Webhook subscription revoked example
{
  "subscription": {
    "id": "f1c2a387-161a-49f9-a165-0f21d7a4e1c4",
    "status": "authorization_revoked",
    "type": "channel.follow",
    "cost": 1,
    "version": "1",
    "condition": {
      "broadcaster_user_id": "12826"
    },
    "transport": {
      "method": "webhook",
      "callback": "https://example.com/webhooks/callback"
    },
    "created_at": "2019-11-16T10:11:12.123Z"
  }
}
```

```json
// Redeem reward example.
{
  "subscription": {
    "id": "62a04e8f-5c47-acfa-6895-274d679abf85",
    "status": "enabled",
    "type": "channel.channel_points_custom_reward_redemption.add",
    "version": "1",
    "condition": {
      "broadcaster_user_id": "54605357"
    },
    "transport": {
      "method": "webhook",
      "callback": "null"
    },
    "created_at": "2022-07-13T04:03:22.577859Z",
    "cost": 0
  },
  "event": {
    "id": "62a04e8f-5c47-acfa-6895-274d679abf85",
    "broadcaster_user_id": "54605357",
    "broadcaster_user_login": "testBroadcaster",
    "broadcaster_user_name": "testBroadcaster",
    "user_id": "91240126",
    "user_login": "testFromUser",
    "user_name": "testFromUser",
    "user_input": "Test Input From CLI",
    "status": "unfulfilled",
    "reward": {
      "id": "c3f5c149-65e9-4bc0-2271-723f41731542",
      "title": "Test Reward from CLI",
      "cost": 150,
      "prompt": "Redeem Your Test Reward from CLI"
    },
    "redeemed_at": "2022-07-13T04:03:22.577859Z"
  }
}
```
