import { createSignal, onMount, Show } from 'solid-js'
import initWasm, { tableau_tautology, sequent_tautology } from 'pc_proof_wasm'
import { ProofTree, ProofNodeData } from './ProofTree'

type TautologyOutcome =
  | { Proved: { tree: ProofNodeData } }
  | { Refuted: { tree: ProofNodeData; model: Record<string, boolean> } }

function formatCounterexample(model: Record<string, boolean>): string {
  return Object.entries(model)
    .sort(([a], [b]) => a.localeCompare(b))
    .map(([k, v]) => `${k} = ${v}`)
    .join(',  ')
}

export default function App() {
  const [ready, setReady] = createSignal(false)
  const [formula, setFormula] = createSignal('p or not p')
  const [method, setMethod] = createSignal<'tableau' | 'sequent'>('tableau')
  const [result, setResult] = createSignal<TautologyOutcome | null>(null)
  const [error, setError] = createSignal<string | null>(null)

  onMount(async () => {
    await initWasm()
    setReady(true)
  })

  function check() {
    setError(null)
    setResult(null)
    try {
      const outcome: TautologyOutcome =
        method() === 'tableau'
          ? tableau_tautology(formula())
          : sequent_tautology(formula())
      setResult(outcome)
    } catch (e) {
      setError(e instanceof Error ? e.message : String(e))
    }
  }

  return (
    <div style={{ "max-width": "860px", margin: "40px auto", "font-family": "system-ui, sans-serif", padding: "0 16px" }}>
      <h1 style={{ "font-size": "1.4rem", "margin-bottom": "8px" }}>pc-proof</h1>
      <p style={{ color: '#888', "font-size": '0.85rem', "margin-bottom": "24px" }}>Propositional logic tautology checker</p>

      <div style={{ display: 'flex', gap: '8px', "margin-bottom": "12px" }}>
        <input
          type="text"
          value={formula()}
          onInput={e => setFormula(e.currentTarget.value)}
          onKeyDown={e => e.key === 'Enter' && check()}
          placeholder="e.g. p or not p"
          style={{ flex: 1, padding: "8px 12px", "font-family": "monospace", "font-size": "1rem", border: "1px solid #ddd", "border-radius": "4px" }}
          disabled={!ready()}
        />
        <button
          onClick={check}
          disabled={!ready()}
          style={{ padding: "8px 20px", cursor: "pointer", "border-radius": "4px", border: "none", background: "#333", color: "#fff", "font-size": "1rem" }}
        >
          Check
        </button>
      </div>

      <div style={{ "margin-bottom": "20px", display: "flex", gap: "20px", "font-size": "0.9rem" }}>
        <label style={{ cursor: "pointer" }}>
          <input type="radio" name="method" checked={method() === 'tableau'} onChange={() => setMethod('tableau')} />
          {' '}Tableau
        </label>
        <label style={{ cursor: "pointer" }}>
          <input type="radio" name="method" checked={method() === 'sequent'} onChange={() => setMethod('sequent')} />
          {' '}Sequent calculus
        </label>
      </div>

      <Show when={error()}>
        <pre style={{ color: "#c00", background: "#fff0f0", padding: "12px", "border-radius": "4px", "font-size": "0.85rem" }}>
          {error()}
        </pre>
      </Show>

      <Show when={result()} keyed>
        {outcome => (
          <div>
            {'Proved' in outcome ? (
              <>
                <div style={{ "font-size": "1.2rem", color: "#2e7d32", "margin-bottom": "16px" }}>
                  ✓ Tautology
                </div>
                <ProofTree node={outcome.Proved.tree} />
              </>
            ) : (
              <div>
                <div style={{ "font-size": "1.2rem", color: "#c62828", "margin-bottom": "8px" }}>
                  ✗ Not a tautology
                </div>
                <Show when={Object.keys(outcome.Refuted.model).length > 0}>
                  <div style={{ "font-family": "monospace", color: "#555", "font-size": "0.95rem" }}>
                    Counterexample: {formatCounterexample(outcome.Refuted.model)}
                  </div>
                </Show>
              </div>
            )}
          </div>
        )}
      </Show>
    </div>
  )
}
