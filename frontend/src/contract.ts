import {
  Contract,
  Networks,
  nativeToScVal,
  scValToNative,
  Address,
  xdr,
  TransactionBuilder,
  BASE_FEE,
  rpc,
  Keypair,
  Transaction,
} from "@stellar/stellar-sdk";

// ─── Network config ─────────────────────────────────────────────────────────
export const NETWORK_CONFIG = {
  testnet: {
    networkPassphrase: Networks.TESTNET,
    rpcUrl: "https://soroban-testnet.stellar.org",
    contractId: import.meta.env.VITE_TESTNET_CONTRACT_ID ?? "",
    xlmSacId: import.meta.env.VITE_TESTNET_XLM_SAC ?? "",
  },
  mainnet: {
    networkPassphrase: Networks.PUBLIC,
    rpcUrl: "https://mainnet.sorobanrpc.com",
    contractId: import.meta.env.VITE_MAINNET_CONTRACT_ID ?? "",
    xlmSacId: import.meta.env.VITE_MAINNET_XLM_SAC ?? "",
  },
} as const;

export type Network = keyof typeof NETWORK_CONFIG;

// ─── Types mirroring the contract ───────────────────────────────────────────
export type ContestStatus = "Active" | "Cancelled" | "Finalized";

export interface PrizeSplit {
  first_pct: number;
  second_pct: number;
  third_pct: number;
}

export interface Contest {
  creator: string;
  token: string;
  entry_fee: bigint;
  prize_pool: bigint;
  details: string;
  max_participants: number;
  prize_split: PrizeSplit;
  winners: string[];
  status: ContestStatus;
}

export interface ContestView {
  contest: Contest;
  participants: string[];
}

// ─── Helpers ─────────────────────────────────────────────────────────────────
function getServer(network: Network) {
  return new rpc.Server(NETWORK_CONFIG[network].rpcUrl, { allowHttp: false });
}

function getContract(network: Network) {
  return new Contract(NETWORK_CONFIG[network].contractId);
}

function parseStatus(raw: unknown): ContestStatus {
  if (raw && typeof raw === "object") {
    const key = Object.keys(raw)[0];
    if (key === "Active") return "Active";
    if (key === "Cancelled") return "Cancelled";
    if (key === "Finalized") return "Finalized";
  }
  return "Active";
}

function parseContestView(raw: unknown): ContestView {
  const map = raw as Record<string, unknown>;
  const c = map["contest"] as Record<string, unknown>;
  const split = c["prize_split"] as Record<string, unknown>;
  return {
    contest: {
      creator: String(c["creator"]),
      token: String(c["token"]),
      entry_fee: BigInt(String(c["entry_fee"])),
      prize_pool: BigInt(String(c["prize_pool"])),
      details: String(c["details"]),
      max_participants: Number(c["max_participants"]),
      prize_split: {
        first_pct: Number(split["first_pct"]),
        second_pct: Number(split["second_pct"]),
        third_pct: Number(split["third_pct"]),
      },
      winners: (c["winners"] as unknown[]).map(String),
      status: parseStatus(c["status"]),
    },
    participants: (map["participants"] as unknown[]).map(String),
  };
}

// ─── Read-only simulation ────────────────────────────────────────────────────
export async function getContest(
  network: Network,
  contestId: number
): Promise<ContestView | null> {
  const server = getServer(network);
  const contract = getContract(network);
  const { networkPassphrase } = NETWORK_CONFIG[network];

  // Dummy account for simulation — sequence number doesn't matter for reads
  const dummyKey = Keypair.random().publicKey();
  const fakeAccount = {
    accountId: () => dummyKey,
    sequenceNumber: () => "0",
    incrementSequenceNumber() {},
  };

  const tx = new TransactionBuilder(fakeAccount as never, {
    fee: BASE_FEE,
    networkPassphrase,
  })
    .addOperation(
      contract.call("get_contest", nativeToScVal(contestId, { type: "u64" }))
    )
    .setTimeout(300)
    .build();

  const sim = await server.simulateTransaction(tx);
  if (rpc.Api.isSimulationError(sim)) return null;
  if (!("result" in sim) || !sim.result?.retval) return null;

  const native = scValToNative(sim.result.retval);
  if (!native) return null;
  return parseContestView(native);
}

