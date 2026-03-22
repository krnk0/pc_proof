import { createSignal, onMount, Show } from 'solid-js'
import initWasm, { tableau_tautology, sequent_tautology } from 'pc_proof_wasm'
import { ProofTree, SequentDerivation, ProofNodeData } from './ProofTree'

type TautologyOutcome =
  | { Proved: { tree: ProofNodeData } }
  | { Refuted: { tree: ProofNodeData; model: Record<string, boolean> } }

function formatCounterexample(model: Record<string, boolean>): string {
  return Object.entries(model)
    .sort(([a], [b]) => a.localeCompare(b))
    .map(([k, v]) => `${k} ↦ ${v ? '⊤' : '⊥'}`)
    .join(',  ')
}

// Fraction-style inference rule box
function RuleBox(props: { name: string; premises: string[]; conclusion: string }) {
  const mono: any = { 'font-family': "'STIX Two Text', 'Cambria', serif", 'font-size': '0.88rem', color: '#444', 'white-space': 'nowrap' }
  return (
    <div style={{ display: 'inline-flex', 'flex-direction': 'column', 'align-items': 'center', 'margin-right': '32px', 'margin-bottom': '16px' }}>
      {/* Premises */}
      <div style={{ display: 'flex', gap: '20px', 'padding-bottom': '4px', 'min-height': '1.2em' }}>
        {props.premises.map(p => <span style={mono}>{p}</span>)}
      </div>
      {/* Bar — stretches to full content width; rule name floats right via absolute */}
      <div style={{ 'align-self': 'stretch', 'border-top': '1px solid #aaa', 'margin-bottom': '4px', 'min-width': '48px', position: 'relative' }}>
        <span style={{
          position: 'absolute',
          left: '100%',
          top: '-0.6em',
          'margin-left': '5px',
          'font-size': '0.70rem',
          color: '#999',
          'white-space': 'nowrap',
        }}>{props.name}</span>
      </div>
      {/* Conclusion */}
      <div style={mono}>{props.conclusion}</div>
    </div>
  )
}

function TableauRules() {
  // Grouped by connective: [positive rule, negated rule]
  const rows = [
    [{ name: 'α∧',  premises: ['A ∧ B'],    conclusion: 'A,  B' },
     { name: 'β¬∧', premises: ['¬(A ∧ B)'], conclusion: '¬A  |  ¬B' }],
    [{ name: 'β∨',  premises: ['A ∨ B'],    conclusion: 'A  |  B' },
     { name: 'α¬∨', premises: ['¬(A ∨ B)'], conclusion: '¬A,  ¬B' }],
    [{ name: 'β→',  premises: ['A → B'],    conclusion: '¬A  |  B' },
     { name: 'α¬→', premises: ['¬(A → B)'], conclusion: 'A,  ¬B' }],
    [{ name: 'α¬¬', premises: ['¬¬A'],       conclusion: 'A' }],
  ]
  return (
    <div style={{ display: 'grid', 'grid-template-columns': 'repeat(2, auto)', 'justify-content': 'start', 'row-gap': '4px' }}>
      {rows.flat().map(r => <RuleBox {...r} />)}
    </div>
  )
}

