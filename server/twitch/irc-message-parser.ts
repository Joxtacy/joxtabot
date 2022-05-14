interface Tags {
  badges?: {
    staff?: string;
    broadcaster?: string;
    turbo?: string;
  };
  color?: string;
  "display-name"?: string;
  "emote-only"?: string;
  emotes?: {
    [index: number]: {
      startPosition: string;
      endPosition: string;
    }[];
  };
  id?: string;
  mod?: string;
  "room-id"?: string;
  subscriber?: string;
  turbo?: string;
  "tmi-sent-ts"?: string;
  "user-id"?: string;
  "user-type"?: string;
}

interface Source {
  nick: string | null;
  host: string;
}

const Commands = {
  CAP: "CAP",
  CLEARCHAT: "CLEARCHAT",
  GLOBALUSERSTATE: "GLOBALUSERSTATE",
  HOSTTARGET: "HOSTTARGET",
  JOIN: "JOIN",
  NOTICE: "NOTICE",
  PART: "PART",
  PING: "PING",
  PRIVMSG: "PRIVMSG",
  RECONNECT: "RECONNECT",
  ROOMSTATE: "ROOMSTATE",
  USERSTATE: "USERSTATE",
  ["001"]: "001",
  ["002"]: "002",
  ["003"]: "003",
  ["004"]: "004",
  ["353"]: "353",
  ["366"]: "366",
  ["372"]: "372",
  ["375"]: "375",
  ["376"]: "376",
  ["421"]: "421",
} as const;

interface Command {
  command?: string;
  channel?: string;
  isCapRequestEnabled?: boolean;
  botCommand?: string;
  botCommandParams?: string;
}

interface TextPosition {
  startPosition: number;
  endPosition: number;
}

type Parameters = string;

export const parseMessage = (message: string) => {
  const parsedMessage: {
    tags: Tags | null;
    source: Source | null;
    command: Command | null;
    parameters: Parameters | null;
  } = { // Contains the component parts.
    tags: null,
    source: null,
    command: null,
    parameters: null,
  };

  // The start index. Increments as we parse the IRC message.

  let idx = 0;

  // The raw components of the IRC message.

  let rawTagsComponent: string | null = null;
  let rawSourceComponent: string | null = null;
  let rawCommandComponent: string | null = null;
  let rawParametersComponent: string | null = null;

  // If the message includes tags, get the tags component of the IRC message.

  if (message[idx] === "@") { // The message includes tags.
    const endIdx = message.indexOf(" ");
    rawTagsComponent = message.slice(1, endIdx);
    idx = endIdx + 1; // Should now point to source colon (:).
  }

  // Get the source component (nick and host) of the IRC message.
  // The idx should point to the source part; otherwise, it's a PING command.

  if (message[idx] === ":") {
    idx += 1;
    const endIdx = message.indexOf(" ", idx);
    rawSourceComponent = message.slice(idx, endIdx);
    idx = endIdx + 1; // Should point to the command part of the message.
  }

  // Get the command component of the IRC message.

  let endIdx = message.indexOf(":", idx); // Looking for the parameters part of the message.
  if (endIdx === -1) { // But not all messages include the parameters part.
    endIdx = message.length;
  }

  rawCommandComponent = message.slice(idx, endIdx).trim();

  // Get the parameters component of the IRC message.

  if (endIdx !== message.length) { // Check if the IRC message contains a parameters component.
    idx = endIdx + 1; // Should point to the parameters part of the message.
    rawParametersComponent = message.slice(idx);
  }

  // Parse the command component of the IRC message.

  parsedMessage.command = parseCommand(rawCommandComponent);

  // Only parse the rest of the components if it's a command
  // we care about; we ignore some messages.

  if (parsedMessage.command === null) { // Is null if it's a message we don't care about.
    return null;
  } else {
    if (rawTagsComponent !== null) {
      parsedMessage.tags = parseTags(rawTagsComponent);
    }

    parsedMessage.source = parseSource(rawSourceComponent);

    parsedMessage.parameters = rawParametersComponent !== null
      ? rawParametersComponent.trim()
      : null;
    if (rawParametersComponent?.[0] === "!") {
      // The user entered a bot command in the chat window.
      parsedMessage.command = parseParameters(
        rawParametersComponent,
        parsedMessage.command,
      );
    }
  }
  return parsedMessage;
};

/**
 * Parses the tags component of the IRC message.
 *
 * Example: badge-info=;badges=broadcaster/1;color=#0000FF;...
 */
