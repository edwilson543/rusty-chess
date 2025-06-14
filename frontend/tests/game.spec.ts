import { expect, test } from "@playwright/test";

import * as assertions from "./assertions.ts";
import * as locators from "./locators.ts";
import { Colour, PieceType } from "../src/domain/chess.ts";

test("page loads with correct title", async ({ page }) => {
  await page.goto("/");

  await expect(page).toHaveTitle("Rusty Chess");
});

test.describe("gameplay", () => {
  test.beforeEach(async ({ page }) => {
    await page.goto("/");
  });

  test("can play opening move", async ({ page }) => {
    await assertions.expectToPlayColourToEqual(page, Colour.White);

    // Select the pawn at E2, which should enlarge it.
    const piece = locators.getPieceAtSquare(
      page,
      "E2",
      Colour.White,
      PieceType.Pawn,
    );
    await expect(piece).toHaveClass(/fa-3x/);

    await piece.click();
    await expect(piece).toHaveClass(/fa-4x/);

    // Move the pawn to E4.
    const squareE4 = locators.getSquare(page, "E4");
    await squareE4.click();

    await assertions.expectToPlayColourToEqual(page, Colour.Black);

    await assertions.expectPieceTypeToOccupySquare(
      page,
      "E4",
      Colour.White,
      PieceType.Pawn,
    );

    const previousSquare = locators.getSquare(page, "E2");
    await expect(previousSquare).toBeEmpty();
  });

  test("cannot play illegal opening move", async ({ page }) => {
    await assertions.expectToPlayColourToEqual(page, Colour.White);

    // Try to move a pawn diagonally when there's no PieceType to capture
    const piece = locators.getPieceAtSquare(
      page,
      "E2",
      Colour.White,
      PieceType.Pawn,
    );
    await piece.click();

    // Try to move to diagonal square D3
    const squareD3 = locators.getSquare(page, "D3");
    await squareD3.click();

    // Verify the pawn hasn't moved and it's still White's turn
    await assertions.expectToPlayColourToEqual(page, Colour.White);

    // The pawn should still be at E2
    await assertions.expectPieceTypeToOccupySquare(
      page,
      "E2",
      Colour.White,
      PieceType.Pawn,
    );

    // And D3 should be empty
    await expect(squareD3).toBeEmpty();
  });

  test("cannot move opponent PieceType", async ({ page }) => {
    await assertions.expectToPlayColourToEqual(page, Colour.White);

    // Try to move a Black PieceType (pawn at E7) when it's White's turn
    const blackPieceType = locators.getPieceAtSquare(
      page,
      "E7",
      Colour.Black,
      PieceType.Pawn,
    );
    await blackPieceType.click();

    // The PieceType shouldn't be selected (no fa-4x class)
    await expect(blackPieceType).not.toHaveClass(/fa-4x/);

    // It should still be White's turn
    await assertions.expectToPlayColourToEqual(page, Colour.White);

    // The black PieceType should still be at its original position
    await expect(blackPieceType).toBeVisible();
  });
});

