import { loadFixture } from "@nomicfoundation/hardhat-network-helpers";
import { expect } from "chai";
import { ethers } from "hardhat";

const ADDRESS_ONE = "0x0000000000000000000000000000000000000001";

describe("RFVoting", () => {
  async function deployContracts() {
    const [deployer, sender, receiver] = await ethers.getSigners();
    const rfvotingFactory = await ethers.getContractFactory("RFVoting");
    const rfvotingContract = await rfvotingFactory
      .connect(deployer)
      .deploy();

    return { deployer, sender, receiver, rfvotingContract };
  }

  describe("test", async () => {
    it("testing", async () => {
      const { deployer, rfvotingContract } = await loadFixture(deployContracts);

      expect(await rfvotingContract.tester()).to.equal("test");
    });
  });
});
