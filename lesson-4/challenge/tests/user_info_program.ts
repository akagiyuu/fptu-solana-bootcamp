import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { UserInfoProgram } from "../target/types/user_info_program";
import { assert } from "chai";

function get_explorer_url(signature: string) {
    return `https://explorer.solana.com/tx/${signature}?cluster=devnet`;
}

describe("user_info_program", () => {
    const provider = anchor.AnchorProvider.env();
    anchor.setProvider(provider);

    const program = anchor.workspace.UserInfoProgram as Program<UserInfoProgram>;
    const payer = provider.wallet;
    const user_data_account_keys = anchor.web3.Keypair.generate();

    it("Should be initialized", async () => {
        const name = "Test";
        const age = 1;

        const transaction = await program.methods.initialize(name, age)
            .accounts({
                userData: user_data_account_keys.publicKey,
                payer: payer.publicKey
            })
            .signers([user_data_account_keys])
            .rpc();
        console.log(get_explorer_url(transaction));
        const user_data_account = await program.account.userData.fetch(
            user_data_account_keys.publicKey
        );
        assert(
            user_data_account.name == name,
            `Expected name to be ${name}`
        );
        assert(
            user_data_account.age == age,
            `Expected age to be ${age}`
        );
    });

    it("Name should be initialized", async () => {
        const new_name = "New Test";
        const transaction = await program.methods.update(new_name, null)
            .accounts({
                userData: user_data_account_keys.publicKey
            })
            .rpc();
        console.log(get_explorer_url(transaction));
        const user_data_account = await program.account.userData.fetch(
            user_data_account_keys.publicKey
        );
        assert(
            user_data_account.name == new_name,
            `Expected name to be ${new_name}`
        );
    })

    it("Name should cause error", async () => {
        try {
            // name with length > 100
            const error_name = new Array(200).join('a');
            const transaction = await program.methods.update(error_name, null)
                .accounts({
                    userData: user_data_account_keys.publicKey
                })
                .rpc();

            assert(false, "Must return error")
        } catch(err) {
            assert(
                err instanceof anchor.AnchorError,
                "Error must be of type AnchorError"
            ) 
        }
    })

    it("Age should be initialized", async () => {
        const new_age = 10;
        const transaction = await program.methods.update(null, new_age)
            .accounts({
                userData: user_data_account_keys.publicKey
            })
            .rpc();
        console.log(get_explorer_url(transaction));
        const user_data_account = await program.account.userData.fetch(
            user_data_account_keys.publicKey
        );
        assert(
            user_data_account.age == new_age,
            `Expected name to be ${new_age}`
        );
    })
});
