import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { TodoProgram } from "../target/types/todo_program";
import { assert } from "chai";

function get_explorer_url(signature: string) {
    return `https://explorer.solana.com/tx/${signature}?cluster=devnet`;
}

describe("todo_program", () => {
    const provider = anchor.AnchorProvider.env();
    anchor.setProvider(provider);

    const program = anchor.workspace.TodoProgram as Program<TodoProgram>;

    const creator = provider.wallet;

    it("Create profile should succeed", async () => {
        const profile = anchor.web3.Keypair.generate();
        const signature = await program.methods.createProfile("Test")
            .accounts({
                creator: creator.publicKey,
                profile: profile.publicKey
            })
            .signers([profile])
            .rpc();
        console.log(get_explorer_url(signature));
    });

    it("Create profile should fail", async () => {
        const profile = anchor.web3.Keypair.generate();
        const long_name = new Array(400).join('a');
        try {
            await program.methods.createProfile(long_name)
                .accounts({
                    creator: creator.publicKey,
                    profile: profile.publicKey
                })
                .signers([profile])
                .rpc();
        } catch (_err) {
            assert.isTrue(_err instanceof anchor.AnchorError)
            const err: anchor.AnchorError = _err;
            assert.strictEqual(err.error.errorMessage, "Name is too long");
            assert.strictEqual(err.error.errorCode.number, 6000);
            assert.strictEqual(err.error.errorCode.code, "NameTooLong");
            assert.strictEqual(
                err.program.toString(),
                program.programId.toString()
            );
        }
    })
});
