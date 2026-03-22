import { createEffect, Show } from 'solid-js'
import * as d3 from 'd3'

export type ProofNodeData = {
  label: string
  closed: boolean
  children: ProofNodeData[]
  rule: string | null
}

// ── Tableau: D3 node-link tree (top-down) ────────────────────────────────────

const NODE_H = 36
const V_SEP = 80
const NODE_GAP = 16
const CHAR_W = 6.6   // approx monospace 11px char width
const NODE_PAD = 34  // left icon (24) + right margin (10)
const MIN_NODE_W = 100
const MAX_NODE_W = 480

function nodeWidth(label: string) {
  return Math.min(MAX_NODE_W, Math.max(MIN_NODE_W, Math.ceil(label.length * CHAR_W) + NODE_PAD))
}

function drawTree(svgEl: SVGSVGElement, data: ProofNodeData) {
  const svg = d3.select(svgEl)
  const g = svg.select<SVGGElement>('g.tree-root')
  g.selectAll('*').remove()

  const root = d3.hierarchy(data, d => d.children.length ? d.children : null)
  const maxW = Math.max(...root.descendants().map((d: any) => nodeWidth(d.data.label)))
  const layout = d3.tree<ProofNodeData>().nodeSize([maxW + NODE_GAP, NODE_H + V_SEP])
  layout(root as any)

  const nodes = root.descendants() as any[]
  const links = root.links() as any[]

  g.selectAll('path')
    .data(links)
    .join('path')
    .attr('fill', 'none')
    .attr('stroke', '#ccc')
    .attr('stroke-width', 1.5)
    .attr('d', (l: any) => {
      const sx = l.source.x, sy = l.source.y
      const tx = l.target.x, ty = l.target.y
      const my = (sy + ty) / 2
      return `M${sx},${sy} C${sx},${my} ${tx},${my} ${tx},${ty}`
    })

  g.selectAll('text.rule')
    .data(links.filter((l: any) => !!l.target.data.rule))
    .join('text')
    .attr('class', 'rule')
    .attr('x', (l: any) => (l.source.x + l.target.x) / 2 + 8)
    .attr('y', (l: any) => (l.source.y + l.target.y) / 2)
    .attr('font-size', 11)
    .attr('fill', '#999')
    .attr('font-family', 'sans-serif')
    .text((l: any) => `[${l.target.data.rule}]`)

  const node = g.selectAll('g.node')
    .data(nodes)
    .join('g')
    .attr('class', 'node')
    .attr('transform', (d: any) => `translate(${d.x},${d.y})`)

  node.append('rect')
    .attr('x', (d: any) => -nodeWidth(d.data.label) / 2)
    .attr('y', -NODE_H / 2)
    .attr('width', (d: any) => nodeWidth(d.data.label))
    .attr('height', NODE_H)
    .attr('rx', 4)
    .attr('fill', (d: any) => d.data.closed ? '#f0faf0' : '#faf0f0')
    .attr('stroke', (d: any) => d.data.closed ? '#4caf50' : '#ef5350')
    .attr('stroke-width', 1.5)

  node.append('text')
    .attr('x', (d: any) => -nodeWidth(d.data.label) / 2 + 10)
    .attr('dy', '0.35em')
    .attr('font-size', 12)
    .attr('fill', (d: any) => d.data.closed ? '#4caf50' : '#ef5350')
    .text((d: any) => d.data.closed ? '✓' : '✗')

  node.append('text')
    .attr('x', (d: any) => -nodeWidth(d.data.label) / 2 + 24)
    .attr('dy', '0.35em')
    .attr('font-size', 11)
    .attr('font-family', 'monospace')
    .attr('fill', '#333')
    .text((d: any) => d.data.label)
    .append('title')
    .text((d: any) => d.data.label)

  const w = svgEl.clientWidth || 800
  const zoom = d3.zoom<SVGSVGElement, unknown>()
    .scaleExtent([0.1, 4])
    .on('zoom', e => g.attr('transform', e.transform.toString()))
  svg.on('.zoom', null)
  svg.call(zoom)
  svg.call(zoom.transform, d3.zoomIdentity.translate(w / 2, NODE_H + 16))
}

export function ProofTree(props: { node: ProofNodeData }) {
  let svgEl!: SVGSVGElement
  createEffect(() => { drawTree(svgEl, props.node) })
  return (
    <svg
      ref={svgEl!}
      style={{ width: '100%', height: '500px', display: 'block', border: '1px solid #eee', 'border-radius': '6px' }}
    >
      <g class="tree-root" />
    </svg>
  )
}

// ── Sequent: recursive derivation tree (mathematical textbook style) ──────────

const mono: any = {
  'font-family': "'STIX Two Text', 'Cambria', serif",
  'font-size': '0.9rem',
  'white-space': 'nowrap',
}

function DerivNode(props: { node: ProofNodeData }) {
  const hasChildren = props.node.children.length > 0
  const hasRule = props.node.rule !== null

  return (
    <div style={{ display: 'inline-flex', 'flex-direction': 'column', 'align-items': 'center' }}>
      {/* Premises row — align bottoms so derivations of different heights line up */}
      <Show when={hasChildren}>
        <div style={{ display: 'flex', gap: '28px', 'align-items': 'flex-end', 'padding-bottom': '4px' }}>
          {props.node.children.map(child => <DerivNode node={child} />)}
        </div>
      </Show>
      {/* Horizontal bar + rule name */}
      <Show when={hasRule}>
        <div style={{ 'align-self': 'stretch', 'border-top': '1.2px solid #555', 'min-width': '52px', position: 'relative', 'margin-bottom': '4px' }}>
          <span style={{
            position: 'absolute',
            left: '100%',
            top: '-0.65em',
            'margin-left': '6px',
            'font-size': '0.75rem',
            color: '#777',
            'white-space': 'nowrap',
          }}>
            {props.node.rule}
          </span>
        </div>
      </Show>
      {/* Conclusion */}
      <div style={{
        ...mono,
        color: props.node.closed ? '#2e7d32' : '#c62828',
        padding: '1px 2px',
      }}>
        {props.node.label}
      </div>
    </div>
  )
}

export function SequentDerivation(props: { node: ProofNodeData }) {
  return (
    <div style={{
      overflow: 'auto',
      border: '1px solid #eee',
      'border-radius': '6px',
      padding: '24px 28px',
      background: '#fafafa',
    }}>
      <DerivNode node={props.node} />
    </div>
  )
}
