// import * as anchor from "@project-serum/anchor";
// import { Program, AnchorProvider, web3 } from "@project-serum/anchor";
// import { Connection, PublicKey, Keypair } from "@solana/web3.js";
// import { readFileSync } from "fs";

// const { SystemProgram } = web3;

// const main = async () => {
//   // Configure the client to use the local cluster.
//   const connection = new Connection("http://127.0.0.1:8899", "confirmed");
//   const wallet = anchor.Wallet.local();
//   const provider = new AnchorProvider(connection, wallet, {
//     preflightCommitment: "recent",
//   });
//   anchor.setProvider(provider);

//   // Load the IDL for your program.
//   const idl = JSON.parse(
//     readFileSync("./target/idl/dane_program.json", "utf8")
//   );

//   // Address of the deployed program.
//   const programId = new PublicKey("DaneProgram111111111111111111111111111111111");

//   // Generate the program client from IDL.
//   const program = new Program(idl, programId, provider);

//   // Define the keypair for the community account
//   const communityKeypair = Keypair.generate();
//   const communityPubkey = communityKeypair.publicKey;

//   // Define the admin pubkey (for this example, we'll use the wallet public key)
//   const adminPubkey = wallet.publicKey;

//   // Define the name for the community
//   const name = Buffer.from("ExampleCommunityName", "utf8");

//   // Send the transaction to create the community
//   try {
//     const tx = await program.methods
//       .createCommunity(Array.from(name), adminPubkey)
//       .accounts({
//         community: communityPubkey,
//         user: wallet.publicKey,
//         communityProgram: programId,
//         systemProgram: SystemProgram.programId,
//       })
//       .signers([communityKeypair])
//       .rpc();

//     console.log("Transaction signature:", tx);
//     console.log("Community account public key:", communityPubkey.toBase58());
//   } catch (err) {
//     console.error("Transaction error:", err);
//   }
// };

// main().then(() => console.log("Success")).catch((err) => console.error(err));


// import * as anchor from "@project-serum/anchor";
// import { Program, AnchorProvider, web3 } from "@project-serum/anchor";
// import { Connection, PublicKey, Keypair } from "@solana/web3.js";
// import { readFileSync } from "fs";

// const { SystemProgram } = web3;

// const main = async () => {
//   const connection = new Connection("http://127.0.0.1:8899", "confirmed");
//   const wallet = anchor.Wallet.local();
//   const provider = new AnchorProvider(connection, wallet, {
//     preflightCommitment: "recent",
//   });
//   anchor.setProvider(provider);

//   // Load the IDL files for your programs.
//   const daneIdl = JSON.parse(readFileSync("./target/idl/dane_program.json", "utf8"));
//   const communityIdl = JSON.parse(readFileSync("./target/idl/community_program.json", "utf8"));

//   // Address of the deployed programs.
//   const daneProgramId = new PublicKey("DaneProgram111111111111111111111111111111111");
//   const communityProgramId = new PublicKey("CommunityProgram1111111111111111111111111111");

//   // Generate the program clients from IDLs.
//   const daneProgram = new Program(daneIdl, daneProgramId, provider);
//   const communityProgram = new Program(communityIdl, communityProgramId, provider);

//   // Create a community
//   const communityKeypair = Keypair.generate();
//   const communityPubkey = communityKeypair.publicKey;
//   const adminPubkey = wallet.publicKey;
//   const name = Buffer.from("ExampleCommunityName", "utf8");

//   const tx = await daneProgram.rpc.createCommunity(name, adminPubkey, {
//     accounts: {
//       community: communityPubkey,
//       user: wallet.publicKey,
//       communityProgram: communityProgramId,
//       systemProgram: SystemProgram.programId,
//     },
//     signers: [communityKeypair],
//     instructions: [
//       await daneProgram.account.community.createInstruction(communityKeypair),
//     ],
//   });

//   console.log("Community created with transaction:", tx);

