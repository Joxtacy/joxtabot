export enum TwitchIrcMessageType {
  CAPACK = "CAP * ACK",
  CLEARCHAT = "CLEARCHAT",
  CLEARMSG = "CLEARMSG",
  HOSTTARGET = "HOSTTARGET",
  JOIN = "JOIN",
  NOTICE = "NOTICE",
  PART = "PART",
  PING = "PING",
  PRIVMSG = "PRIVMSG",
  RECONNECT = "RECONNECT",
  ROOMSTATE = "ROOMSTATE",
  USERNOTICE = "USERNOTICE",
  USERSTATE = "USERSTATE",
  INIT = "INIT", // Custom type used internally
  NAMES = "NAMES", // Custom type used internally
}

export enum TwitchPrivmsgIrcTags {
  BADGES = "badges",
  BADGE_INFO = "badge-info",
  BITS = "bits",
  CLIENT_NONCE = "client-nonce",
  COLOR = "color",
  DISPLAY_NAME = "display-name",
  EMOTES = "emotes", // Emotes needs a different parsing.
  EMOTE_SETS = "emote-sets",
  FLAGS = "flags",
  ID = "id",
  MOD = "mod",
  ROOM_ID = "room-id",
  SUBSCRIBER = "subscriber",
  TMI_SENT_TS = "tmi-sent-ts",
  TURBO = "turbo",
  USER_ID = "user-id",
  USER_TYPE = "user-type",
}

export interface TwitchIrcMessageMetadata {
  tags: Map<string, string[]> | null;
  channel: string | null;
  username: string | null;
}

export interface TwitchIrcMessage {
  type: TwitchIrcMessageType;
  message: string | null;
  metadata: TwitchIrcMessageMetadata;
}
