// import { expect } from "chai";
// import { ethers } from "hardhat";
// import { CRISPRegistry } from "../typechain-types";
//
// describe("CRISPRegistry Integration Test", function () {
//     let crispRegistry: CRISPRegistry;
//     const crispRegistryAddress = "0xdc64a140aa3e981100a9beca4e685f962f0cf6c9";
//
//     before(async function () {
//         // Connect to your existing CRISPRegistry contract
//         crispRegistry = await ethers.getContractAt("CRISPRegistry", crispRegistryAddress) as CRISPRegistry;
//     });
//
//     it("Should create a group for round 1 and join it", async function () {
//         // First check if the round already has a group
//         const hasGroupBefore = await crispRegistry.hasGroup(1);
//         console.log(`Before: Round 1 has a group: ${hasGroupBefore}`);
//
//         if (!hasGroupBefore) {
//             // Create a group for round 1
//             console.log("Creating group for round 1...");
//             const tx = await crispRegistry.createGroupForRound(1);
//             const receipt = await tx.wait();
//             console.log("Group creation transaction hash:", receipt?.hash || "No receipt");
//         }
//
//         // Verify the group was created
//         const hasGroupAfter = await crispRegistry.hasGroup(1);
//         console.log(`After: Round 1 has a group: ${hasGroupAfter}`);
//         expect(hasGroupAfter).to.be.true;
//
//         // Check the group ID
//         const groupId = await crispRegistry.roundToGroupId(1);
//         console.log("Group ID for round 1:", groupId.toString());
//         expect(groupId).to.be.gt(0);
//
//         // Sample identity commitment (this would normally be generated client-side)
//         const identityCommitment = "21663839004416932945382355908790599225266501822907911457504978515578255421292";
//
//         // Join the round
//         console.log("Joining round 1...");
//         const joinTx = await crispRegistry.joinRound(1, identityCommitment);
//         const joinReceipt = await joinTx.wait();
//         console.log("Join transaction hash:", joinReceipt?.hash || "No receipt");
//
//         console.log("Successfully joined round 1 with identity commitment");
//     });
// });
// scripts/create-group.ts
import { ethers } from "hardhat";

async function main() {
    const contractAddress = "0x0165878A594ca255338adfa4d48449f69242Eb8F";

    // Connect to the contract
    const CRISPRegistry = await ethers.getContractAt("CRISPRegistry", contractAddress);

    // Create a group for round 1
    console.log("Creating group for round 1...");
    const tx = await CRISPRegistry.createGroupForRound(1);
    await tx.wait();

    // Check if the group was created
    const hasGroup = await CRISPRegistry.hasGroup(1);
    console.log("Round 1 has group:", hasGroup);

    // Get the group ID
    const groupId = await CRISPRegistry.roundToGroupId(1);
    console.log("Group ID for round 1:", groupId.toString());

}

main()
    .then(() => process.exit(0))
    .catch(error => {
        console.error(error);
        process.exit(1);
    });