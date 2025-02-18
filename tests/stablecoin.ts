import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Stablecoin } from "../target/types/stablecoin";
import { PythSolanaReceiver } from "@pythnetwork/pyth-solana-receiver";

describe("stablecoin", () => {
  const provider = anchor.AnchorProvider.env();
  const connection = provider.connection;
  const wallet = provider.wallet as anchor.Wallet;
  anchor.setProvider(provider);

  const program = anchor.workspace.Stablecoin as Program<Stablecoin>;

  const pythSolanaReceiver = new PythSolanaReceiver({ connection, wallet });
  const SOL_PRICE_FEED_ID =
    "0xef0d8b6fda2ceba41da15d4095d1da392a0d2f8ed0c6c7bc0f4cfac8c280b56d";
  const solUsdPriceFeedAccount = pythSolanaReceiver
    .getPriceFeedAccountAddress(0, SOL_PRICE_FEED_ID);

  const [collateralAccount] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("collateral_account"), wallet.publicKey.toBuffer()],
    program.programId
  );

  it("Should initialized!", async () => {
    const tx = await program.methods
      .initConfig()
      .accounts({})
      .rpc({ skipPreflight: true, commitment: "confirmed" });
    console.log("Init_Config tx signature", tx);
  });

  it("Should Deposit Collateral and Mint USDS", async () => {
    const amountCollateral = 1_000_000_000;
    const amountToMint = 1_000_000_000;
    const tx = await program.methods
      .depositCollateralAndMintTokens(
        new anchor.BN(amountCollateral),
        new anchor.BN(amountToMint)
      )
      .accounts({ priceUpdate: solUsdPriceFeedAccount })
      .rpc({ skipPreflight: true, commitment: "confirmed" });
    console.log("Deposit collateral and mint tokens tx signature", tx);
  });

  it("Should Redeem Collateral and Burn Tokens", async () => {
    const amountCollateral = 500_000_000;
    const amountToBurn = 500_000_000;
    const tx = await program.methods
      .redeemCollateralAndBurnTokens(
        new anchor.BN(amountCollateral),
        new anchor.BN(amountToBurn)
      )
      .accounts({ priceUpdate: solUsdPriceFeedAccount })
      .rpc({ skipPreflight: true, commitment: "confirmed" });
    console.log("Redeem collateral & burn tokens tx signature", tx);
  });

  // Increase minimum health threshold to test liquidate
  it("Should Update Config 1", async () => {
    const tx = await program.methods
      .updateConfig(new anchor.BN(100))
      .accounts({})
      .rpc({ skipPreflight: true, commitment: "confirmed" });
    console.log("Increase min. heal threshold tx signature", tx);
  });

  it("Liquidate", async () => {
    const amountToLiquidate = 500_000_000;
    const tx = await program.methods
      .liquidate(new anchor.BN(amountToLiquidate))
      .accounts({ collateralAccount, priceUpdate: solUsdPriceFeedAccount })
      .rpc({ skipPreflight: true, commitment: "confirmed" });
    console.log("Liquidate tx signature", tx);
  });

  it("Should Update Config 2", async () => {
    const tx = await program.methods
      .updateConfig(new anchor.BN(1))
      .accounts({})
      .rpc({ skipPreflight: true, commitment: "confirmed" });
    console.log("Back to normal min. health threshold tx signature", tx);
  });
});