import {
  CasperServiceByJsonRPC,
  CLValue,
  CLKey,
  CLAccountHash,
  Keys,
  CLPublicKey,
  CasperClient,
} from "casper-js-sdk";
import * as fs from "fs";
import { RecipientType } from "./types";
import { config } from "dotenv";
config();

const { BASE_PATH } = process.env;

const { NODE_ADDRESS, MASTER_KEY_PAIR_PATH } = process.env;
const KEYS = Keys.Ed25519.parseKeyFiles(
  `${MASTER_KEY_PAIR_PATH}/public_key.pem`,
  `${MASTER_KEY_PAIR_PATH}/secret_key.pem`
);

/**
 * Returns a binary as u8 array.
 * @param pathToBinary - Path to binary file to be loaded into memory.
 * @return Uint8Array Byte array.
 */
export const getBinary = (pathToBinary: string) => {
  return new Uint8Array(fs.readFileSync(pathToBinary, null).buffer);
};

export const createRecipientAddress = (recipient: RecipientType): CLKey => {
  if (recipient instanceof CLPublicKey) {
    return new CLKey(new CLAccountHash(recipient.toAccountHash()));
  } else {
    return new CLKey(recipient);
  }
};

/**
 * Returns global state root hash at current block.
 * @param {Object} client - JS SDK client for interacting with a node.
 * @return {String} Root hash of global state at most recent block.
 */
export const getStateRootHash = async (nodeAddress: string) => {
  const client = new CasperServiceByJsonRPC(nodeAddress);
  const { block } = await client.getLatestBlockInfo();
  if (block) {
    return block.header.state_root_hash;
  } else {
    throw Error("Problem when calling getLatestBlockInfo");
  }
};

export const getAccountInfo = async (
  nodeAddress: string,
  publicKey: CLPublicKey
) => {
  const stateRootHash = await getStateRootHash(nodeAddress);
  const client = new CasperServiceByJsonRPC(nodeAddress);
  const accountHash = publicKey.toAccountHashStr();
  const blockState = await client.getBlockState(stateRootHash, accountHash, []);
  return blockState.Account;
};

/**
 * Returns a value under an on-chain account's storage.
 * @param accountInfo - On-chain account's info.
 * @param namedKey - A named key associated with an on-chain account.
 */
export const getAccountNamedKeyValue = (accountInfo: any, namedKey: string) => {
  const found = accountInfo.namedKeys.find((i: any) => i.name === namedKey);
  if (found) {
    return found.key;
  }
  return undefined;
};

export const sleep = (ms: number) => {
  return new Promise(resolve => setTimeout(resolve, ms));
}

export const getDeploy = async (NODE_URL: string, deployHash: string) => {
  const client = new CasperClient(NODE_URL);
  let i = 300;
  console.log("Countdown");
  while (i != 0) {
    console.log(i);
    const [deploy, raw] = await client.getDeploy(deployHash);
    if (raw.execution_results.length !== 0) {
      // @ts-ignore
      if (raw.execution_results[0].result.Success) {
        return deploy;
      } else {
        // @ts-ignore
        throw Error("Contract execution: " + raw.execution_results[0].result.Failure.error_message);
      }
    } else {
      i--;
      await sleep(1000);
      continue;
    }
  }
  throw Error('Timeout after ' + i + 's. Something\'s wrong');
}


export const getKeys = (keyPath: string) => {
  let keys = Keys.Ed25519.parseKeyFiles(
    `${keyPath}/public_key.pem`,
    `${keyPath}/secret_key.pem`);
  return keys;
}

export const getDeploymentCount = () => {
  return fs.readFileSync(BASE_PATH + 'deploymentCount', 'utf8');
}

export const updateDeploymentCount = () => {
  let val: bigint = BigInt(fs.readFileSync(BASE_PATH + 'deploymentCount', 'utf8'));
  let newVal = val + BigInt(1);
  fs.writeFileSync(BASE_PATH + 'deploymentCount', newVal.toString(), { encoding: 'utf8', flag: 'w' });
}

export const writeHashToFile = (filename: string, hash: string) => {
  fs.writeFileSync(filename, hash, { encoding: 'utf8', flag: 'w' });
}

export const getContractHash = async (contractName: string) => {
  let info = await getAccountInfo(NODE_ADDRESS!, KEYS.publicKey);
  let contractHash = await getAccountNamedKeyValue(info, `${contractName!}_contract_hash`);
  return contractHash.replace('hash-', '');
}

export const getPackageHash = async (contractName: string) => {
  let info = await getAccountInfo(NODE_ADDRESS!, KEYS.publicKey);
  let packageHash = await getAccountNamedKeyValue(info, `${contractName!}_package_hash`);
  return packageHash.replace('hash-', '');
}