import { useState, useEffect } from "react";

import useWebSocket, { ReadyState } from "react-use-websocket";

import { newGameMessage } from "./inboundMessageSchema.ts";

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

  useEffect(() => {
    if (lastMessage !== null) {
      validateMessage(lastMessage as MessageEvent<string>);
      setMessageHistory((prevState) => prevState.concat(lastMessage));
    }
  }, [lastMessage]);

  return { sendMessage, messageHistory, connectionStatus };
};

const validateMessage = (message: MessageEvent<string>) => {
  // TODO -> upgrade this to a message handler, rather than a mere validator.

  if (newGameMessage.safeParse(message.data)) {
    return message;
  } else {
    throw new Error("Unrecognized message type.");
  }
};
