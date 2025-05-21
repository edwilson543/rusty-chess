import { test, expect } from "@playwright/test";

import * as assertions from "./assertions.ts";
import * as locators from "./locators.ts";

test("page loads with correct title", async ({ page }) => {
  await page.goto("/");

  await expect(page).toHaveTitle("Rusty Chess");
});

test.describe("gameplay", () => {
  test.beforeEach(async ({ page }) => {
    await page.goto("/");
  });

  test("can play opening move", async ({ page }) => {
    await assertions.expectToPlayColourToEqual(page, "White");

    // Select the pawn at E2, which should enlarge it.
    const piece = locators.getPieceAtSquare(page, "E2", "White", "Pawn");
    await expect(piece).toHaveClass(/fa-3x/);

    await piece.click();
    await expect(piece).toHaveClass(/fa-4x/);

    // Move the pawn to E4.
    const squareE4 = locators.getSquare(page, "E4");
    await squareE4.click();

    await assertions.expectToPlayColourToEqual(page, "Black");

    const movedPiece = locators.getPieceAtSquare(page, "E4", "White", "Pawn");
    await expect(movedPiece).toBeVisible();

    const previousSquare = locators.getSquare(page, "E2");
    await expect(previousSquare).toBeEmpty();
  });

  test("cannot play illegal opening move", async ({ page }) => {
    await assertions.expectToPlayColourToEqual(page, "White");

    // Try to move a pawn diagonally when there's no piece to capture
    const piece = locators.getPieceAtSquare(page, "E2", "White", "Pawn");
    await piece.click();

    // Try to move to diagonal square D3
    const squareD3 = locators.getSquare(page, "D3");
    await squareD3.click();

    // Verify the pawn hasn't moved and it's still White's turn
    await assertions.expectToPlayColourToEqual(page, "White");

    // The pawn should still be at E2
    const originalPiece = locators.getPieceAtSquare(
      page,
      "E2",
      "White",
      "Pawn",
    );
    await expect(originalPiece).toBeVisible();

    // And D3 should be empty
    await expect(squareD3).toBeEmpty();
  });

  test("cannot move opponent piece", async ({ page }) => {
    await assertions.expectToPlayColourToEqual(page, "White");

    // Try to move a Black piece (pawn at E7) when it's White's turn
    const blackPiece = locators.getPieceAtSquare(page, "E7", "Black", "Pawn");
    await blackPiece.click();

    // The piece shouldn't be selected (no fa-4x class)
    await expect(blackPiece).not.toHaveClass(/fa-4x/);

    // It should still be White's turn
    await assertions.expectToPlayColourToEqual(page, "White");

    // The black piece should still be at its original position
    await expect(blackPiece).toBeVisible();
  });
});

test.describe("game controls", () => {
  test.beforeEach(async ({ page }) => {
    await page.goto("/");
  });

  test("can start new game", async ({ page }) => {
    // Make an opening move, to change the board state.
    await locators.getPieceAtSquare(page, "B1", "White", "Knight").click();
    await locators.getSquare(page, "A3").click();

    await expect(
      locators.getPieceAtSquare(page, "A3", "White", "Knight"),
    ).toBeVisible();
    await assertions.expectToPlayColourToEqual(page, "Black");

    // Start a new game.
    const startNewGame = page.getByRole("img", { name: "Start new game" });
    await expect(startNewGame).toBeVisible();
    await startNewGame.click();

    // Verify the board has been reset.
    await expect(
      locators.getPieceAtSquare(page, "B1", "White", "Knight"),
    ).toBeVisible();
    await expect(locators.getSquare(page, "A3")).toBeEmpty();

    await assertions.expectToPlayColourToEqual(page, "White");
  });

  test("can swap player colours", async ({ page }) => {
    // Ensure local player is initially assigned to white.
    let playerColourText = page.getByTestId("local-player-colour");
    await expect(playerColourText).toBeVisible();
    await expect(playerColourText).toHaveText("White");

    // Swap the colours, so that the local player is assigned to black.
    const swapColours = page.getByRole("button", { name: "Swap colours" });
    await expect(swapColours).toBeVisible();

    await swapColours.click();

    playerColourText = page.getByTestId("local-player-colour");
    await expect(playerColourText).toHaveText("Black");
    await assertions.expectToPlayColourToEqual(page, "White");

    // Should not be able to swap back to white during opponent's turn.
    await swapColours.click();

    playerColourText = page.getByTestId("local-player-colour");
    await expect(playerColourText).toHaveText("Black");
    await assertions.expectToPlayColourToEqual(page, "White");
  });

  test("can select game engine to play against", async ({ page }) => {
    const engineSelect = page.getByRole("combobox");
    await expect(engineSelect).toHaveValue("Minimax");

    // Change to Random.
    await engineSelect.selectOption("Random");
    await expect(engineSelect).toHaveValue("Random");

    // Change to MCTS.
    await engineSelect.selectOption("MCTS");
    await expect(engineSelect).toHaveValue("MCTS");
  });
});
