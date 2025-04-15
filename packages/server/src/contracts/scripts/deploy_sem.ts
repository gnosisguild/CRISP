import { ethers } from "hardhat";

async function main() {
    // Make sure the Semaphore contract is available in your contracts directory
    const Semaphore = await ethers.getContractFactory("Semaphore");

    // Deploy the contract
    const semaphore = await Semaphore.deploy();

    // Wait for deployment to complete
    await semaphore.waitForDeployment();

    // Get the contract address
    const address = await semaphore.getAddress();
    console.log("Semaphore deployed to:", address);
}

// Execute the deployment
main()
    .then(() => process.exit(0))
    .catch((error) => {
        console.error(error);
        process.exit(1);
    });