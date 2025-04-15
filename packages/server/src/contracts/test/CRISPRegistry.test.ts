import { expect } from "chai";
import { ethers } from "hardhat";
import { CRISPRegistry } from "../typechain-types";

describe("Check Round Status", function () {
    let crispRegistry: CRISPRegistry;

    before(async function () {
        // Connect to your existing CRISPRegistry contract
        const crispRegistryAddress = "0x0165878A594ca255338adfa4d48449f69242Eb8F";
        crispRegistry = await ethers.getContractAt("CRISPRegistry", crispRegistryAddress) as CRISPRegistry;
    });

    it("Should check if round 2 has a group", async function () {
        // Check initial state
        const initialHasGroup = await crispRegistry.hasGroup(3);
        console.log("Round 2 has a group:", initialHasGroup);
        expect(initialHasGroup).to.be.false;

        // Create a group for round 1 with explicit gas settings
        console.log("Creating group for round 2...");
        const tx = await crispRegistry.createGroupForRound(3, {
            gasLimit: 1000000, // Explicit gas limit
        });

        console.log("Transaction hash:", tx.hash);
        console.log("Waiting for transaction confirmation...");

        const receipt = await tx.wait();
        if (receipt) {
            console.log("Transaction confirmed in block:", receipt.blockNumber);
            console.log("Gas used:", receipt.gasUsed.toString());

            // Check for GroupCreated event
            const groupCreatedEvent = (receipt as any).events?.find((event: any) => event.event === "GroupCreated");
            if (groupCreatedEvent && groupCreatedEvent.args) {
                const groupId = groupCreatedEvent.args.groupId;
                console.log("Group ID from event:", groupId.toString());
            } else {
                console.log("GroupCreated event not found in transaction receipt");
            }
        } else {
            console.error("Transaction receipt is null.");
        }

        // Check the updated state
        const hasGroupAfter = await crispRegistry.hasGroup(3);
        console.log("After creation, round 2 has a group:", hasGroupAfter);
        expect(hasGroupAfter).to.be.true;

        // Get the group ID from the mapping
        const groupId = await crispRegistry.roundToGroupId(3);
        console.log("Group ID from mapping:", groupId.toString());
        expect(groupId).to.be.gt(0);
    });
});