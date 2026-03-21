import { createSignal, onMount, Show } from 'solid-js'
import initWasm, { tableau_tautology, sequent_tautology } from 'pc_proof_wasm'

type ProofNode = {
  label: string
  closed: boolean
  children: ProofNode[]
  rule: string | null
}

type TautologyOutcome =
  | { Proved: { tree: ProofNode } }
  | { Refuted: { tree: ProofNode; model: Record<string, boolean> } }

function formatCounterexample(model: Record<string, boolean>): string {
  return Object.entries(model)
    .sort(([a], [b]) => a.localeCompare(b))
    .map(([k, v]) => `${k}=${v}`)
    .join(', ')
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
    <div style={{ "max-width": "640px", margin: "40px auto", "font-family": "monospace", padding: "0 16px" }}>
      <h1 style={{ "font-size": "1.4rem", "margin-bottom": "24px" }}>pc-proof</h1>

      <div style={{ "margin-bottom": "12px" }}>
        <input
          type="text"
          value={formula()}
          onInput={e => setFormula(e.currentTarget.value)}
          onKeyDown={e => e.key === 'Enter' && check()}
          placeholder="e.g. p or not p"
          style={{ width: "100%", padding: "8px", "font-family": "monospace", "font-size": "1rem", "box-sizing": "border-box" }}
          disabled={!ready()}
        />
      </div>

      <div style={{ "margin-bottom": "12px", display: "flex", gap: "16px", "align-items": "center" }}>
        <label>
          <input type="radio" name="method" value="tableau" checked={method() === 'tableau'} onChange={() => setMethod('tableau')} />
          {' '}tableau
        </label>
        <label>
          <input type="radio" name="method" value="sequent" checked={method() === 'sequent'} onChange={() => setMethod('sequent')} />
          {' '}sequent
        </label>
        <button onClick={check} disabled={!ready()} style={{ "margin-left": "auto", padding: "6px 16px", cursor: "pointer" }}>
          Check
        </button>
      </div>

      <Show when={error()}>
        <pre style={{ color: "red" }}>{error()}</pre>
      </Show>

      <Show when={result()} keyed>
        {outcome => (
          <div style={{ "margin-top": "16px" }}>
            {'Proved' in outcome
              ? <span style={{ color: "green", "font-size": "1.2rem" }}>✓ Tautology</span>
              : (
                <div>
                  <span style={{ color: "red", "font-size": "1.2rem" }}>✗ Not a tautology</span>
                  {'Refuted' in outcome && Object.keys(outcome.Refuted.model).length > 0 && (
                    <div style={{ "margin-top": "8px", color: "#555" }}>
                      Counterexample: {formatCounterexample(outcome.Refuted.model)}
                    </div>
                  )}
                </div>
              )
            }
          </div>
        )}
      </Show>
    </div>
  )
}
