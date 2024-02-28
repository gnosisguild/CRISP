import { DeployFunction } from "hardhat-deploy/types";
import { HardhatRuntimeEnvironment } from "hardhat/types";

const func: DeployFunction = async function (hre: HardhatRuntimeEnvironment) {
  const { deployer } = await hre.getNamedAccounts();
  const { deploy } = hre.deployments;
  console.log(deployer)
  const rfvoting = await deploy("RFVoting", {
    from: deployer,
    args: [],
    log: true,
  });

  console.log(`RFVoting contract: `, rfvoting.address);
};
export default func;
func.id = "deploy_rfvoting"; // id required to prevent reexecution
func.tags = ["RFVoting"];