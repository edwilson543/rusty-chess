import { Page, Locator } from "@playwright/test";

export const getSquare = (page: Page, square: string): Locator => {
  const squareTestId = `square-${square}`;
  return page.getByTestId(squareTestId);
};

export const getPieceAtSquare = (
  page: Page,
  square: string,
  colour: string,
  piece: string,
): Locator => {
  const squareTestId = `square-${square}`;
  return page
    .getByTestId(squareTestId)
    .getByRole("img", { name: `${colour} ${piece}` });
};

export const getLocalPlayerColour = (page: Page): Locator => {
  return page.locator("span", { hasText: "You are:" }).locator("i").first();
};
