import { useState } from "react";
import type { Network, ContestView, PrizeSplit } from "../contract";
import {
  getContest,
  buildCreateContest,
  buildJoinContest,
  buildDeclareWinners,
  buildCancelContest,
} from "../contract";

interface Props {
  network: Network;
  address: string | null;
  signAndSubmit: (xdr: string) => Promise<string>;
}

// XLM has 7 decimal places (1 XLM = 10_000_000 stroops)
function stroopsToXlm(stroops: bigint): string {
  return (Number(stroops) / 10_000_000).toFixed(2);
}

export default function ContestPanel({ network, address, signAndSubmit }: Props) {
  const [contestId, setContestId] = useState("");
  const [view, setView] = useState<ContestView | null>(null);
  const [loading, setLoading] = useState(false);
  const [status, setStatus] = useState("");

  // Create form state
  const [createForm, setCreateForm] = useState({
    id: "",
    details: "",
    entryFeeXlm: "1",
    maxParticipants: "0",
    firstPct: "100",
    secondPct: "0",
    thirdPct: "0",
  });

  // Declare winners form
  const [winnerInputs, setWinnerInputs] = useState(["", "", ""]);

  async function fetchContest() {
    if (!contestId) return;
    setLoading(true);
    try {
      const result = await getContest(network, Number(contestId));
      setView(result);
      setStatus(result ? "" : "Contest not found");
    } catch (e: unknown) {
      setStatus(String(e));
    } finally {
      setLoading(false);
    }
  }

  async function handleCreate() {
    if (!address) return setStatus("Connect wallet first");
    const split: PrizeSplit = {
      first_pct: Number(createForm.firstPct),
      second_pct: Number(createForm.secondPct),
      third_pct: Number(createForm.thirdPct),
    };
    if (split.first_pct + split.second_pct + split.third_pct !== 100)
      return setStatus("Prize split must sum to 100");
    setLoading(true);
    try {
      const xdr = await buildCreateContest(network, address, {
        contestId: Number(createForm.id),
        entryFee: BigInt(Math.round(parseFloat(createForm.entryFeeXlm) * 10_000_000)),
        details: createForm.details,
        maxParticipants: Number(createForm.maxParticipants),
        prizeSplit: split,
      });
      const hash = await signAndSubmit(xdr);
      setStatus(`✅ Contest created! Tx: ${hash.slice(0, 12)}...`);
    } catch (e: unknown) {
      setStatus(`❌ ${String(e)}`);
    } finally {
      setLoading(false);
    }
  }

  async function handleJoin() {
    if (!address) return setStatus("Connect wallet first");
    if (!contestId) return setStatus("Enter a contest ID first");
    setLoading(true);
    try {
      const xdr = await buildJoinContest(network, address, Number(contestId));
      const hash = await signAndSubmit(xdr);
      setStatus(`✅ Joined! Tx: ${hash.slice(0, 12)}...`);
      fetchContest();
    } catch (e: unknown) {
      setStatus(`❌ ${String(e)}`);
    } finally {
      setLoading(false);
    }
  }

  async function handleDeclareWinners() {
    if (!address) return setStatus("Connect wallet first");
    if (!contestId || !view) return;
    const { prize_split } = view.contest;
    const numWinners =
      prize_split.third_pct > 0 ? 3 : prize_split.second_pct > 0 ? 2 : 1;
    const winners = winnerInputs.slice(0, numWinners).filter(Boolean);
    if (winners.length !== numWinners)
      return setStatus(`Need exactly ${numWinners} winner address(es)`);
    setLoading(true);
    try {
      const xdr = await buildDeclareWinners(network, address, Number(contestId), winners);
      const hash = await signAndSubmit(xdr);
      setStatus(`✅ Winners declared! Tx: ${hash.slice(0, 12)}...`);
      fetchContest();
    } catch (e: unknown) {
      setStatus(`❌ ${String(e)}`);
    } finally {
      setLoading(false);
    }
  }

  async function handleCancel() {
    if (!address) return setStatus("Connect wallet first");
    if (!contestId) return;
    setLoading(true);
    try {
      const xdr = await buildCancelContest(network, address, Number(contestId));
      const hash = await signAndSubmit(xdr);
      setStatus(`✅ Cancelled & refunded! Tx: ${hash.slice(0, 12)}...`);
      fetchContest();
    } catch (e: unknown) {
      setStatus(`❌ ${String(e)}`);
    } finally {
      setLoading(false);
    }
  }

  const isCreator = view && address && view.contest.creator === address;
  const numWinners = view
    ? view.contest.prize_split.third_pct > 0
      ? 3
      : view.contest.prize_split.second_pct > 0
      ? 2
      : 1
    : 1;

  return (
    <div className="panel-grid">
      {/* ── Create Contest ── */}
      <section className="card">
        <h2>Create Contest</h2>
        <label>Contest ID
          <input type="number" value={createForm.id} onChange={e => setCreateForm(f => ({ ...f, id: e.target.value }))} />
        </label>
        <label>Details / Description
          <input value={createForm.details} onChange={e => setCreateForm(f => ({ ...f, details: e.target.value }))} />
        </label>
        <label>Entry Fee (XLM)
          <input type="number" step="0.1" min="0" value={createForm.entryFeeXlm} onChange={e => setCreateForm(f => ({ ...f, entryFeeXlm: e.target.value }))} />
        </label>
        <label>Max Participants (0 = unlimited)
          <input type="number" min="0" value={createForm.maxParticipants} onChange={e => setCreateForm(f => ({ ...f, maxParticipants: e.target.value }))} />
        </label>
        <fieldset>
          <legend>Prize Split (must sum to 100%)</legend>
          <label>1st %<input type="number" min="0" max="100" value={createForm.firstPct} onChange={e => setCreateForm(f => ({ ...f, firstPct: e.target.value }))} /></label>
          <label>2nd %<input type="number" min="0" max="100" value={createForm.secondPct} onChange={e => setCreateForm(f => ({ ...f, secondPct: e.target.value }))} /></label>
          <label>3rd %<input type="number" min="0" max="100" value={createForm.thirdPct} onChange={e => setCreateForm(f => ({ ...f, thirdPct: e.target.value }))} /></label>
        </fieldset>
        <button onClick={handleCreate} disabled={loading || !address}>
          {loading ? "…" : "Create Contest"}
        </button>
      </section>

      {/* ── Look up + Join ── */}
      <section className="card">
        <h2>Look Up Contest</h2>
        <div className="row">
          <input
            type="number"
            placeholder="Contest ID"
            value={contestId}
            onChange={e => setContestId(e.target.value)}
          />
          <button onClick={fetchContest} disabled={loading}>
            {loading ? "…" : "Fetch"}
          </button>
        </div>

        {view && (
          <div className="contest-detail">
            <p><strong>Details:</strong> {view.contest.details}</p>
            <p><strong>Status:</strong> <span className={`status status-${view.contest.status.toLowerCase()}`}>{view.contest.status}</span></p>
            <p><strong>Entry Fee:</strong> {stroopsToXlm(view.contest.entry_fee)} XLM</p>
            <p><strong>Prize Pool:</strong> {stroopsToXlm(view.contest.prize_pool)} XLM</p>
            <p><strong>Max Players:</strong> {view.contest.max_participants === 0 ? "Unlimited" : view.contest.max_participants}</p>
            <p><strong>Split:</strong> {view.contest.prize_split.first_pct}% / {view.contest.prize_split.second_pct}% / {view.contest.prize_split.third_pct}%</p>
            <p><strong>Participants ({view.participants.length}):</strong></p>
            <ul className="addr-list">
              {view.participants.map(p => <li key={p} title={p}>{p.slice(0, 8)}…{p.slice(-6)}</li>)}
            </ul>
            {view.contest.winners.length > 0 && (
              <>
                <p><strong>Winners:</strong></p>
                <ul className="addr-list">
                  {view.contest.winners.map((w, i) => (
                    <li key={w}>{["🥇", "🥈", "🥉"][i]} {w.slice(0, 8)}…{w.slice(-6)}</li>
                  ))}
                </ul>
              </>
            )}
            {view.contest.status === "Active" && (
              <button onClick={handleJoin} disabled={loading || !address}>
                Join Contest
              </button>
            )}
          </div>
        )}
      </section>

      {/* ── Creator Controls ── */}
      {view && isCreator && view.contest.status === "Active" && (
        <section className="card">
          <h2>Creator Controls</h2>
          <p className="hint">Declare {numWinners} winner{numWinners > 1 ? "s" : ""} (addresses must be participants)</p>
          {Array.from({ length: numWinners }).map((_, i) => (
            <label key={i}>{["🥇 1st", "🥈 2nd", "🥉 3rd"][i]} Place
              <input
                placeholder="G... address"
                value={winnerInputs[i]}
                onChange={e => {
                  const updated = [...winnerInputs];
                  updated[i] = e.target.value;
                  setWinnerInputs(updated);
                }}
              />
            </label>
          ))}
          <div className="row">
            <button onClick={handleDeclareWinners} disabled={loading}>
              Declare Winners
            </button>
            <button className="btn-danger" onClick={handleCancel} disabled={loading}>
              Cancel & Refund All
            </button>
          </div>
        </section>
      )}

      {status && <p className="status-msg">{status}</p>}
    </div>
  );
}
