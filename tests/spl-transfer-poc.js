const anchor = require("@coral-xyz/anchor");

describe("spl-transfer-poc", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  it("Is initialized!", async () => {
    // Add your test here.
    const program = anchor.workspace.SplTransferPoc;
    const user = new anchor.web3.PublicKey("recipient");

    const mintAccount = new anchor.web3.PublicKey("mintkey")
    const toAcc = new anchor.web3.PublicKey("recipient ata")

    const initVault = await program.methods.initTransfer()
      .accounts({
        mint: mintAccount,
        toOwner: user,
        toAta: toAcc
      }).rpc();
    console.log(initVault)

    const withdraw = await program.methods.withdraw(new anchor.BN(90000000000))
      .accounts({
        mint: mintAccount,
        toOwner: user,
        toAta: toAcc
      }).rpc()

    console.log(withdraw)

  });
});
