import { ethers } from "hardhat";

async function main() {
    const semaphoreAddress = "0x9fE46736679d2D9a65F0992F2272dE9f3c7fa6e0";

    // Get the Contract Factory properly through hardhat ethers
    const CRISPRegistry = await ethers.getContractFactory("CRISPRegistry");

    // Deploy with the semaphore address as a constructor argument
    const registry = await CRISPRegistry.deploy(semaphoreAddress);

    await registry.waitForDeployment();

    console.log("CRISPRegistry deployed at:", await registry.getAddress());
}

main().catch(console.error);