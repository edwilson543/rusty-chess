import { expect, Page } from "@playwright/test";

export const expectToPlayColourToEqual = async (page: Page, colour: string) => {
  const toPlayColour = page.getByTestId("to-play-colour");
  await expect(toPlayColour).toContainText(colour);
};