test.describe("game controls", () => {
  test.beforeEach(async ({ page }) => {
    await page.goto("/");
  });

  test("can start new game", async ({ page }) => {
    // Make an opening move, to change the board state.
    await locators
      .getPieceAtSquare(page, "B1", Colour.White, PieceType.Knight)
      .click();
    await locators.getSquare(page, "A3").click();

    await assertions.expectPieceTypeToOccupySquare(
      page,
      "A3",
      Colour.White,
      PieceType.Knight,
    );
    await assertions.expectToPlayColourToEqual(page, Colour.Black);

    // Start a new game.
    const startNewGame = page.getByRole("img", { name: "Start new game" });
    await expect(startNewGame).toBeVisible();
    await startNewGame.click();

    // Verify the board has been reset.
    await assertions.expectPieceTypeToOccupySquare(
      page,
      "B1",
      Colour.White,
      PieceType.Knight,
    );
    await expect(locators.getSquare(page, "A3")).toBeEmpty();

    await assertions.expectToPlayColourToEqual(page, Colour.White);
  });

  test("can swap player colours", async ({ page }) => {
    // Ensure local player is initially assigned to white.
    await assertions.expectLocalPlayerColourToBe(page, Colour.White);

    // Swap the colours, so that the local player is assigned to black.
    const swapColours = page.getByRole("button", { name: "Swap colours" });
    await expect(swapColours).toBeVisible();

    await swapColours.click();

    await assertions.expectLocalPlayerColourToBe(page, Colour.Black);
    await assertions.expectToPlayColourToEqual(page, Colour.White);

    // Should not be able to swap back to white during opponent's turn.
    await swapColours.click();

    await assertions.expectLocalPlayerColourToBe(page, Colour.Black);
    await assertions.expectToPlayColourToEqual(page, Colour.White);
  });

  test("can select game engine to play against", async ({ page }) => {
    const engineSelect = page.getByRole("combobox");
    await expect(engineSelect).toHaveValue("Random");

    // Change to Minimax.
    await engineSelect.selectOption("Minimax");
    await expect(engineSelect).toHaveValue("Minimax");

    // Change to MCTS.
    await engineSelect.selectOption("MCTS");
    await expect(engineSelect).toHaveValue("MCTS");
  });
});

test.describe("existing games", () => {
  test("can share game by copying link to clipboard", async ({
    page,
    context,
  }) => {
    await page.goto("/");

    await expect(page).toHaveURL(/\?colour=White/);
    await expect(page).toHaveURL(/&gameId=/);

    // Move a PieceType.
    const squareE2 = locators.getSquare(page, "E2");
    await squareE2.click();

    const squareE4 = locators.getSquare(page, "E4");
    await squareE4.click();

    await assertions.expectPieceTypeToOccupySquare(
      page,
      "E4",
      Colour.White,
      PieceType.Pawn,
    );

    // Grant clipboard permissions to browser context.
    await context.grantPermissions(["clipboard-read", "clipboard-write"]);

    // Copy the game's link to the clipboard.
    const shareGame = page.getByRole("button", { name: "Share game" });
    await shareGame.click();

    const clipboardText = await page.evaluate(() =>
      navigator.clipboard.readText(),
    );
    const expectedUrl = page.url().replace(Colour.White, Colour.Black);
    expect(clipboardText).toEqual(expectedUrl);

    // Open a new page with the url - the new page should be playing as black.
    const newPage = await context.newPage();
    await newPage.goto(clipboardText);

    await assertions.expectLocalPlayerColourToBe(newPage, Colour.Black);
    await assertions.expectToPlayColourToEqual(newPage, Colour.Black);
    await assertions.expectPieceTypeToOccupySquare(
      newPage,
      "E4",
      Colour.White,
      PieceType.Pawn,
    );
  });

  test("can revisit game using game id pushed to url search params", async ({
    page,
  }) => {
    await page.goto("/");

    await expect(page).toHaveURL(/\?colour=White/);
    await expect(page).toHaveURL(/&gameId=/);
    const gameId = page.url().split("gameId=")[1];

    // Move a PieceType in the first game.
    const squareE2 = locators.getSquare(page, "E2");
    await squareE2.click();

    const squareE4 = locators.getSquare(page, "E4");
    await squareE4.click();

    await assertions.expectPieceTypeToOccupySquare(
      page,
      "E4",
      Colour.White,
      PieceType.Pawn,
    );

    // Start a new game.
    const startNewGame = page.getByRole("img", { name: "Start new game" });
    await startNewGame.click();

    await expect(locators.getSquare(page, "E4")).toBeEmpty();

    // Reload the first game.
    await page.goto(`/?gameId=${gameId}`);
    await assertions.expectPieceTypeToOccupySquare(
      page,
      "E4",
      Colour.White,
      PieceType.Pawn,
    );
  });
});
