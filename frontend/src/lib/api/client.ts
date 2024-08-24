import { initClient } from "@ts-rest/core";

import { contract } from "./contract.ts";

export const APIClient = initClient(contract, {
  baseUrl: "http://127.0.0.1:8000/api",
  cors: false,
});
