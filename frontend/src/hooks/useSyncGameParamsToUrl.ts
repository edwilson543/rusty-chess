import { useEffect, useMemo } from "react";

import { useSearchParams } from "react-router-dom";

import { useActiveChessGame } from "./useActiveChessGame.ts";

/**
 * Synchronise the URL search parameters with the active game ID.
 * This allows bookmarking and sharing the game.
 * */
export const useSyncGameParamsToUrl = () => {
  const [searchParams, setSearchParams] = useSearchParams();
  const { publicGameId: gameIdInMachine } = useActiveChessGame();

  const gameIdFromUrl = getPublicGameId(searchParams);

  useEffect(() => {
    if (gameIdFromUrl !== gameIdInMachine && gameIdInMachine) {
      setSearchParams(
        (currentParams) => {
          const newParams = new URLSearchParams(currentParams);
          newParams.set("gameId", gameIdInMachine.toString());
          return newParams;
        },
        { replace: false },
      );
    }
  }, [gameIdInMachine, gameIdFromUrl, setSearchParams]);
};

interface UrlGameParams {
  publicGameId: number | null;
}

export const useParseGameParamsFromUrl = (): UrlGameParams => {
  const [searchParams] = useSearchParams();

  return useMemo<UrlGameParams>(() => {
    return { publicGameId: getPublicGameId(searchParams) };
  }, [searchParams]);
};

const getPublicGameId = (searchParams: URLSearchParams): number | null => {
  const id = searchParams.get("gameId");
  return id ? parseInt(id) : null;
};
