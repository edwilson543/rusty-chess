import { createActorContext } from "@xstate/react";

import { gameMachine } from "./machine.ts";

export const GameMachineContext = createActorContext(gameMachine);
