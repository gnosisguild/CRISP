// scripts/inspect-contract.ts
import { ethers } from "hardhat";
import { JsonRpcProvider } from "ethers";
async function main() {
    const address = "0x0165878A594ca255338adfa4d48449f69242Eb8F";

    // Create a provider explicitly connected to localhost
    const provider = new JsonRpcProvider("http://127.0.0.1:8545");
    // Use this provider to get code
    const code = await provider.getCode(address);

    console.log("Contract code exists:", code !== "0x");
    console.log("Code length:", (code.length - 2) / 2, "bytes");

    if (code !== "0x") {
        // Create a contract instance using explicit provider
        const contract = new ethers.Contract(
            address,
            [
                "function semaphore() view returns (address)",
                "function hasGroup(uint256) view returns (bool)",
                "function roundToGroupId(uint256) view returns (uint256)"
            ],
            provider
        );

        try {
            const semaphoreAddress = await contract.semaphore();
            console.log("Semaphore address:", semaphoreAddress);
        } catch (error) {
            if (error instanceof Error) {
                console.log("no semaphore function :", error.message);
            } else {
                console.log("no semaphore function", error);
            }
        }

        try {
            const hasGroup = await contract.hasGroup(1);
            console.log("hasGroup(1) result:", hasGroup);
        } catch (error) {
            {
                if (error instanceof Error) {
                    console.log("Error with hasGroup:", error.message);
                } else {
                    console.log("Error with hasGroup:", error);
                }
            }

            try {
                const groupId = await contract.roundToGroupId(1);
                console.log("roundToGroupId(1) result:", groupId.toString());
            } catch (error) {
                if (error instanceof Error) {
                    console.log("Error with roundToGroupId:", error.message);
                } else {
                    console.log("Error with roundToGroupId:", error);
                }
            }
        }
    }
}

main()
    .then(() => process.exit(0))
    .catch((error) => {
        console.error(error);
        process.exit(1);
    });