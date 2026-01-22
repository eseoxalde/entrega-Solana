import {
  Keypair,
  Connection,
  Transaction,
  SystemProgram,
  sendAndConfirmTransaction,
  PublicKey,
} from "@solana/web3.js";
import assert from "assert";

const PROGRAM_ID = Keypair.generate().publicKey;

describe("Voting Program", () => {
  const connection = new Connection("http://localhost:8899", "confirmed");
  const owner = Keypair.generate();
  const voter = Keypair.generate();

  it("Inicializa config", async () => {
    // crear cuenta config y llamar initialize
  });

  it("Crea propuesta", async () => {
    // enviar instrucción create_proposal
  });

  it("Vota", async () => {
    // enviar instrucción vote
  });

  it("No permite doble voto", async () => {
    // intentar crear la misma cuenta vote dos veces
  });
});
