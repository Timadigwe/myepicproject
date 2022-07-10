const anchor = require('@project-serum/anchor');

const { SystemProgram } = anchor.web3;

const main = async() => {
  console.log('starting test...');

  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.Myepicproject;

  const baseAccount = anchor.web3.Keypair.generate();
  const tx = await program.rpc.startStuffOff();
  console.log("Your transaction signature", tx);
}

const runMain = async () => {
  try {
    await main();
    process.exit(0);
  } catch (error) {
    console.error(error);
    process.exit(1);
  }
}

runMain();