import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { AnchorRustTest } from "../target/types/anchor_rust_test";
import { fromWorkspace, LiteSVMProvider } from "anchor-litesvm";



describe("solana_lottery", () => {
	const client = fromWorkspace("");
  const svm = client;
	const provider = new LiteSVMProvider(client);
  anchor.setProvider(provider);
  const program = anchor.workspace.anchor_rust_test as Program<AnchorRustTest>;;
  
  const initialClock = svm.getClock();
  initialClock.unixTimestamp = BigInt(Math.floor(Date.now()/1000));
  svm.setClock(initialClock);
  
  const user = anchor.web3.Keypair.generate();


  const [counter] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("counter")],
    program.programId
  );


  before(async () => {

     svm.airdrop(user.publicKey,BigInt(2e9));

  });

  it("Allows a user to buy tickets", async () => {


    const tx = await program.methods
      .initialize() // 0.5 SOL
      .accounts({
        counter:counter,
        payer:user.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([user])
      .rpc();

    console.log("User bought ticket:", tx);
  });

  it("increment", async () => {

    const initialClock = svm.getClock();
  initialClock.unixTimestamp = BigInt(Math.floor(Date.now()/1000)+( 60*61));
  svm.setClock(initialClock);
  console.log(initialClock.unixTimestamp)

    const tx = await program.methods
      .increment() // 0.5 SOL
      .accounts({
        counter:counter,
                payer:user.publicKey,

      })
.signers([user])
      .rpc();

    console.log("User bought ticket:", tx);
  });
  
});