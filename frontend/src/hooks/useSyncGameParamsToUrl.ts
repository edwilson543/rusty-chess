import { useEffect, useMemo } from "react";

import { useSearchParams } from "react-router-dom";

import { useActiveChessGame } from "./useActiveChessGame.ts";
import { Colour } from "../domain/chess.ts";

/**
 * Synchronise the URL search parameters with the active game ID.
 * This allows bookmarking and sharing the game.
 * */
export const useSyncGameParamsToUrl = () => {
  const [searchParams, setSearchParams] = useSearchParams();
  const { publicGameId: gameIdInMachine, localPlayerColour: colourInMachine } =
    useActiveChessGame();

  const gameIdFromUrl = getPublicGameId(searchParams);
  const colourFromUrl = getColour(searchParams);

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

  useEffect(() => {
    if (colourFromUrl !== colourInMachine) {
      setSearchParams(
        (currentParams) => {
          const newParams = new URLSearchParams(currentParams);
          newParams.set("colour", colourInMachine);
          return newParams;
        },
        { replace: false },
      );
    }
  }, [colourInMachine, colourFromUrl, setSearchParams]);
};

interface UrlGameParams {
  publicGameId: number | null;
  localPlayerColour: Colour | null;
}

export const useParseGameParamsFromUrl = (): UrlGameParams => {
  const [searchParams] = useSearchParams();

  return useMemo<UrlGameParams>(() => {
    return {
      publicGameId: getPublicGameId(searchParams),
      localPlayerColour: getColour(searchParams),
    };
  }, [searchParams]);
};

// Helpers.

const getPublicGameId = (searchParams: URLSearchParams): number | null => {
  const id = searchParams.get("gameId");
  return id ? parseInt(id) : null;
};

const getColour = (searchParams: URLSearchParams): Colour | null => {
  const colour = searchParams.get("colour");
  switch (colour) {
    case "White":
      return Colour.White;
    case "Black":
      return Colour.Black;
    default:
      return null;
  }
};
