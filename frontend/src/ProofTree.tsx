import { createEffect } from 'solid-js'
import * as d3 from 'd3'

export type ProofNodeData = {
  label: string
  closed: boolean
  children: ProofNodeData[]
  rule: string | null
}

const NODE_W = 230
const NODE_H = 36
const V_SEP = 80

function truncate(s: string, max: number) {
  return s.length > max ? s.slice(0, max - 1) + '…' : s
}

function drawTree(svgEl: SVGSVGElement, data: ProofNodeData) {
  const svg = d3.select(svgEl)
  const g = svg.select<SVGGElement>('g.tree-root')
  g.selectAll('*').remove()

  const root = d3.hierarchy(data, d => d.children.length ? d.children : null)
  const layout = d3.tree<ProofNodeData>().nodeSize([NODE_W + 16, NODE_H + V_SEP])
  layout(root as any)

  const nodes = root.descendants() as any[]
  const links = root.links() as any[]

  // Links
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

  // Rule labels on links
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

  // Node groups
  const node = g.selectAll('g.node')
    .data(nodes)
    .join('g')
    .attr('class', 'node')
    .attr('transform', (d: any) => `translate(${d.x},${d.y})`)

  // Background rect
  node.append('rect')
    .attr('x', -NODE_W / 2).attr('y', -NODE_H / 2)
    .attr('width', NODE_W).attr('height', NODE_H)
    .attr('rx', 4)
    .attr('fill', (d: any) => d.data.closed ? '#f0faf0' : '#faf0f0')
    .attr('stroke', (d: any) => d.data.closed ? '#4caf50' : '#ef5350')
    .attr('stroke-width', 1.5)

  // Status icon
  node.append('text')
    .attr('x', -NODE_W / 2 + 10)
    .attr('dy', '0.35em')
    .attr('font-size', 12)
    .attr('fill', (d: any) => d.data.closed ? '#4caf50' : '#ef5350')
    .text((d: any) => d.data.closed ? '✓' : '✗')

  // Label text
  node.append('text')
    .attr('x', -NODE_W / 2 + 24)
    .attr('dy', '0.35em')
    .attr('font-size', 11)
    .attr('font-family', 'monospace')
    .attr('fill', '#333')
    .text((d: any) => truncate(d.data.label, 26))
    .append('title')
    .text((d: any) => d.data.label)

  // Zoom setup
  const w = svgEl.clientWidth || 800
  const zoom = d3.zoom<SVGSVGElement, unknown>()
    .scaleExtent([0.1, 4])
    .on('zoom', e => g.attr('transform', e.transform.toString()))
  svg.on('.zoom', null)  // remove previous listeners
  svg.call(zoom)
  svg.call(zoom.transform, d3.zoomIdentity.translate(w / 2, NODE_H + 16))
}

export function ProofTree(props: { node: ProofNodeData }) {
  let svgEl!: SVGSVGElement

  createEffect(() => {
    drawTree(svgEl, props.node)
  })

  return (
    <svg
      ref={svgEl!}
      style={{ width: '100%', height: '500px', display: 'block', border: '1px solid #eee', 'border-radius': '6px' }}
    >
      <g class="tree-root" />
    </svg>
  )
}