//   // Add a user to the community
//   const userKeypair = Keypair.generate();
//   const userPubkey = userKeypair.publicKey;

//   const txAddUser = await communityProgram.rpc.addUser(userPubkey, {
//     accounts: {
//       community: communityPubkey,
//       admin: wallet.publicKey,
//     },
//   });

//   console.log("User added with transaction:", txAddUser);

//   // User publishes consumption data
//   const dataKeypair = Keypair.generate();
//   const dataPubkey = dataKeypair.publicKey;
//   const timestamp = Math.floor(Date.now() / 1000);
//   const consumption = 500;
//   const production = 600;

//   const txPublishData = await communityProgram.rpc.publishData({
//     timestamp,
//     consumption,
//     production,
//   }, {
//     accounts: {
//       community: communityPubkey,
//       user: userPubkey,
//       record: dataPubkey,
//       systemProgram: SystemProgram.programId,
//     },
//     signers: [dataKeypair],
//     instructions: [
//       await communityProgram.account.data.createInstruction(dataKeypair),
//     ],
//   });

//   console.log("Data published with transaction:", txPublishData);
// };

// main().then(() => console.log("Success")).catch((err) => console.error(err));


// import * as anchor from "@project-serum/anchor";
// import { Connection, Keypair, PublicKey, SystemProgram } from "@solana/web3.js";
// import fs from "fs";

// // Function to load a keypair from a file
// const loadKeypairFromFile = (path: string): Keypair => {
//   return Keypair.fromSecretKey(new Uint8Array(JSON.parse(fs.readFileSync(path, "utf-8"))));
// };

// const main = async () => {
//   // Configure the client to use the local cluster.
//   const connection = new anchor.web3.Connection(anchor.web3.clusterApiUrl("devnet"), "confirmed");
//   const admin = loadKeypairFromFile("/path/to/admin/keypair.json");

//   // Use an Anchor wallet provider
//   const provider = new anchor.Provider(connection, admin, {
//     preflightCommitment: "confirmed",
//   });

//   // Load IDLs (Interface Definition Language) for both programs
//   const daneIdl = JSON.parse(fs.readFileSync("/path/to/dane_idl.json", "utf-8"));
//   const communityIdl = JSON.parse(fs.readFileSync("/path/to/community_idl.json", "utf-8"));

//   // Address of the deployed DANE and Community programs
//   const daneProgramId = new PublicKey("DANE_PROGRAM_ID");
//   const communityProgramId = new PublicKey("COMMUNITY_PROGRAM_ID");

//   // Initialize the DANE program
//   const daneProgram = new anchor.Program(daneIdl, daneProgramId, provider);

//   const daneAccount = Keypair.generate();
//   await daneProgram.rpc.initialize(admin.publicKey, {
//     accounts: {
//       daneAccount: daneAccount.publicKey,
//       admin: admin.publicKey,
//       systemProgram: SystemProgram.programId,
//     },
//     signers: [daneAccount, admin],
//   });

//   console.log("DANE initialized with account:", daneAccount.publicKey.toBase58());

//   // Initialize the Community program
//   const communityProgram = new anchor.Program(communityIdl, communityProgramId, provider);

//   const communityAccount = Keypair.generate();
//   await communityProgram.rpc.initialize(admin.publicKey, {
//     accounts: {
//       communityAccount: communityAccount.publicKey,
//       admin: admin.publicKey,
//       systemProgram: SystemProgram.programId,
//     },
//     signers: [communityAccount, admin],
//   });

//   console.log("Community initialized with account:", communityAccount.publicKey.toBase58());

//   // Add the community to DANE
//   await daneProgram.rpc.createCommunity(communityAccount.publicKey, {
//     accounts: {
//       daneAccount: daneAccount.publicKey,
//       communityAccount: communityAccount.publicKey,
//       admin: admin.publicKey,
//       systemProgram: SystemProgram.programId,
//     },
//     signers: [admin],
//   });

//   console.log("Community added to DANE");

