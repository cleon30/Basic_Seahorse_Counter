import * as anchor from "@project-serum/anchor";
import { BN, Program, web3 } from '@project-serum/anchor'
import { Counter } from "../target/types/counter";
const assert = require('assert')

describe("counter", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env()
  anchor.setProvider(provider)

  const program = anchor.workspace.Counter as Program<Counter>;
  const owner = provider.wallet.publicKey;

  const counter = web3.PublicKey.findProgramAddressSync(
    [Buffer.from('Counter'), owner.toBuffer()],
    program.programId
  )[0]
  console.log("pass1");
  it('Inits the counter', async () => {
    try{
      await program.methods
        .initCounter()
        .accounts({
                    owner, 
                    counter
                    })
        .rpc()
      }catch(e){
        console.log("counter already initialized or error on it lol");
      }
  })
  it('Add 1 to the counter', async () => {
    const prevcounterAccount = await (await program.account.counter.fetch(counter)).count.toNumber();
    const add2 = await program.methods
      .add()
      .accounts({ owner, counter })
      .instruction()

    const tx = new web3.Transaction()
    tx.add(add2,add2,add2);
    await provider.sendAndConfirm(tx)
    const counterAccount = await program.account.counter.fetch(counter);
    console.log("so, your current count is: ", counterAccount.count.toNumber());
    assert.ok(counterAccount.count.toNumber() === prevcounterAccount+3); // I have added 3 times the call , because of that the 3
});
})
