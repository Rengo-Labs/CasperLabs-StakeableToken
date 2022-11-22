import {
  CasperClient,
  CLByteArray,
  CLPublicKey,
  CLValueBuilder,
  DeployUtil,
  Keys,
  RuntimeArgs,
} from "casper-js-sdk";
import * as utils from "./utils";
import { config } from "dotenv";
config();
const { CHAIN_NAME, NODE_ADDRESS } = process.env;

export const install = async (
  keys: Keys.AsymmetricKey,
  paymentAmount: string,
  wasmPath: string,
  runtimeArgs: RuntimeArgs
) => {
  const deployHash = await installWasmFile({
    chainName: CHAIN_NAME!,
    paymentAmount,
    nodeAddress: NODE_ADDRESS!,
    keys,
    pathToContract: wasmPath,
    runtimeArgs,
  });
  if (deployHash !== null) {
    return deployHash;
  } else {
    throw Error("Problem with installation");
  }
}

export const reserveWise = async (
  keys: Keys.AsymmetricKey,
  paymentAmount: string,
  sessionWasmPath: string,
  packageHash: string,
  investmentMode: string,
  msgValue: string,
) => {
  const ltPackageHash = new CLByteArray(
    Uint8Array.from(Buffer.from(packageHash, "hex"))
  );
  const runtimeArgs = RuntimeArgs.fromMap({
    package_hash: CLValueBuilder.key(ltPackageHash),
    entrypoint: CLValueBuilder.string("reserve_wise"),
    investment_mode: CLValueBuilder.u8(investmentMode),
    amount: CLValueBuilder.u512(msgValue),
  });
  const deployHash = await installWasmFile({
    chainName: CHAIN_NAME!,
    paymentAmount,
    nodeAddress: NODE_ADDRESS!,
    keys,
    pathToContract: sessionWasmPath,
    runtimeArgs,
  });
  if (deployHash !== null) {
    return deployHash;
  } else {
    throw Error("Invalid Deploy");
  }
}


interface IInstallParams {
  nodeAddress: string;
  keys: Keys.AsymmetricKey;
  chainName: string;
  pathToContract: string;
  runtimeArgs: RuntimeArgs;
  paymentAmount: string;
}

const installWasmFile = async ({
  nodeAddress,
  keys,
  chainName,
  pathToContract,
  runtimeArgs,
  paymentAmount,
}: IInstallParams): Promise<string> => {
  const client = new CasperClient(nodeAddress);
  // Set contract installation deploy (unsigned).
  let deploy = DeployUtil.makeDeploy(
    new DeployUtil.DeployParams(
      CLPublicKey.fromHex(keys.publicKey.toHex()),
      chainName
    ),
    DeployUtil.ExecutableDeployItem.newModuleBytes(
      utils.getBinary(pathToContract),
      runtimeArgs
    ),
    DeployUtil.standardPayment(paymentAmount)
  );
  // Sign deploy.
  deploy = client.signDeploy(deploy, keys);
  // Dispatch deploy to node.
  return await client.putDeploy(deploy);
};