// ─── Transaction builders ─────────────────────────────────────────────────────
async function buildAndAssemble(
  network: Network,
  signerAddress: string,
  operation: ReturnType<Contract["call"]>
): Promise<string> {
  const server = getServer(network);
  const { networkPassphrase } = NETWORK_CONFIG[network];
  const account = await server.getAccount(signerAddress);

  const tx = new TransactionBuilder(account, {
    fee: BASE_FEE,
    networkPassphrase,
  })
    .addOperation(operation)
    .setTimeout(300)
    .build();

  const sim = await server.simulateTransaction(tx);
  if (rpc.Api.isSimulationError(sim))
    throw new Error((sim as rpc.Api.SimulateTransactionErrorResponse).error);

  return rpc.assembleTransaction(tx, sim).build().toXDR();
}

export async function buildCreateContest(
  network: Network,
  signerAddress: string,
  params: {
    contestId: number;
    entryFee: bigint;
    details: string;
    maxParticipants: number;
    prizeSplit: PrizeSplit;
  }
): Promise<string> {
  const contract = getContract(network);
  const { xlmSacId } = NETWORK_CONFIG[network];

  const splitScVal = xdr.ScVal.scvMap([
    new xdr.ScMapEntry({
      key: xdr.ScVal.scvSymbol("first_pct"),
      val: nativeToScVal(params.prizeSplit.first_pct, { type: "u32" }),
    }),
    new xdr.ScMapEntry({
      key: xdr.ScVal.scvSymbol("second_pct"),
      val: nativeToScVal(params.prizeSplit.second_pct, { type: "u32" }),
    }),
    new xdr.ScMapEntry({
      key: xdr.ScVal.scvSymbol("third_pct"),
      val: nativeToScVal(params.prizeSplit.third_pct, { type: "u32" }),
    }),
  ]);

  return buildAndAssemble(
    network,
    signerAddress,
    contract.call(
      "create_contest",
      nativeToScVal(params.contestId, { type: "u64" }),
      new Address(signerAddress).toScVal(),
      new Address(xlmSacId).toScVal(),
      nativeToScVal(params.entryFee, { type: "i128" }),
      nativeToScVal(params.details, { type: "string" }),
      nativeToScVal(params.maxParticipants, { type: "u32" }),
      splitScVal
    )
  );
}

export async function buildJoinContest(
  network: Network,
  signerAddress: string,
  contestId: number
): Promise<string> {
  const contract = getContract(network);
  return buildAndAssemble(
    network,
    signerAddress,
    contract.call(
      "join_contest",
      nativeToScVal(contestId, { type: "u64" }),
      new Address(signerAddress).toScVal()
    )
  );
}

export async function buildDeclareWinners(
  network: Network,
  signerAddress: string,
  contestId: number,
  winners: string[]
): Promise<string> {
  const contract = getContract(network);
  return buildAndAssemble(
    network,
    signerAddress,
    contract.call(
      "declare_winners",
      nativeToScVal(contestId, { type: "u64" }),
      xdr.ScVal.scvVec(winners.map((w) => new Address(w).toScVal()))
    )
  );
}

export async function buildCancelContest(
  network: Network,
  signerAddress: string,
  contestId: number
): Promise<string> {
  const contract = getContract(network);
  return buildAndAssemble(
    network,
    signerAddress,
    contract.call(
      "cancel_contest",
      nativeToScVal(contestId, { type: "u64" })
    )
  );
}

// ─── Submit a signed XDR ─────────────────────────────────────────────────────
export async function submitTransaction(
  network: Network,
  signedXdr: string
): Promise<string> {
  const server = getServer(network);
  const { networkPassphrase } = NETWORK_CONFIG[network];
  const tx = TransactionBuilder.fromXDR(signedXdr, networkPassphrase) as Transaction;

  const response = await server.sendTransaction(tx);
  if (response.status === "ERROR")
    throw new Error(JSON.stringify(response.errorResult));

  let result = await server.getTransaction(response.hash);
  for (let i = 0; i < 20 && result.status === "NOT_FOUND"; i++) {
    await new Promise((r) => setTimeout(r, 1500));
    result = await server.getTransaction(response.hash);
  }
  if (result.status === "FAILED") throw new Error("Transaction failed on-chain");
  return response.hash;
}
