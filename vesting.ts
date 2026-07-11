import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { SolanaTokenVestingMinimal } from "./target/types/solana_token_vesting_minimal";
import { assert } from "chai";

describe("solana-token-vesting-minimal", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.SolanaTokenVestingMinimal as Program<SolanaTokenVestingMinimal>;

  it("Initializes a vesting schedule baseline test template", async () => {
    assert.isOk(program.programId);
  });
});
