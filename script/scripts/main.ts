import { config } from "dotenv";
import * as utils from "../src/utils";
import { CLAccountHash, CLByteArray, CLValueBuilder, Keys, RuntimeArgs } from "casper-js-sdk";
import { install, reserveWise } from "../src/install";
config();

const {
  NODE_ADDRESS,
  MASTER_KEY_PAIR_PATH,
  BASE_PATH,
  // WASMS
  ERC20_WASM,
  FACTORY_WASM,
  WCSPR_WASM,
  LIBRARY_WASM,
  ROUTER_WASM,
  FLASH_SWAPPER_WASM,
  LIQUIDITY_GUARD_WASM,
  PAIR_WASM,
  SCSPR_WASM,
  STAKEABLE_WASM,
  LIQUIDITY_TRANSFORMER_WASM,
  LT_SESSION_WASM_PATH,
  // PAYMENT
  PAYMENT_ERC20,
  PAYMENT_FACTORY,
  PAYMENT_WCSPR,
  PAYMENT_LIBRARY,
  PAYMENT_ROUTER,
  PAYMENT_FLASH_SWAPPER,
  PAYMENT_LIQUIDITY_GUARD,
  PAYMENT_PAIR,
  PAYMENT_SCSPR,
  PAYMENT_STAKEABLE,
  PAYMENT_LIQUIDITY_TRANSFORMER,
  RESERVE_WISE_PAYMENT_AMOUNT,
  // NAMES
  DAI,
  STABLE_USD,
  FACTORY,
  WCSPR,
  LIBRARY,
  ROUTER,
  FLASH_SWAPPER,
  LIQUIDITY_GUARD,
  PAIR_WISE,
  PAIR_SCSPR,
  PAIR_SUSD,
  SCSPR,
  STAKEABLE,
  LIQUIDITY_TRANSFORMER,
  // PARAMS
  FEE_TO_SETTER,
  SCSPR_AMOUNT,
  STAKEABLE_AMOUNT,
  LIQUIDITY_TRANSFORMER_AMOUNT,
  INVESTMENT_MODE,
  MSG_VALUE
} = process.env;

const KEYS = Keys.Ed25519.parseKeyFiles(
  `${MASTER_KEY_PAIR_PATH}/public_key.pem`,
  `${MASTER_KEY_PAIR_PATH}/secret_key.pem`
);

const zip = require("zip-local");

