import useWebSocket from "react-use-websocket";
import { useState, useEffect } from "react";

export const useGameWebSocket = () => {
  const socketUrl = "ws://127.0.0.1:8000/api/play/";
  const [messageHistory, setMessageHistory] = useState<MessageEvent<string>[]>(
    [],
  );

  const { sendMessage, lastMessage } = useWebSocket(socketUrl);

  useEffect(() => {
    if (lastMessage !== null) {
      setMessageHistory((prevState) => prevState.concat(lastMessage));
    }
  }, [lastMessage]);

  return { sendMessage, messageHistory };
};
