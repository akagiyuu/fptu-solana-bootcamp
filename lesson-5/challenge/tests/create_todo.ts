import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { TodoProgram } from "../target/types/todo_program";
import { assert, expect } from "chai";

function get_explorer_url(signature: string) {
    return `https://explorer.solana.com/tx/${signature}?cluster=devnet`;
}

describe("todo_program", () => {
    const provider = anchor.AnchorProvider.env();
    anchor.setProvider(provider);

    const program = anchor.workspace.TodoProgram as Program<TodoProgram>;

    const creator = provider.wallet;
    const profile_name = "Test";
    const profile = anchor.web3.Keypair.generate();

    // init the profile
    before(async () => {
        const signature = await program.methods.createProfile(profile_name)
            .accounts({
                creator: creator.publicKey,
                profile: profile.publicKey
            })
            .signers([profile])
            .rpc();
        console.log(get_explorer_url(signature));
    })

    it("Create todo should succeed", async () => {
        const title = "Test";
        const content = "Test";
        const todo = anchor.web3.Keypair.generate();
        const current_todo_count = (await program.account.profile.fetch(profile.publicKey)).todoCount;

        const signature = await program.methods.createTodo(title, content)
            .accounts({
                profile: profile.publicKey,
                creator: creator.publicKey,
                todo: todo.publicKey
            })
            .signers([todo])
            .rpc();
        console.log(get_explorer_url(signature));

        const profile_account = await program.account.profile.fetch(profile.publicKey);
        const todo_account = await program.account.todo.fetch(todo.publicKey);

        expect(todo_account.profile.toBase58()).to.equal(profile.publicKey.toBase58());
        expect(todo_account.content).to.equal(content);
        expect(todo_account.isCompleted).to.equal(false)

        expect(profile_account.todoCount).to.equal(current_todo_count + 1)
    });

    it("Create profile should fail due to long content", async () => {
        const title = "Test";
        const content = new Array(400).join('a');
        const todo = anchor.web3.Keypair.generate();

        try {
            await program.methods.createTodo(title, content)
                .accounts({
                    profile: profile.publicKey,
                    creator: creator.publicKey,
                    todo: todo.publicKey
                })
                .signers([todo])
                .rpc();
        } catch (_err) {
            assert.isTrue(_err instanceof anchor.AnchorError);
            const err: anchor.AnchorError = _err;
            expect(err.error.errorMessage).to.equal("Content is too long");
            expect(err.error.errorCode.number).to.equal(6002);
            expect(err.error.errorCode.code).to.equal("ContentTooLong");
            expect(err.program.toString()).to.equal(program.programId.toString());
        }
    })

    // it("Create profile should fail due to invalid authority", async () => {
    //     const content = "Test";
    //     const todo = anchor.web3.Keypair.generate();
    //     const invalid_creator = anchor.web3.Keypair.generate();
    //     await provider.connection.confirmTransaction(
    //         await provider.connection.requestAirdrop(
    //             invalid_creator.publicKey,
    //             anchor.web3.LAMPORTS_PER_SOL
    //         )
    //     );
    //
    //     try {
    //         await program.methods.createTodo(content)
    //             .accounts({
    //                 profile: profile.publicKey,
    //                 creator: invalid_creator.publicKey,
    //                 todo: todo.publicKey
    //             })
    //             .rpc();
    //     } catch (_err) {
    //         console.log(_err);
    //         assert.isTrue(_err instanceof anchor.AnchorError);
    //         const err: anchor.AnchorError = _err;
    //         assert.strictEqual(err.error.errorMessage, "Invalid authority");
    //         assert.strictEqual(err.error.errorCode.number, 6002);
    //         assert.strictEqual(err.error.errorCode.code, "InvalidAuthority");
    //         assert.strictEqual(
    //             err.program.toString(),
    //             program.programId.toString()
    //         );
    //     }
    // })
});
