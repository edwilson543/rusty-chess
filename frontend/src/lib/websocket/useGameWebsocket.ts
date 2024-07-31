import { useState, useEffect } from "react";

import useWebSocket, { ReadyState } from "react-use-websocket";
import { EventFromLogic } from "xstate";

import { newGameMessage } from "./inboundMessageSchema.ts";
import { GameMachineContext } from "../../context.ts";
import { GameTypes, GameMachine } from "../../machines/game";

export const useGameWebSocket = () => {
  const socketUrl = "ws://127.0.0.1:8000/api/play/";
  const [messageHistory, setMessageHistory] = useState<MessageEvent<string>[]>(
    [],
  );

  const { sendMessage, lastMessage, readyState } = useWebSocket(socketUrl);
  const connectionStatus = {
    [ReadyState.CONNECTING]: "Connecting",
    [ReadyState.OPEN]: "Open",
    [ReadyState.CLOSING]: "Closing",
    [ReadyState.CLOSED]: "Closed",
    [ReadyState.UNINSTANTIATED]: "Uninstantiated",
  }[readyState];

  const gameMachineRef = GameMachineContext.useActorRef();

  useEffect(() => {
    if (lastMessage !== null) {
      forwardMessageToGameMachine(lastMessage as MessageEvent<string>, gameMachineRef.send);
      setMessageHistory((prevState) => prevState.concat(lastMessage));
    }
  }, [lastMessage, gameMachineRef]);

  return { sendMessage, messageHistory, connectionStatus };
};

const forwardMessageToGameMachine = (
  message: MessageEvent<string>,
  send: (event: EventFromLogic<typeof GameMachine>) => void,
) => {
  if (newGameMessage.safeParse(message.data)) {
    send({
      type: GameTypes.GameEvent.StartNewGame,
      game: newGameMessageToGame(message),
    });
    return message;
  } else {
    throw new Error("Unrecognized message type.");
  }
};

const newGameMessageToGame = (
  validatedMessage: MessageEvent<string>,
): GameTypes.Game => {
  const parsedMessage = newGameMessage.parse(validatedMessage.data);

  const chessboardPosition: Record<
    GameTypes.File,
    Record<GameTypes.Rank, GameTypes.Piece | null>
  > = {};

  for (const [key, value] of Object.entries(
    parsedMessage.payload.chessboard.position,
  )) {
    const rank = positionKeyToRank(key);
    const file = positionKeyToFile(key);

    let piece = null;
    if (value !== null) {
      piece = positionValueToPiece(value);
    }
    chessboardPosition[file][rank] = piece;
  }

  return {
    id: parsedMessage.payload.id,
    chessboard: { position: chessboardPosition },
    player: GameTypes.Colour.White, // TODO - allow to select either colour...
  };
};

const positionKeyToRank = (key: string): GameTypes.Rank => {
  const rank = key[1];
  if (Object.values(GameTypes.Rank).includes(rank)) {
    return rank as unknown as GameTypes.Rank;
  }
  throw new Error(`${rank} is not a recognised Rank.`);
};

const positionKeyToFile = (key: string): GameTypes.File => {
  const file = parseInt(key[1]) as unknown as GameTypes.File;
  if (Object.values(GameTypes.File).includes(file)) {
    return file;
  }
  throw new Error(`${file} is not a recognised File.`);
};

const positionValueToPiece = ({
  colour,
  pieceType,
}: {
  colour: string;
  pieceType: string;
}): GameTypes.Piece | null => {
  if (!Object.values(GameTypes.Colour).includes(colour)) {
    throw new Error(`${colour} is not a recognised Colour.`);
  }

  if (!Object.values(GameTypes.PieceType).includes(pieceType)) {
    throw new Error(`${pieceType} is not a recognised PieceType.`);
  }

  return {
    colour: colour as unknown as GameTypes.Colour,
    pieceType: pieceType as unknown as GameTypes.PieceType,
  };
};