//   // Add a user to the community
//   const user = Keypair.generate();
//   await communityProgram.rpc.addUser(user.publicKey, {
//     accounts: {
//       communityAccount: communityAccount.publicKey,
//       admin: admin.publicKey,
//       systemProgram: SystemProgram.programId,
//     },
//     signers: [admin],
//   });

//   console.log("User added to the community with account:", user.publicKey.toBase58());

//   // User publishes consumption data
//   const userAccount = Keypair.generate();
//   const consumption = 100;
//   const production = 50;
//   const timestamp = Math.floor(Date.now() / 1000);

//   await communityProgram.rpc.publishData(consumption, production, timestamp, {
//     accounts: {
//       userAccount: userAccount.publicKey,
//       user: user.publicKey,
//     },
//     signers: [user],
//   });

//   console.log("User published consumption data");
// };

// main().catch((err) => {
//   console.error(err);
// });


// import * as anchor from "@coral-xyz/anchor";
// import { Program, BN } from "@coral-xyz/anchor";
// import { Dane } from "../target/types/dane";
 
// const provider = anchor.AnchorProvider.env();
// anchor.setProvider(provider);
// const program = anchor.workspace.Dane as Program<Dane>;

import * as anchor from '@coral-xyz/anchor';
import { Connection, PublicKey, SystemProgram, Keypair } from '@solana/web3.js';
// import { IDL, Dane } from '../target/idl/dane.json';

const provider = anchor.AnchorProvider.env();
anchor.setProvider(provider);

// const program = anchor.workspace.dane;
const idl = JSON.parse(
    require("fs").readFileSync("./target/idl/dane.json", "utf8")
);

//Address of the deployed program
const programId = new anchor.web3.PublicKey("13YVAfE694jp4BF1poBcjcaafVkWtKhdhUZutNvHQw1z");

//Generate the program client from IDL
const program = new anchor.Program(idl);

const admin = provider.wallet.publicKey;

async function initialize() {
    const daneAccount = Keypair.generate();
    await program.methods.initialize(admin, {
        accounts: {
            daneAccount: daneAccount.publicKey,
            user: provider.wallet.publicKey,
            systemProgram: SystemProgram.programId,
        },
        signers: [daneAccount],
    });
}

async function createCommunity(communityAdmin: PublicKey) {
    const community = Keypair.generate();
    await program.methods.createCommunity(communityAdmin, {
        accounts: {
            community: community.publicKey,
            user: provider.wallet.publicKey,
            systemProgram: SystemProgram.programId,
        },
        signers: [community],
    });
}

async function addUser(communityPubkey: PublicKey, user: PublicKey) {
    await program.methods.addUser(user, {
        accounts: {
            community: communityPubkey,
            admin: provider.wallet.publicKey,
        },
    });
}

async function publishEnergyData(communityPubkey: PublicKey, user: PublicKey, consumption: number, production: number, timestamp: number) {
    const data = { user, consumption, production, timestamp };
    await program.methods.publishEnergyData(user, data, {
        accounts: {
            community: communityPubkey,
            user: provider.wallet.publicKey,
        },
    });
}

async function getCommunityData(communityPubkey: PublicKey) {
    console.log(program.account);
    const communityAccount = await program.account['community'].fetch(communityPubkey);
    console.log('Energy Data:', communityAccount.energyData);
}

// Example Usage
(async () => {
    console.log("Initializing");
    await initialize();
    console.log("Initialized");

    console.log("Creating Community");
    const communityAdmin = Keypair.generate().publicKey;
    await createCommunity(communityAdmin);
    console.log("Community created", communityAdmin);

    console.log("Adding user");
    const user = Keypair.generate().publicKey;
    await addUser(communityAdmin, user);
    console.log("User added", user);

    console.log("Publishing data");
    await publishEnergyData(communityAdmin, user, 100, 50, Date.now());
    console.log("Data published");

    await getCommunityData(communityAdmin);
})();
