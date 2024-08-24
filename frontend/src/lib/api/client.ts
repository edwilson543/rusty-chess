import { initClient } from "@ts-rest/core";

import { contract } from "./contract.ts";

export const APIClient = initClient(contract, { baseUrl: "/api" });