function SequentRules() {
  const axioms = [
    { name: 'Ax',  premises: [], conclusion: 'A ⊢ A' },
    { name: '⊥L',  premises: [], conclusion: '⊥, Γ ⊢ Δ' },
    { name: '⊤R',  premises: [], conclusion: 'Γ ⊢ Δ, ⊤' },
  ]
  const rules = [
    { name: '¬L',  premises: ['Γ ⊢ Δ, A'],              conclusion: 'Γ, ¬A ⊢ Δ' },
    { name: '¬R',  premises: ['Γ, A ⊢ Δ'],              conclusion: 'Γ ⊢ Δ, ¬A' },
    { name: '∧L',  premises: ['Γ, A, B ⊢ Δ'],           conclusion: 'Γ, A ∧ B ⊢ Δ' },
    { name: '∧R',  premises: ['Γ ⊢ Δ, A', 'Γ ⊢ Δ, B'], conclusion: 'Γ ⊢ Δ, A ∧ B' },
    { name: '∨L',  premises: ['Γ, A ⊢ Δ', 'Γ, B ⊢ Δ'], conclusion: 'Γ, A ∨ B ⊢ Δ' },
    { name: '∨R',  premises: ['Γ ⊢ Δ, A, B'],           conclusion: 'Γ ⊢ Δ, A ∨ B' },
    { name: '→L',  premises: ['Γ ⊢ Δ, A', 'Γ, B ⊢ Δ'], conclusion: 'Γ, A → B ⊢ Δ' },
    { name: '→R',  premises: ['Γ, A ⊢ Δ, B'],           conclusion: 'Γ ⊢ Δ, A → B' },
  ]
  // Grouped by connective: [R (intro), L (elim)]
  const rows = [
    [{ name: '¬R',  premises: ['Γ, A ⊢ Δ'],              conclusion: 'Γ ⊢ Δ, ¬A' },
     { name: '¬L',  premises: ['Γ ⊢ Δ, A'],              conclusion: 'Γ, ¬A ⊢ Δ' }],
    [{ name: '∧R',  premises: ['Γ ⊢ Δ, A', 'Γ ⊢ Δ, B'], conclusion: 'Γ ⊢ Δ, A ∧ B' },
     { name: '∧L',  premises: ['Γ, A, B ⊢ Δ'],           conclusion: 'Γ, A ∧ B ⊢ Δ' }],
    [{ name: '∨R',  premises: ['Γ ⊢ Δ, A, B'],           conclusion: 'Γ ⊢ Δ, A ∨ B' },
     { name: '∨L',  premises: ['Γ, A ⊢ Δ', 'Γ, B ⊢ Δ'], conclusion: 'Γ, A ∨ B ⊢ Δ' }],
    [{ name: '→R',  premises: ['Γ, A ⊢ Δ, B'],           conclusion: 'Γ ⊢ Δ, A → B' },
     { name: '→L',  premises: ['Γ ⊢ Δ, A', 'Γ, B ⊢ Δ'], conclusion: 'Γ, A → B ⊢ Δ' }],
  ]
  const label: any = { 'font-size': '0.72rem', color: '#bbb', 'margin-bottom': '10px', 'letter-spacing': '0.03em' }
  const grid3: any = { display: 'grid', 'grid-template-columns': 'repeat(3, auto)', 'justify-content': 'start', 'margin-bottom': '20px' }
  return (
    <div>
      <div style={label}>Axioms</div>
      <div style={grid3}>
        {axioms.map(r => <RuleBox {...r} />)}
      </div>
      <div style={{ display: 'grid', 'grid-template-columns': 'repeat(2, auto)', 'justify-content': 'start', 'row-gap': '4px' }}>
        {rows.flat().map(r => <RuleBox {...r} />)}
      </div>
    </div>
  )
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

  const segBtn = (m: 'tableau' | 'sequent', label: string) => (
    <button
      onClick={() => { setMethod(m); setResult(null); setError(null) }}
      disabled={!ready()}
      style={{
        padding: '6px 18px',
        border: 'none',
        cursor: ready() ? 'pointer' : 'default',
        background: method() === m ? '#333' : '#f5f5f5',
        color: method() === m ? '#fff' : '#555',
        'font-size': '0.88rem',
        transition: 'background 0.15s',
      }}
    >
      {label}
    </button>
  )

  return (
    <div style={{ 'max-width': '860px', margin: '48px auto', 'font-family': 'system-ui, sans-serif', padding: '0 20px' }}>
      <h1 style={{ 'font-size': '1.3rem', 'margin-bottom': '4px', 'font-weight': 600 }}>Propositional Logic Prover</h1>
      <p style={{ color: '#aaa', 'font-size': '0.82rem', 'margin-bottom': '28px' }}>Propositional logic tautology checker</p>

      {/* Input row */}
      <div style={{ display: 'flex', gap: '8px', 'margin-bottom': '6px' }}>
        <input
          type="text"
          value={formula()}
          onInput={e => setFormula(e.currentTarget.value)}
          onKeyDown={e => e.key === 'Enter' && check()}
          placeholder="e.g. p or not p"
          style={{
            flex: 1,
            padding: '9px 12px',
            'font-family': 'monospace',
            'font-size': '1rem',
            border: '1px solid #ddd',
            'border-radius': '6px',
            outline: 'none',
          }}
          disabled={!ready()}
        />
        <button
          onClick={check}
          disabled={!ready()}
          style={{
            padding: '9px 22px',
            cursor: ready() ? 'pointer' : 'default',
            'border-radius': '6px',
            border: 'none',
            background: '#333',
            color: '#fff',
            'font-size': '0.95rem',
          }}
        >
          Check
        </button>
      </div>

      {/* Syntax hint */}
      <div style={{ 'font-size': '0.78rem', color: '#bbb', 'margin-bottom': '16px', 'font-family': 'monospace' }}>
        {(['and', 'or', 'not', '->', '<->'] as const).map((op, i) => (
          <><span style={{ background: '#f5f5f5', padding: '1px 5px', 'border-radius': '3px', color: '#888' }}>{op}</span>{i < 4 ? '  ' : ''}</>
        ))}
      </div>

      {/* Method selector */}
      <div style={{ 'margin-bottom': '28px' }}>
        <div style={{
          display: 'inline-flex',
          border: '1px solid #ddd',
          'border-radius': '6px',
          overflow: 'hidden',
        }}>
          {segBtn('tableau', 'Tableau')}
          {segBtn('sequent', 'Sequent calculus')}
        </div>
      </div>

      {/* Result */}
      <Show when={error()}>
        <pre style={{ color: '#c00', background: '#fff0f0', padding: '12px', 'border-radius': '6px', 'font-size': '0.85rem', 'margin-bottom': '20px' }}>
          {error()}
        </pre>
      </Show>

      <Show when={result()} keyed>
        {outcome => (
          <div style={{ 'margin-bottom': '24px' }}>
            {'Proved' in outcome ? (
              <>
                <div style={{ 'font-size': '1.1rem', color: '#2e7d32', 'margin-bottom': '16px' }}>✓ Tautology</div>
                {method() === 'sequent'
                  ? <SequentDerivation node={outcome.Proved.tree} />
                  : <ProofTree node={outcome.Proved.tree} />}

              </>
            ) : (
              <div>
                <div style={{ 'font-size': '1.1rem', color: '#c62828', 'margin-bottom': '8px' }}>✗ Not a tautology</div>
                <Show when={Object.keys(outcome.Refuted.model).length > 0}>
                  <div style={{ 'font-family': 'monospace', color: '#666', 'font-size': '0.92rem' }}>
                    Counterexample: {formatCounterexample(outcome.Refuted.model)}
                  </div>
                </Show>
              </div>
            )}
          </div>
        )}
      </Show>

      {/* Rules reference */}
      <div style={{ 'border-top': '1px solid #eee', 'padding-top': '20px', 'margin-top': '8px' }}>
        <div style={{ 'font-size': '0.72rem', color: '#bbb', 'margin-bottom': '16px', 'letter-spacing': '0.05em', 'text-transform': 'uppercase' }}>
          Inference rules
        </div>
        <Show when={method() === 'tableau'} fallback={<SequentRules />}>
          <TableauRules />
        </Show>
      </div>
    </div>
  )
}
