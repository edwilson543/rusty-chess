import { expect, Page } from "@playwright/test";

import * as locators from "./locators";

export const expectLocalPlayerColourToBe = async (
  page: Page,
  colour: string,
) => {
  const localPlayerColour = page.getByTestId("local-player-colour");
  await expect(localPlayerColour).toBeVisible();
  await expect(localPlayerColour).toHaveText(colour);
};

export const expectToPlayColourToEqual = async (page: Page, colour: string) => {
  const toPlayColour = page.getByTestId("to-play-colour");
  await expect(toPlayColour).toContainText(colour);
};

export const expectPieceToOccupySquare = async (
  page: Page,
  square: string,
  colour: string,
  piece: string,
) => {
  const movedPiece = locators.getPieceAtSquare(page, square, colour, piece);
  await expect(movedPiece).toBeVisible();
};
