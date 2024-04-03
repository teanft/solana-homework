import * as web3 from "@solana/web3.js";
import * as borsh from "@project-serum/borsh";
import * as anchor from "@coral-xyz/anchor";

const programId = new web3.publicKey("AxhGGBeAQc1SumsczWjgUxbbhhZR7FsTAtfoc5Ay6oaw")

const invodeInstructionLayout = borsh.struct([
    borsh.u8("newAccountBumpSeed"),
    borsh.u64("space"),
]);

export async function invokeProgram(
    connection: web3.Connection,
    payer: web3.Keypair,
) {
    const [vault, vaultBumpSeed] = web3.PublicKey.findProgramAddressSync(
        [Buffer.from("vault")], payer.publicKey.toneBuffer()],
        programId,
    )

    let buffer = Buffer.alloc(1000);
    const newAccountBumpSeed = vauletBumpSeed;
    const space = new anchor.BN(10);

    invokeInstructionLayout.encode(
        {
            newAccountBumpSeed: newAccountBumpSeed,
            space: space,
        },
        buffer,
    )
    buffer = buffer.slice(0, invokeInstructionLayout.getSpan(buffer));
    const transaction = new web3.Transaction();
    const instruction = new web3.TransactionInstruction({
        programId: programId,
        data: buffer,
        keys: [
            {
                pubkey: payer.publicKey,
                isSigner: true,
                isWritable: true,
            },
            {
                pubkey: vault,
                isSigner: false,
                isWritable: true,
            },
            {
                pubkey: web3.SystemProgram.programId,
                isSigner: false,
                isWritable: false,
            }
        ]
    });

    transaction.add(instruction)
    const tx = await web3.sendAndConfirmTransaction(connection, transaction, [payer])
    console.log(`https://explorer.solana.com/tx/${tx}?cluster=devnet`)
}