const deployContract = async () => {
  // GET A DEPLOYMENT COUNT (VERSION)
  let version = utils.getDeploymentCount();
  let contractName, runtimeArgs, installHash;

  // --- DAI --- //

  console.log("dai being deployed...");

  contractName = DAI + version;
  runtimeArgs = RuntimeArgs.fromMap({
    name: CLValueBuilder.string("DaiToken"),
    symbol: CLValueBuilder.string("DAI"),
    decimals: CLValueBuilder.u8("9"),
    initial_supply: CLValueBuilder.u256("0"),
    contract_name: CLValueBuilder.string(contractName),
  });
  installHash = await install(
    KEYS,
    PAYMENT_ERC20!,
    ERC20_WASM!,
    runtimeArgs
  );
  await utils.getDeploy(NODE_ADDRESS!, installHash);
  let daiContractHash = await utils.getContractHash(contractName);
  let daiPackageHash = await utils.getPackageHash(contractName);
  utils.writeHashToFile(BASE_PATH + "daiContractHash", daiContractHash);
  utils.writeHashToFile(BASE_PATH + "daiPackageHash", daiPackageHash);

  console.log("dai deployed and saved");

  // --- STABLE USD --- //

  console.log("stable usd being deployed...");

  contractName = STABLE_USD + version;
  runtimeArgs = RuntimeArgs.fromMap({
    name: CLValueBuilder.string("StableUSDToken"),
    symbol: CLValueBuilder.string("SUSD"),
    decimals: CLValueBuilder.u8("9"),
    initial_supply: CLValueBuilder.u256("0"),
    contract_name: CLValueBuilder.string(contractName),
  });
  installHash = await install(
    KEYS,
    PAYMENT_ERC20!,
    ERC20_WASM!,
    runtimeArgs
  );
  await utils.getDeploy(NODE_ADDRESS!, installHash);
  let stableUSDContractHash = await utils.getContractHash(contractName);
  let stableUSDPackageHash = await utils.getPackageHash(contractName);
  utils.writeHashToFile(BASE_PATH + "stableUSDContractHash", stableUSDContractHash);
  utils.writeHashToFile(BASE_PATH + "stableUSDPackageHash", stableUSDPackageHash);

  console.log("stable usd deployed and saved");

  // --- FACTORY --- //

  console.log("factory being deployed...");

  contractName = FACTORY + version;
  runtimeArgs = RuntimeArgs.fromMap({
    fee_to_setter: CLValueBuilder.key(new CLAccountHash(Uint8Array.from(Buffer.from(FEE_TO_SETTER!, "hex")))),
    contract_name: CLValueBuilder.string(contractName),
  });
  installHash = await install(
    KEYS,
    PAYMENT_FACTORY!,
    FACTORY_WASM!,
    runtimeArgs
  );
  await utils.getDeploy(NODE_ADDRESS!, installHash);
  let factoryContractHash = await utils.getContractHash(contractName);
  let factoryPackageHash = await utils.getPackageHash(contractName);
  utils.writeHashToFile(BASE_PATH + "factoryContractHash", factoryContractHash);
  utils.writeHashToFile(BASE_PATH + "factoryPackageHash", factoryPackageHash);

  console.log("factory deployed and saved");

  // --- WCSPR --- //

  console.log("wcspr being deployed...");

  contractName = WCSPR + version;
  runtimeArgs = RuntimeArgs.fromMap({
    name: CLValueBuilder.string("WrappedCsprToken"),
    symbol: CLValueBuilder.string("WCSPR"),
    decimals: CLValueBuilder.u8("9"),
    contract_name: CLValueBuilder.string(contractName),
  });
  installHash = await install(
    KEYS,
    PAYMENT_WCSPR!,
    WCSPR_WASM!,
    runtimeArgs
  );
  await utils.getDeploy(NODE_ADDRESS!, installHash);
  let wcsprContractHash = await utils.getContractHash(contractName);
  let wcsprPackageHash = await utils.getPackageHash(contractName);
  utils.writeHashToFile(BASE_PATH + "wcsprContractHash", wcsprContractHash);
  utils.writeHashToFile(BASE_PATH + "wcsprPackageHash", wcsprPackageHash);

  console.log("wcspr deployed and saved");

  // --- LIBRARY --- //

  console.log("library being deployed...");

  contractName = LIBRARY + version;
  runtimeArgs = RuntimeArgs.fromMap({
    contract_name: CLValueBuilder.string(contractName),
  });
  installHash = await install(
    KEYS,
    PAYMENT_LIBRARY!,
    LIBRARY_WASM!,
    runtimeArgs
  );
  await utils.getDeploy(NODE_ADDRESS!, installHash);
  let libraryContractHash = await utils.getContractHash(contractName);
  let libraryPackageHash = await utils.getPackageHash(contractName);
  utils.writeHashToFile(BASE_PATH + "libraryContractHash", libraryContractHash);
  utils.writeHashToFile(BASE_PATH + "libraryPackageHash", libraryPackageHash);

  console.log("library deployed and saved");

  // --- ROUTER --- //

  console.log("router being deployed...");

  contractName = ROUTER + version;
  runtimeArgs = RuntimeArgs.fromMap({
    factory: CLValueBuilder.key(new CLByteArray(Uint8Array.from(Buffer.from(factoryPackageHash, "hex")))),
    wcspr: CLValueBuilder.key(new CLByteArray(Uint8Array.from(Buffer.from(wcsprPackageHash, "hex")))),
    library: CLValueBuilder.key(new CLByteArray(Uint8Array.from(Buffer.from(libraryPackageHash, "hex")))),
    contract_name: CLValueBuilder.string(contractName),
  });
  installHash = await install(
    KEYS,
    PAYMENT_ROUTER!,
    ROUTER_WASM!,
    runtimeArgs
  );
  await utils.getDeploy(NODE_ADDRESS!, installHash);
  let routerContractHash = await utils.getContractHash(contractName);
  let routerPackageHash = await utils.getPackageHash(contractName);
  utils.writeHashToFile(BASE_PATH + "routerContractHash", routerContractHash);
  utils.writeHashToFile(BASE_PATH + "routerPackageHash", routerPackageHash);

  console.log("router deployed and saved");

  // --- FLASH SWAPPER --- //

  console.log("flash swapper being deployed...");

  contractName = FLASH_SWAPPER + version;
  runtimeArgs = RuntimeArgs.fromMap({
    uniswap_v2_factory: CLValueBuilder.key(new CLByteArray(Uint8Array.from(Buffer.from(factoryPackageHash, "hex")))),
    wcspr: CLValueBuilder.key(new CLByteArray(Uint8Array.from(Buffer.from(wcsprPackageHash, "hex")))),
    dai: CLValueBuilder.key(new CLByteArray(Uint8Array.from(Buffer.from(daiPackageHash, "hex")))),
    contract_name: CLValueBuilder.string(contractName),
  });
  installHash = await install(
    KEYS,
    PAYMENT_FLASH_SWAPPER!,
    FLASH_SWAPPER_WASM!,
    runtimeArgs
  );
  await utils.getDeploy(NODE_ADDRESS!, installHash);
  let flashSwapperContractHash = await utils.getContractHash(contractName);
  let flashSwapperPackageHash = await utils.getPackageHash(contractName);
  utils.writeHashToFile(BASE_PATH + "flashSwapperContractHash", flashSwapperContractHash);
  utils.writeHashToFile(BASE_PATH + "flashSwapperPackageHash", flashSwapperPackageHash);

  console.log("flash swapper deployed and saved");

  // --- LIQUIDITY GUARD --- //

  console.log("liquidity guard being deployed...");

  contractName = LIQUIDITY_GUARD + version;
  runtimeArgs = RuntimeArgs.fromMap({
    contract_name: CLValueBuilder.string(contractName),
  });
  installHash = await install(
    KEYS,
    PAYMENT_LIQUIDITY_GUARD!,
    LIQUIDITY_GUARD_WASM!,
    runtimeArgs
  );
  await utils.getDeploy(NODE_ADDRESS!, installHash);
  let liquidityGuardContractHash = await utils.getContractHash(contractName);
  let liquidityGuardPackageHash = await utils.getPackageHash(contractName);
  utils.writeHashToFile(BASE_PATH + "liquidityGuardContractHash", liquidityGuardContractHash);
  utils.writeHashToFile(BASE_PATH + "liquidityGuardPackageHash", liquidityGuardPackageHash);

  console.log("liquidity guard deployed and saved");

  // --- PAIR WISE --- //

  console.log("pair wise being deployed...");

  contractName = PAIR_WISE + version;
  runtimeArgs = RuntimeArgs.fromMap({
    name: CLValueBuilder.string("PairWiseScspr"),
    symbol: CLValueBuilder.string("WiseScspr"),
    decimals: CLValueBuilder.u8("9"),
    initial_supply: CLValueBuilder.u256("0"),
    callee_package_hash: CLValueBuilder.key(new CLByteArray(Uint8Array.from(Buffer.from(flashSwapperPackageHash, "hex")))),
    factory_hash: CLValueBuilder.key(new CLByteArray(Uint8Array.from(Buffer.from(factoryPackageHash, "hex")))),
    contract_name: CLValueBuilder.string(contractName),
  });
  installHash = await install(
    KEYS,
    PAYMENT_PAIR!,
    PAIR_WASM!,
    runtimeArgs
  );
  await utils.getDeploy(NODE_ADDRESS!, installHash);
  let pairWiseContractHash = await utils.getContractHash(contractName);
  let pairWisePackageHash = await utils.getPackageHash(contractName);
  utils.writeHashToFile(BASE_PATH + "pairWiseContractHash", pairWiseContractHash);
  utils.writeHashToFile(BASE_PATH + "pairWisePackageHash", pairWisePackageHash);

  console.log("pair wise deployed and saved");

  // --- PAIR SCSPR --- //

  console.log("pair scspr being deployed...");

  contractName = PAIR_SCSPR + version;
  runtimeArgs = RuntimeArgs.fromMap({
    name: CLValueBuilder.string("PairScsprWcspr"),
    symbol: CLValueBuilder.string("ScsprWcspr"),
    decimals: CLValueBuilder.u8("9"),
    initial_supply: CLValueBuilder.u256("0"),
    callee_package_hash: CLValueBuilder.key(new CLByteArray(Uint8Array.from(Buffer.from(flashSwapperPackageHash, "hex")))),
    factory_hash: CLValueBuilder.key(new CLByteArray(Uint8Array.from(Buffer.from(factoryPackageHash, "hex")))),
    contract_name: CLValueBuilder.string(contractName),
  });
  installHash = await install(
    KEYS,
    PAYMENT_PAIR!,
    PAIR_WASM!,
    runtimeArgs
  );
  await utils.getDeploy(NODE_ADDRESS!, installHash);
  let pairScsprContractHash = await utils.getContractHash(contractName);
  let pairScsprPackageHash = await utils.getPackageHash(contractName);
  utils.writeHashToFile(BASE_PATH + "pairScsprContractHash", pairScsprContractHash);
  utils.writeHashToFile(BASE_PATH + "pairScsprPackageHash", pairScsprPackageHash);

  console.log("pair scspr deployed and saved");

  // --- PAIR STABLE USD --- //

  console.log("pair stable usd being deployed...");

  contractName = PAIR_SUSD + version;
  runtimeArgs = RuntimeArgs.fromMap({
    name: CLValueBuilder.string("PairSUSDWcspr"),
    symbol: CLValueBuilder.string("SUSDWcspr"),
    decimals: CLValueBuilder.u8("9"),
    initial_supply: CLValueBuilder.u256("0"),
    callee_package_hash: CLValueBuilder.key(new CLByteArray(Uint8Array.from(Buffer.from(flashSwapperPackageHash, "hex")))),
    factory_hash: CLValueBuilder.key(new CLByteArray(Uint8Array.from(Buffer.from(factoryPackageHash, "hex")))),
    contract_name: CLValueBuilder.string(contractName),
  });
  installHash = await install(
    KEYS,
    PAYMENT_PAIR!,
    PAIR_WASM!,
    runtimeArgs
  );
  await utils.getDeploy(NODE_ADDRESS!, installHash);
  let pairSusdContractHash = await utils.getContractHash(contractName);
  let pairSusdPackageHash = await utils.getPackageHash(contractName);
  utils.writeHashToFile(BASE_PATH + "pairSusdContractHash", pairSusdContractHash);
  utils.writeHashToFile(BASE_PATH + "pairSusdPackageHash", pairSusdPackageHash);

  console.log("pair stable usd deployed and saved");

  // --- SCSPR --- //

  console.log("scspr being deployed...");

  contractName = SCSPR + version;
  runtimeArgs = RuntimeArgs.fromMap({
    wcspr: CLValueBuilder.key(new CLByteArray(Uint8Array.from(Buffer.from(wcsprPackageHash, "hex")))),
    uniswap_pair: CLValueBuilder.key(new CLByteArray(Uint8Array.from(Buffer.from(pairScsprPackageHash, "hex")))),
    uniswap_router: CLValueBuilder.key(new CLByteArray(Uint8Array.from(Buffer.from(routerPackageHash, "hex")))),
    uniswap_factory: CLValueBuilder.key(new CLByteArray(Uint8Array.from(Buffer.from(factoryPackageHash, "hex")))),
    amount: CLValueBuilder.u512(SCSPR_AMOUNT!),
    contract_name: CLValueBuilder.string(contractName),
  });
  installHash = await install(
    KEYS,
    PAYMENT_SCSPR!,
    SCSPR_WASM!,
    runtimeArgs
  );
  await utils.getDeploy(NODE_ADDRESS!, installHash);
  let scsprContractHash = await utils.getContractHash(contractName);
  let scsprPackageHash = await utils.getPackageHash(contractName);
  utils.writeHashToFile(BASE_PATH + "scsprContractHash", scsprContractHash);
  utils.writeHashToFile(BASE_PATH + "scsprPackageHash", scsprPackageHash);

  console.log("scspr deployed and saved");

  // --- STAKEABLE --- //

  console.log("stakeable being deployed...");

  contractName = STAKEABLE + version;
  runtimeArgs = RuntimeArgs.fromMap({
    stable_usd: CLValueBuilder.key(new CLByteArray(Uint8Array.from(Buffer.from(stableUSDPackageHash, "hex")))),
    scspr: CLValueBuilder.key(new CLByteArray(Uint8Array.from(Buffer.from(scsprPackageHash, "hex")))),
    wcspr: CLValueBuilder.key(new CLByteArray(Uint8Array.from(Buffer.from(wcsprPackageHash, "hex")))),
    uniswap_router: CLValueBuilder.key(new CLByteArray(Uint8Array.from(Buffer.from(routerPackageHash, "hex")))),
    uniswap_factory: CLValueBuilder.key(new CLByteArray(Uint8Array.from(Buffer.from(factoryPackageHash, "hex")))),
    uniswap_pair: CLValueBuilder.key(new CLByteArray(Uint8Array.from(Buffer.from(pairWisePackageHash, "hex")))),
    liquidity_guard: CLValueBuilder.key(new CLByteArray(Uint8Array.from(Buffer.from(liquidityGuardPackageHash, "hex")))),
    amount: CLValueBuilder.u512(STAKEABLE_AMOUNT!),
    contract_name: CLValueBuilder.string(contractName),
  });
  installHash = await install(
    KEYS,
    PAYMENT_STAKEABLE!,
    STAKEABLE_WASM!,
    runtimeArgs
  );
  await utils.getDeploy(NODE_ADDRESS!, installHash);
  let stakeableContractHash = await utils.getContractHash(contractName);
  let stakeablePackageHash = await utils.getPackageHash(contractName);
  utils.writeHashToFile(BASE_PATH + "stakeableContractHash", stakeableContractHash);
  utils.writeHashToFile(BASE_PATH + "stakeablePackageHash", stakeablePackageHash);

  console.log("stakeable deployed and saved");

  // --- LIQUIDITY TRANSFORMER --- //

  console.log("liquidity transformer being deployed...");

  contractName = LIQUIDITY_TRANSFORMER + version;
  runtimeArgs = RuntimeArgs.fromMap({
    wise: CLValueBuilder.key(new CLByteArray(Uint8Array.from(Buffer.from(stakeablePackageHash, "hex")))),
    scspr: CLValueBuilder.key(new CLByteArray(Uint8Array.from(Buffer.from(scsprPackageHash, "hex")))),
    pair_wise: CLValueBuilder.key(new CLByteArray(Uint8Array.from(Buffer.from(pairWisePackageHash, "hex")))),
    pair_scspr: CLValueBuilder.key(new CLByteArray(Uint8Array.from(Buffer.from(pairScsprPackageHash, "hex")))),
    uniswap_router: CLValueBuilder.key(new CLByteArray(Uint8Array.from(Buffer.from(routerPackageHash, "hex")))),
    wcspr: CLValueBuilder.key(new CLByteArray(Uint8Array.from(Buffer.from(wcsprPackageHash, "hex")))),
    amount: CLValueBuilder.u512(LIQUIDITY_TRANSFORMER_AMOUNT!),
    contract_name: CLValueBuilder.string(contractName),
  });
  installHash = await install(
    KEYS,
    PAYMENT_LIQUIDITY_TRANSFORMER!,
    LIQUIDITY_TRANSFORMER_WASM!,
    runtimeArgs
  );
  await utils.getDeploy(NODE_ADDRESS!, installHash);
  let liquidityTransformerContractHash = await utils.getContractHash(contractName);
  let liquidityTransformerPackageHash = await utils.getPackageHash(contractName);
  utils.writeHashToFile(BASE_PATH + "liquidityTransformerContractHash", liquidityTransformerContractHash);
  utils.writeHashToFile(BASE_PATH + "liquidityTransformerPackageHash", liquidityTransformerPackageHash);

  console.log("liquidity transformer deployed and saved");

  // UPDATE THE DEPLOYMENT COUNT (VERSION)
  utils.updateDeploymentCount();

  // let reserveWiseDeployHash;

  // // --- RESERVE WISE --- //

  // reserveWiseDeployHash = await reserveWise(
  //   KEYS,
  //   RESERVE_WISE_PAYMENT_AMOUNT!,
  //   LT_SESSION_WASM_PATH!,
  //   liquidityTransformerPackageHash,
  //   INVESTMENT_MODE!,
  //   MSG_VALUE!
  // );
  // await utils.getDeploy(NODE_ADDRESS!, reserveWiseDeployHash);
  // utils.writeHashToFile(BASE_PATH + "reserveWiseDeployHash", reserveWiseDeployHash);

  // console.log("reserve wise call success and saved");

  // --- HASHES ZIP CREATED --- //

  zip.sync.zip("hashes").compress().save("hashes.zip");

  console.log("hashes zip created successfully...");
};

deployContract();