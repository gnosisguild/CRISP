import { DeployFunction } from "hardhat-deploy/types";
import { HardhatRuntimeEnvironment } from "hardhat/types";

const func: DeployFunction = async function (hre: HardhatRuntimeEnvironment) {
  const { deployer } = await hre.getNamedAccounts();
  const { deploy } = hre.deployments;
  console.log(deployer)
  const crispVoting = await deploy("CRISPVoting", {
    from: deployer,
    args: [],
    log: true,
  });

  console.log(`CRISPVoting contract: `, crispVoting.address);
};
export default func;
func.id = "deploy_crispVoting"; // id required to prevent reexecution
func.tags = ["CRISPVoting"];