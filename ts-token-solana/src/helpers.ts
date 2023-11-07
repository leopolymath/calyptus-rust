import { readFileSync } from "fs";
import { homedir } from "os";
import { Keypair } from "@solana/web3.js";

const USER_KEYPAIR_PATH = homedir() + "/.config/solana/devnet.json";
export const userKeypair = Keypair.fromSecretKey(
  Buffer.from(JSON.parse(readFileSync(USER_KEYPAIR_PATH, "utf-8")))
);
