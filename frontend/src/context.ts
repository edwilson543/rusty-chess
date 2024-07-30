import { createActorContext } from "@xstate/react";

import { GameMachine } from "./machines/game";

export const GameMachineContext = createActorContext(GameMachine);
