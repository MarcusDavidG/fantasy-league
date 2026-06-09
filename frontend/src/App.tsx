import { useState } from "react";
import { useWallet } from "./wallet";
import ContestPanel from "./components/ContestPanel";
import type { Network } from "./contract";
import "./App.css";

export default function App() {
  const { address, network, connect, disconnect, signAndSubmit } = useWallet();
  const [selectedNetwork, setSelectedNetwork] = useState<Network>("testnet");

  return (
    <div className="app">
      <header className="header">
        <div className="header-left">
          <span className="logo">⚽ Stellar Fantasy League</span>
          <select
            className="network-select"
            value={selectedNetwork}
            onChange={e => setSelectedNetwork(e.target.value as Network)}
          >
            <option value="testnet">Testnet</option>
            <option value="mainnet">Mainnet</option>
          </select>
        </div>
        <div className="header-right">
          {address ? (
            <>
              <span className="addr-badge" title={address}>
                {address.slice(0, 6)}…{address.slice(-4)}
              </span>
              <button className="btn-outline" onClick={disconnect}>Disconnect</button>
            </>
          ) : (
            <button onClick={() => connect(selectedNetwork)}>Connect Wallet</button>
          )}
        </div>
      </header>

      <main className="main">
        <ContestPanel
          network={network ?? selectedNetwork}
          address={address}
          signAndSubmit={signAndSubmit}
        />
      </main>

      <footer className="footer">
        Built on <a href="https://stellar.org" target="_blank" rel="noreferrer">Stellar</a> &nbsp;·&nbsp;
        <a href="https://github.com/marcus/stellar-fantasy-league" target="_blank" rel="noreferrer">GitHub</a>
      </footer>
    </div>
  );
}
