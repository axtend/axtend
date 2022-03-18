import { ApiPromise } from "@axia/api";
import { JsonRpcResponse } from "web3-core-helpers";
import type { BlockHash } from "@axia/types/interfaces/chain/types";

import { ethers } from "ethers";
import { startMoonbeamDevNode } from "./dev-node";
import {
  provideWeb3Api,
  provideEthersApi,
  provideAxiaApi,
  EnhancedWeb3,
  customWeb3Request,
} from "./providers";
import { ChildProcess } from "child_process";
import { createAndFinalizeBlock } from "./block";
import { SPAWNING_TIME, DEBUG_MODE } from "./constants";
import { HttpProvider } from "web3-core";
const debug = require("debug")("test:setup");

export interface BlockCreation {
  parentHash?: BlockHash;
  finalize?: boolean;
  transactions?: string[];
}

export interface DevTestContext {
  createWeb3: (protocol?: "ws" | "http") => Promise<EnhancedWeb3>;
  createEthers: () => Promise<ethers.providers.JsonRpcProvider>;
  createAxiaApi: () => Promise<ApiPromise>;

  createBlock: (options?: BlockCreation) => Promise<{
    txResults: JsonRpcResponse[];
    block: {
      duration: number;
      hash: BlockHash;
    };
  }>;

  // We also provided singleton providers for simplicity
  web3: EnhancedWeb3;
  ethers: ethers.providers.JsonRpcProvider;
  axiaApi: ApiPromise;
  rpcPort: number;
  ethTransactionType?: EthTransactionType;
}

interface InternalDevTestContext extends DevTestContext {
  _axiaApis: ApiPromise[];
  _web3Providers: HttpProvider[];
}

type EthTransactionType = "Legacy" | "EIP2930" | "EIP1559";

export function describeDevMoonbeam(
  title: string,
  cb: (context: DevTestContext) => void,
  ethTransactionType: EthTransactionType = "Legacy",
  withWasm?: boolean
) {
  describe(title, function () {
    // Set timeout to 5000 for all tests.
    this.timeout(5000);

    // The context is initialized empty to allow passing a reference
    // and to be filled once the node information is retrieved
    let context: InternalDevTestContext = { ethTransactionType } as InternalDevTestContext;

    // The currently running node for this describe
    let axtendProcess: ChildProcess;

    // Making sure the Moonbeam node has started
    before("Starting Moonbeam Test Node", async function () {
      this.timeout(SPAWNING_TIME);
      const init = !DEBUG_MODE
        ? await startMoonbeamDevNode(withWasm)
        : {
            runningNode: null,
            p2pPort: 19931,
            wsPort: 19933,
            rpcPort: 19932,
          };
      axtendProcess = init.runningNode;
      context.rpcPort = init.rpcPort;

      // Context is given prior to this assignement, so doing
      // context = init.context will fail because it replace the variable;

      context._axiaApis = [];
      context._web3Providers = [];
      axtendProcess = init.runningNode;

      context.createWeb3 = async (protocol: "ws" | "http" = "http") => {
        const provider =
          protocol == "ws"
            ? await provideWeb3Api(init.wsPort, "ws")
            : await provideWeb3Api(init.rpcPort, "http");
        context._web3Providers.push((provider as any)._provider);
        return provider;
      };
      context.createEthers = async () => provideEthersApi(init.rpcPort);
      context.createAxiaApi = async () => {
        const apiPromise = await provideAxiaApi(init.wsPort);
        // We keep track of the axiaApis to close them at the end of the test
        context._axiaApis.push(apiPromise);
        await apiPromise.isReady;
        // Necessary hack to allow axiaApi to finish its internal metadata loading
        // apiPromise.isReady unfortunately doesn't wait for those properly
        await new Promise((resolve) => {
          setTimeout(resolve, 100);
        });

        return apiPromise;
      };

      context.axiaApi = await context.createAxiaApi();
      context.web3 = await context.createWeb3();
      context.ethers = await context.createEthers();

      context.createBlock = async <T>(options: BlockCreation = {}) => {
        let { parentHash, finalize, transactions = [] } = options;

        let txResults = await Promise.all(
          transactions.map((t) => customWeb3Request(context.web3, "eth_sendRawTransaction", [t]))
        );
        const block = await createAndFinalizeBlock(context.axiaApi, parentHash, finalize);
        return {
          txResults,
          block,
        };
      };

      debug(
        `Setup ready [${/:([0-9]+)$/.exec((context.web3.currentProvider as any).host)[1]}] for ${
          this.currentTest.title
        }`
      );
    });

    after(async function () {
      await Promise.all(context._web3Providers.map((p) => p.disconnect()));
      await Promise.all(context._axiaApis.map((p) => p.disconnect()));

      if (axtendProcess) {
        await new Promise((resolve) => {
          axtendProcess.once("exit", resolve);
          axtendProcess.kill();
          axtendProcess = null;
        });
      }
    });

    cb(context);
  });
}

export function describeDevMoonbeamAllEthTxTypes(
  title: string,
  cb: (context: DevTestContext) => void,
  withWasm?: boolean
) {
  let wasm = withWasm !== undefined ? withWasm : false;
  describeDevMoonbeam(title + " (Legacy)", cb, "Legacy", wasm);
  describeDevMoonbeam(title + " (EIP1559)", cb, "EIP1559", wasm);
  describeDevMoonbeam(title + " (EIP2930)", cb, "EIP2930", wasm);
}