const parseTags = (rawTags: string) => {
  const tagsToIgnore = { // List of tags to ignore.
    "client-nonce": null,
    flags: null,
  };

  type ParsedTags = Record<
    string,
    | Record<string, string>
    | Record<string, TextPosition[]>
    | string[]
    | string
    | null
  >;

  const dictParsedTags: ParsedTags = {}; // Holds the parsed list of tags.
  // The key is the tag's name (e.g., color).
  const parsedTags = rawTags.split(";");

  parsedTags.forEach((tag) => {
    const parsedTag = tag.split("="); // Tags are key/value pairs.
    const tagValue = (parsedTag[1] === "") ? null : parsedTag[1];

    switch (parsedTag[0]) { // Switch on tag name.
      case "badges":
      case "badge-info": {
        // badges=staff/1,broadcaster/1,turbo/1;

        if (tagValue) {
          const dict: Record<string, string> = {}; // Holds the list of badge objects.
          // The key is the badge's name (e.g., subscriber).
          const badges = tagValue.split(",");
          badges.forEach((pair) => {
            const badgeParts = pair.split("/");
            dict[badgeParts[0]] = badgeParts[1];
          });
          dictParsedTags[parsedTag[0]] = dict;
        } else {
          dictParsedTags[parsedTag[0]] = null;
        }
        break;
      }
      case "emotes": {
        // emotes=25:0-4,12-16/1902:6-10

        if (tagValue) {
          const dictEmotes: Record<string, TextPosition[]> = {}; // Holds a list of emote objects.
          // The key is the emote's ID.
          const emotes = tagValue.split("/");
          emotes.forEach((emote) => {
            const emoteParts = emote.split(":");

            const textPositions: TextPosition[] = []; // The list of position objects that identify
            // the location of the emote in the chat message.
            const positions = emoteParts[1].split(",");
            positions.forEach((position) => {
              const positionParts = position.split("-");
              textPositions.push({
                startPosition: Number(positionParts[0]),
                endPosition: Number(positionParts[1]),
              });
            });

            dictEmotes[emoteParts[0]] = textPositions;
          });

          dictParsedTags[parsedTag[0]] = dictEmotes;
        } else {
          dictParsedTags[parsedTag[0]] = null;
        }

        break;
      }
      case "emote-sets": {
        // emote-sets=0,33,50,237

        if (tagValue) {
          const emoteSetIds = tagValue.split(",");
          dictParsedTags[parsedTag[0]] = emoteSetIds;
        }
        break;
      }
      default: {
        // If the tag is in the list of tags to ignore, ignore
        // it; otherwise, add it.

        if (!Object.hasOwn(tagsToIgnore, parsedTag[0])) {
          dictParsedTags[parsedTag[0]] = tagValue;
        }
      }
    }
  });

  return dictParsedTags;
};

const parseCommand = (rawCommand: string): Command | null => {
  let parsedCommand = null;
  const commandParts = rawCommand.split(" ");

  switch (commandParts[0]) {
    case Commands.JOIN:
    case Commands.PART:
    case Commands.NOTICE:
    case Commands.CLEARCHAT:
    case Commands.HOSTTARGET:
    case Commands.PRIVMSG: {
      parsedCommand = {
        command: commandParts[0],
        channel: commandParts[1],
      };
      break;
    }
    case Commands.PING: {
      parsedCommand = {
        command: commandParts[0],
      };
      break;
    }
    case Commands.CAP: {
      parsedCommand = {
        command: commandParts[0],
        isCapRequestEnabled: (commandParts[2] === "ACK"),
        // The parameters part of the messages contains the
        // enabled capabilities.
      };
      break;
    }
    case Commands.GLOBALUSERSTATE: { // Included only if you request the /commands capability.
      // But it has no meaning without also including the /tags capability.
      parsedCommand = {
        command: commandParts[0],
      };
      break;
    }
    case Commands.USERSTATE: // Included only if you request the /commands capability.
    case Commands.ROOMSTATE: { // But it has no meaning without also including the /tags capability.
      parsedCommand = {
        command: commandParts[0],
        channel: commandParts[1],
      };
      break;
    }
    case Commands.RECONNECT: {
      console.log(
        "The Twitch IRC server is about to terminate the connection for maintenance.",
      );
      parsedCommand = {
        command: commandParts[0],
      };
      break;
    }
    case Commands["421"]: {
      console.log(`Unsupported IRC command: ${commandParts[2]}`);
      return null;
    }
    case Commands["001"]: { // Logged in (successfully authenticated).
      parsedCommand = {
        command: commandParts[0],
        channel: commandParts[1],
      };
      break;
    }
    case Commands["002"]: // Ignoring all other numeric messages.
    case Commands["003"]:
    case Commands["004"]:
    case Commands["353"]: // Tells you who else is in the chat room you're joining.
    case Commands["366"]:
    case Commands["372"]:
    case Commands["375"]:
    case Commands["376"]: {
      console.log(`numeric message: ${commandParts[0]}`);
      return null;
    }
    default: {
      console.log(`\nUnexpected command: ${commandParts[0]}\n`);
      return null;
    }
  }

  return parsedCommand;
};

/**
 * Parses the source (nick and host) components of the IRC message.
 */
const parseSource = (rawSource: string | null): Source | null => {
  if (rawSource === null) {
    return null;
  } else {
    const sourceParts = rawSource.split("!");
    return {
      nick: (sourceParts.length === 2) ? sourceParts[0] : null,
      host: (sourceParts.length === 2) ? sourceParts[1] : sourceParts[0],
    };
  }
};

/**
 * Parses the IRC parameters component if it contains a command (e.g., !dice).
 */
const parseParameters = (rawParameters: string, command: Command): Command => {
  const idx = 0;
  const commandParts = rawParameters.slice(idx + 1).trim();
  const paramsIdx = commandParts.indexOf(" ");

  if (paramsIdx === -1) {
    command.botCommand = commandParts.slice(0);
  } else {
    command.botCommand = commandParts.slice(0, paramsIdx);
    command.botCommandParams = commandParts.slice(paramsIdx).trim();
    // TODO: remove extra spaces in parameters string
  }

  return command;
};
