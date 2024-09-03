Chess web app that I made to try out Rust.
The app consists of a Rust backend and TypeScript frontend that communicate via a REST API.

## Demo:
It took surprisingly many attempts to b2b Scholar's Mate the random chess engine...

https://github.com/user-attachments/assets/1db85aea-3a0e-4dd7-a5e8-4ac497ce53b7


## System requirements:
- Cargo 1.78.0
- Node 18.16.0
- Postgres 14.9

## Install the app:
```bash
git clone git@github.com:edwilson543/rusty-chess.git
cd rusty-chess
make install
```


## Run the app:
```bash
make run --jobs=2
```
Then visit http://localhost:5173/


## Tech stack
### Backend:
- The domain and service layers are written in pure Rust
- The data and repository layers use [Diesel][diesel] to interact with a Postgres db
- The interfaces layer consists of a REST API implemented using the [Rocket][rocket] web framework

### Frontend:
- Quick implementation using TypeScript, React and XState


## Domain model:
- The domain of "chess" is modelled at `backend/src/domain/`
- My modelling approach was to [rawdog][rawdog] it. That is, I intentionally did not research typical approaches
- This domain model is split into the following components
  - **`chess_set/`**
    - This represents a physical chess set, as you might expect to buy from a shop
    - All models are value objects; for example `Chessboard`, `Piece` and `Colour`
    - Each model is implemented either as a `struct` or an `enum`
    - The `Chessboard` does not enforce any rules of chess. The most it does is provide methods for adding and removing
    pieces from the board
  - **`rulebook/`**
    - This is where the rules of chess are defined
    - Legal moves are modelled by implementations of the `Move` trait. For example, en passant
    - All other rules are modelled by functions. For example, the initial starting position and checkmate
  - **`game/`**
    - The `game` subdomain pulls together the `chess_set` and `rulebook` into a playable model of chess
    - The key model is the `Game` struct, an entity representing a single game of chess
    - `Game` includes methods for playing moves, and records the history of a single chess game
  - **`engine/`**
    - This portion of the domain is what allows you to "play against the computer"
    - The `ChessEngine` trait models the process of move generation
    - Each implementation of this trait represents a different strategy for generating moves
    - The simplest implementation is `Random`, which simply picks a random legal move to play



[diesel]: https://diesel.rs/
[rawdog]: https://www.nytimes.com/2024/07/17/style/rawdog-flights-term.html
[rocket]: https://rocket.rs/guide/v0.5/
[domain-model]: https://github.com/edwilson543/rusty-chess/tree/main/backend/src/domain