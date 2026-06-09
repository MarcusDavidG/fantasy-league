import { useState, useCallback } from "react";
import { StellarWalletsKit } from "@creit.tech/stellar-wallets-kit/sdk";
import { Networks as KitNetworks } from "@creit.tech/stellar-wallets-kit/types";
import { LobstrModule, LOBSTR_ID } from "@creit.tech/stellar-wallets-kit/modules/lobstr";
import { FreighterModule } from "@creit.tech/stellar-wallets-kit/modules/freighter";
import type { Network } from "./contract";
import { NETWORK_CONFIG, submitTransaction } from "./contract";

export interface WalletState {
  address: string | null;
  network: Network;
}

// Initialise kit once (static API — no instantiation needed)
StellarWalletsKit.init({
  network: KitNetworks.TESTNET,
  modules: [new LobstrModule(), new FreighterModule()],
  selectedWalletId: LOBSTR_ID,
});

export function useWallet() {
  const [state, setState] = useState<WalletState>({
    address: null,
    network: "testnet",
  });

  const connect = useCallback(async (network: Network) => {
    const kitNetwork =
      network === "mainnet" ? KitNetworks.PUBLIC : KitNetworks.TESTNET;
    StellarWalletsKit.setNetwork(kitNetwork);

    const { address } = await StellarWalletsKit.authModal();
    setState({ address, network });
  }, []);

  const disconnect = useCallback(() => {
    setState((s) => ({ ...s, address: null }));
  }, []);

  const signAndSubmit = useCallback(
    async (xdrTx: string): Promise<string> => {
      if (!state.address) throw new Error("Not connected");
      const { signedTxXdr } = await StellarWalletsKit.signTransaction(xdrTx, {
        networkPassphrase: NETWORK_CONFIG[state.network].networkPassphrase,
      });
      return submitTransaction(state.network, signedTxXdr);
    },
    [state]
  );

  return { ...state, connect, disconnect, signAndSubmit };
}
