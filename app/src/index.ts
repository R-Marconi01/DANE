import * as anchor from "@project-serum/anchor";
import { Program, Provider, web3 } from "@project-serum/anchor";
import {
    Connection,
    PublicKey,
    Keypair,
    SystemProgram,
    BlockheightBasedTransactionConfirmationStrategy
} from "@solana/web3.js";
import { BN } from "bn.js";

const { Wallet } = anchor;

const main = async () => {
    const connection = new Connection("https://api.devnet.solana.com", "confirmed");

    const userKeypair = Keypair.generate();
    const wallet = new Wallet(userKeypair);
    const provider = new Provider(connection, wallet, {
        preflightCommitment: "confirmed",
    });

    anchor.setProvider(provider);

    const idl = await anchor.Program.fetchIdl("YOUR_PROGRAM_ID_HERE", provider);
    const programId = new PublicKey("YOUR_PROGRAM_ID_HERE");
    const program = new Program(idl, programId, provider);

    const cerAccount = Keypair.generate();

    const airdropSignature = await connection.requestAirdrop(userKeypair.publicKey, web3.LAMPORTS_PER_SOL);

    await connection.confirmTransaction({signature: airdropSignature});

    await program.rpc.initializeCer(
        "CER001",
        {
            accounts: {
                cer: cerAccount.publicKey,
                user: userKeypair.publicKey,
                systemProgram: SystemProgram.programId,
            },
            signers: [cerAccount, userKeypair],
        }
    );

    console.log("CER initialized:", cerAccount.publicKey.toString());

    await program.rpc.addUser(
        "User001",
        {
            accounts: {
                cer: cerAccount.publicKey,
                user: userKeypair.publicKey,
                systemProgram: SystemProgram.programId,
            },
            signers: [userKeypair],
        }
    );

    console.log("User added.");

    const externalTimestamp = Math.floor(Date.now() / 1000);

    await program.rpc.updateUserData(
        "User001",
        new BN(50), // Consumption
        new BN(20), // Production
        new BN(externalTimestamp), // External timestamp
        {
            accounts: {
                cer: cerAccount.publicKey,
            },
            signers: [userKeypair],
        }
    );

    console.log("User data updated.");

    await program.rpc.removeUser(
        "User001",
        {
            accounts: {
                cer: cerAccount.publicKey,
                user: userKeypair.publicKey,
                systemProgram: SystemProgram.programId,
            },
            signers: [userKeypair],
        }
    );

    console.log("User removed.");

    await program.rpc.uploadEntireCerData(
        [
            ["User001", new BN(100), new BN(50), new BN(externalTimestamp)],
            ["User002", new BN(200), new BN(100), new BN(externalTimestamp)],
        ],
        {
            accounts: {
                cer: cerAccount.publicKey,
            },
            signers: [userKeypair],
        }
    );

    console.log("CER data uploaded.");
};

main().then(() => console.log("Success")).catch((err) => console.error(err));
