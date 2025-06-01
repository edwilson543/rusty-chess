import { expect, Page } from "@playwright/test";

import * as locators from "./locators";
import { Colour, PieceType } from "../src/domain/chess.ts";

export const expectLocalPlayerColourToBe = async (
  page: Page,
  colour: Colour,
) => {
  const localPlayerColour = page.getByTestId("local-player-colour");
  await expect(localPlayerColour).toBeVisible();
  await expect(localPlayerColour).toHaveText(colour);
};

export const expectToPlayColourToEqual = async (page: Page, colour: string) => {
  const toPlayColour = page.getByTestId("to-play-colour");
  await expect(toPlayColour).toContainText(colour);
};

export const expectPieceTypeToOccupySquare = async (
  page: Page,
  square: string,
  colour: Colour,
  PieceType: PieceType,
) => {
  const movedPieceType = locators.getPieceAtSquare(
    page,
    square,
    colour,
    PieceType,
  );
  await expect(movedPieceType).toBeVisible();
};
