import {
  Keypair,
  Connection,
  PublicKey,
  LAMPORTS_PER_SOL,
  Transaction,
  SystemProgram,
  sendAndConfirmTransaction,
} from "@solana/web3.js";

const connection = new Connection("https://api.devnet.solana.com", "confirmed");
const transaction = new Transaction();

const secretKey = Uint8Array.from([
  87, 249, 115, 224, 171, 56, 222, 38, 163, 230, 102, 193, 142, 247, 181, 65,
  171, 80, 172, 23, 180, 220, 120, 248, 226, 2, 174, 66, 158, 229, 64, 252, 64,
  245, 84, 206, 254, 9, 201, 250, 101, 34, 64, 166, 243, 150, 222, 165, 90, 6,
  245, 185, 158, 160, 23, 94, 207, 233, 232, 62, 223, 135, 226, 156,
]);
const keyPair = Keypair.fromSecretKey(secretKey);
const senderPublicKey = keyPair.publicKey.toBase58();
console.log(senderPublicKey);

const senderPubKey = new PublicKey(senderPublicKey);
const toPubKey = new PublicKey("5Ri1y7YHkj1A4LpShwWaQKXBP6twoaVDHB3K4vjUtuis");

const sendSolInstruction = SystemProgram.transfer({
  fromPubkey: senderPubKey,
  toPubkey: toPubKey,
  lamports: 1000,
});
//aad instructions to the transaction
transaction.add(sendSolInstruction);

(async () => {
  try {
    const txSignature = await sendAndConfirmTransaction(
      connection,
      transaction,
      [keyPair]
    );
    console.log("Signature:", txSignature);
  } catch (error) {
    console.error("Transaction failed", error);
  }
})();
