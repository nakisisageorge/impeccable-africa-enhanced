import { useState, useCallback } from 'react'
import {
  ReactFlow,
  Node,
  Edge,
  Background,
  Controls,
  MiniMap,
  Panel,
  useNodesState,
  useEdgesState,
  addEdge,
  NodeTypes,
  EdgeTypes,
  Handle,
  Position,
} from 'reactflow'
import 'reactflow/dist/style.css'

// Custom node types
const CrateNode = ({ data }: { data: { label: string; description: string; category: string } }) => (
  <div
    style={{
      padding: '12px 16px',
      background: data.category === 'core' ? '#1e293b' : data.category === 'infra' ? '#0c4a6e' : data.category === 'feature' ? '#14532d' : '#7c2d12',
      border: '1px solid',
      borderColor: data.category === 'core' ? '#475569' : data.category === 'infra' ? '#0ea5e9' : data.category === 'feature' ? '#22c55e' : '#ea580c',
      borderRadius: '8px',
      color: 'white',
      minWidth: 180,
      fontSize: '13px',
      fontFamily: 'system-ui, sans-serif',
    }}
  >
    <Handle type="target" position={Position.Top} />
    <div style={{ fontWeight: 600, marginBottom: '4px' }}>{data.label}</div>
    <div style={{ fontSize: '11px', opacity: 0.8 }}>{data.description}</div>
    <Handle type="source" position={Position.Bottom} />
  </div>
)

const CommandNode = ({ data }: { data: { label: string; description: string } }) => (
  <div
    style={{
      padding: '10px 14px',
      background: '#7c3aed',
      border: '1px solid #a855f7',
      borderRadius: '8px',
      color: 'white',
      minWidth: 160,
      fontSize: '12px',
      fontFamily: 'system-ui, sans-serif',
    }}
  >
    <Handle type="target" position={Position.Left} />
    <div style={{ fontWeight: 600, marginBottom: '2px' }}>{data.label}</div>
    <div style={{ fontSize: '10px', opacity: 0.8 }}>{data.description}</div>
    <Handle type="source" position={Position.Right} />
  </div>
)

const ToolNode = ({ data }: { data: { label: string; transport: string } }) => (
  <div
    style={{
      padding: '8px 12px',
      background: '#0ea5e9',
      border: '1px solid #38bdf8',
      borderRadius: '6px',
      color: 'white',
      minWidth: 140,
      fontSize: '11px',
      fontFamily: 'system-ui, sans-serif',
    }}
  >
    <Handle type="target" position={Position.Left} />
    <div style={{ fontWeight: 600, marginBottom: '2px' }}>{data.label}</div>
    <div style={{ fontSize: '9px', opacity: 0.8 }}>{data.transport}</div>
    <Handle type="source" position={Position.Right} />
  </div>
)

const ViewNode = ({ data }: { data: { label: string; nodes: number } }) => (
  <div
    style={{
      padding: '8px 12px',
      background: '#ec4899',
      border: '1px solid #f472b6',
      borderRadius: '6px',
      color: 'white',
      minWidth: 140,
      fontSize: '11px',
      fontFamily: 'system-ui, sans-serif',
    }}
  >
    <Handle type="target" position={Position.Top} />
    <div style={{ fontWeight: 600, marginBottom: '2px' }}>{data.label}</div>
    <div style={{ fontSize: '9px', opacity: 0.8 }}>{data.nodes} nodes</div>
    <Handle type="source" position={Position.Bottom} />
  </div>
)

const nodeTypes: NodeTypes = {
  crate: CrateNode,
  command: CommandNode,
  tool: ToolNode,
  view: ViewNode,
}

const edgeTypes: EdgeTypes = {}

const initialNodes: Node[] = [
  // Core Crates
  { id: '1', type: 'crate', position: { x: 100, y: 100 }, data: { label: 'impeccable-foundation', description: 'Rule registry, color, CSS, JS analysis, 62+ rules', category: 'core' } },
  { id: '2', type: 'crate', position: { x: 100, y: 220 }, data: { label: 'impeccable-common', description: 'Shared types, IO abstractions, error handling', category: 'core' } },
  { id: '3', type: 'crate', position: { x: 100, y: 340 }, data: { label: 'impeccable-core', description: 'Engine orchestration, verb routing, session mgmt', category: 'core' } },
  { id: '4', type: 'crate', position: { x: 100, y: 460 }, data: { label: 'impeccable-detect', description: 'Slop detector, rule evaluation, findings', category: 'core' } },
  { id: '5', type: 'crate', position: { x: 100, y: 580 }, data: { label: 'impeccable-html', description: 'HTML parsing, component extraction, WASM', category: 'core' } },
  { id: '6', type: 'crate', position: { x: 100, y: 700 }, data: { label: 'impeccable-browser', description: 'CDP automation, browser relay, extension host', category: 'core' } },
  { id: '7', type: 'crate', position: { x: 100, y: 820 }, data: { label: 'impeccable-live', description: 'Live mode, variant overlays, visual critique', category: 'core' } },
  
  // Infrastructure Crates
  { id: '8', type: 'crate', position: { x: 500, y: 100 }, data: { label: 'impeccable-mcp', description: 'MCP server (stdio, HTTP, SSE, Streamable, WS)', category: 'infra' } },
  { id: '9', type: 'crate', position: { x: 500, y: 220 }, data: { label: 'impeccable-benchmark', description: 'Benchmark runner, regression detection (5%), SQLite', category: 'infra' } },
  { id: '10', type: 'crate', position: { x: 500, y: 340 }, data: { label: 'impeccable-account', description: 'Auth, sign-in, session management, API client', category: 'infra' } },
  { id: '11', type: 'crate', position: { x: 500, y: 460 }, data: { label: 'impeccable-hook', description: 'Git hooks, pre-commit, CI integration', category: 'infra' } },
  { id: '12', type: 'crate', position: { x: 500, y: 580 }, data: { label: 'impeccable-bundle', description: 'WASM bundling, browser-bundle, xtask', category: 'infra' } },
  { id: '13', type: 'crate', position: { x: 500, y: 700 }, data: { label: 'impeccable-agents', description: 'Agent skill definitions, 30+ integrations', category: 'infra' } },
  { id: '14', type: 'crate', position: { x: 500, y: 820 }, data: { label: 'impeccable-cli', description: 'CLI entrypoint, verb dispatch, install logic', category: 'infra' } },
  
  // Feature Crates
  { id: '15', type: 'crate', position: { x: 900, y: 100 }, data: { label: 'impeccable-comp', description: 'Component review, composition fidelity', category: 'feature' } },
  { id: '16', type: 'crate', position: { x: 900, y: 220 }, data: { label: 'impeccable-comp-verbs', description: 'Polish, bolder, quieter, clarify, adapt verbs', category: 'feature' } },
  { id: '17', type: 'crate', position: { x: 900, y: 340 }, data: { label: 'impeccable-context', description: 'Context gathering, file analysis, git integration', category: 'feature' } },
  { id: '18', type: 'crate', position: { x: 900, y: 460 }, data: { label: 'impeccable-skills', description: 'Skill definitions, African design vocabulary', category: 'feature' } },
  { id: '19', type: 'crate', position: { x: 900, y: 580 }, data: { label: 'impeccable-foundations-scan', description: 'Design system foundations, token extraction', category: 'feature' } },
  
  // Design Commands
  { id: '20', type: 'command', position: { x: 1300, y: 100 }, data: { label: 'critique', description: 'Design critique with rubric' } },
  { id: '21', type: 'command', position: { x: 1300, y: 180 }, data: { label: 'audit', description: 'Technical quality audit (10 dims)' } },
  { id: '22', type: 'command', position: { x: 1300, y: 260 }, data: { label: 'polish', description: 'Fix slop, apply design rules' } },
  { id: '23', type: 'command', position: { x: 1300, y: 340 }, data: { label: 'bolder', description: 'Strengthen visual hierarchy' } },
  { id: '24', type: 'command', position: { x: 1300, y: 420 }, data: { label: 'clarify', description: 'Rewrite UX copy (7 African langs)' } },
  { id: '25', type: 'command', position: { x: 1300, y: 500 }, data: { label: 'adapt', description: 'Adapt to platform conventions' } },
  { id: '26', type: 'command', position: { x: 1300, y: 580 }, data: { label: 'optimize', description: 'Performance optimization' } },
  { id: '27', type: 'command', position: { x: 1300, y: 660 }, data: { label: 'harden', description: 'Security hardening' } },
  { id: '28', type: 'command', position: { x: 1300, y: 740 }, data: { label: 'visualize', description: 'Generate architecture viz' } },
  { id: '29', type: 'command', position: { x: 1300, y: 820 }, data: { label: 'new-work', description: 'Greenfield project scaffolding' } },
  
  // MCP Tools
  { id: '30', type: 'tool', position: { x: 1600, y: 100 }, data: { label: 'design_critique', transport: 'HTTP/SSE/WS' } },
  { id: '31', type: 'tool', position: { x: 1600, y: 160 }, data: { label: 'design_audit', transport: 'HTTP/SSE/WS' } },
  { id: '32', type: 'tool', position: { x: 1600, y: 220 }, data: { label: 'design_polish', transport: 'HTTP/SSE/WS' } },
  { id: '33', type: 'tool', position: { x: 1600, y: 280 }, data: { label: 'design_bolder', transport: 'HTTP/SSE/WS' } },
  { id: '34', type: 'tool', position: { x: 1600, y: 340 }, data: { label: 'design_clarify', transport: 'HTTP/SSE/WS' } },
  { id: '35', type: 'tool', position: { x: 1600, y: 400 }, data: { label: 'design_live_start', transport: 'HTTP/SSE/WS' } },
  { id: '36', type: 'tool', position: { x: 1600, y: 460 }, data: { label: 'design_benchmark_run', transport: 'HTTP/SSE/WS' } },
  { id: '37', type: 'tool', position: { x: 1600, y: 520 }, data: { label: 'design_viz_generate', transport: 'HTTP/SSE/WS' } },
  
  // Architecture Views
  { id: '38', type: 'view', position: { x: 1900, y: 100 }, data: { label: 'Overview', nodes: 82 } },
  { id: '39', type: 'view', position: { x: 1900, y: 180 }, data: { label: 'Crates', nodes: 23 } },
  { id: '40', type: 'view', position: { x: 1900, y: 260 }, data: { label: 'Commands', nodes: 24 } },
  { id: '41', type: 'view', position: { x: 1900, y: 340 }, data: { label: 'Detectors', nodes: 82 } },
  { id: '42', type: 'view', position: { x: 1900, y: 420 }, data: { label: 'MCP', nodes: 15 } },
  { id: '43', type: 'view', position: { x: 1900, y: 500 }, data: { label: 'Skills', nodes: 30 } },
  { id: '44', type: 'view', position: { x: 1900, y: 580 }, data: { label: 'CI/CD', nodes: 12 } },
  { id: '45', type: 'view', position: { x: 1900, y: 660 }, data: { label: 'Data Flow', nodes: 18 } },
]

const initialEdges: Edge[] = [
  // Core crate dependencies
  { id: 'e1-3', source: '1', target: '3', type: 'default' },
  { id: 'e2-3', source: '2', target: '3', type: 'default' },
  { id: 'e1-4', source: '1', target: '4', type: 'default' },
  { id: 'e5-4', source: '5', target: '4', type: 'default' },
  { id: 'e4-6', source: '4', target: '6', type: 'default' },
  { id: 'e4-7', source: '4', target: '7', type: 'default' },
  { id: 'e6-7', source: '6', target: '7', type: 'default' },
  
  // Infrastructure dependencies
  { id: 'e2-8', source: '2', target: '8', type: 'default' },
  { id: 'e10-8', source: '10', target: '8', type: 'default' },
  { id: 'e2-9', source: '2', target: '9', type: 'default' },
  { id: 'e1-9', source: '1', target: '9', type: 'default' },
  { id: 'e2-10', source: '2', target: '10', type: 'default' },
  { id: 'e2-11', source: '2', target: '11', type: 'default' },
  { id: 'e5-12', source: '5', target: '12', type: 'default' },
  { id: 'e13-3', source: '13', target: '3', type: 'default' },
  { id: 'e3-14', source: '3', target: '14', type: 'default' },
  
  // Feature dependencies
  { id: 'e1-15', source: '1', target: '15', type: 'default' },
  { id: 'e4-15', source: '4', target: '15', type: 'default' },
  { id: 'e1-16', source: '1', target: '16', type: 'default' },
  { id: 'e24-16', source: '24', target: '16', type: 'default' },
  { id: 'e2-17', source: '2', target: '17', type: 'default' },
  { id: 'e1-18', source: '1', target: '18', type: 'default' },
  { id: 'e1-19', source: '1', target: '19', type: 'default' },
  
  // Commands to crates
  { id: 'e3-20', source: '3', target: '20', type: 'default' },
  { id: 'e3-21', source: '3', target: '21', type: 'default' },
  { id: 'e3-22', source: '3', target: '22', type: 'default' },
  { id: 'e3-23', source: '3', target: '23', type: 'default' },
  { id: 'e3-24', source: '3', target: '24', type: 'default' },
  { id: 'e3-25', source: '3', target: '25', type: 'default' },
  { id: 'e3-26', source: '3', target: '26', type: 'default' },
  { id: 'e3-27', source: '3', target: '27', type: 'default' },
  { id: 'e3-28', source: '3', target: '28', type: 'default' },
  { id: 'e3-29', source: '3', target: '29', type: 'default' },
  
  // Commands to MCP tools
  { id: 'e20-30', source: '20', target: '30', type: 'default' },
  { id: 'e21-31', source: '21', target: '31', type: 'default' },
  { id: 'e22-32', source: '22', target: '32', type: 'default' },
  { id: 'e23-33', source: '23', target: '33', type: 'default' },
  { id: 'e24-34', source: '24', target: '34', type: 'default' },
  { id: 'e7-35', source: '7', target: '35', type: 'default' },
  { id: 'e9-36', source: '9', target: '36', type: 'default' },
  { id: 'e28-37', source: '28', target: '37', type: 'default' },
  
  // MCP tools to views
  { id: 'e30-38', source: '30', target: '38', type: 'default' },
  { id: 'e31-38', source: '31', target: '38', type: 'default' },
  { id: 'e32-38', source: '32', target: '38', type: 'default' },
  { id: 'e33-38', source: '33', target: '38', type: 'default' },
  { id: 'e34-38', source: '34', target: '38', type: 'default' },
  { id: 'e35-38', source: '35', target: '38', type: 'default' },
  { id: 'e36-38', source: '36', target: '38', type: 'default' },
  { id: 'e37-38', source: '37', target: '38', type: 'default' },
  
  // Views to individual views
  { id: 'e38-39', source: '38', target: '39', type: 'default' },
  { id: 'e38-40', source: '38', target: '40', type: 'default' },
  { id: 'e38-41', source: '38', target: '41', type: 'default' },
  { id: 'e38-42', source: '38', target: '42', type: 'default' },
  { id: 'e38-43', source: '38', target: '43', type: 'default' },
  { id: 'e38-44', source: '38', target: '44', type: 'default' },
  { id: 'e38-45', source: '38', target: '45', type: 'default' },
]

// View configurations
const views = [
  { id: 'overview', label: 'Overview', nodes: initialNodes, edges: initialEdges },
  { id: 'crates', label: 'Crates', nodes: initialNodes.filter(n => n.type === 'crate'), edges: initialEdges.filter(e => initialNodes.find(n => n.id === e.source)?.type === 'crate' && initialNodes.find(n => n.id === e.target)?.type === 'crate') },
  { id: 'commands', label: 'Commands', nodes: [...initialNodes.filter(n => n.type === 'crate'), ...initialNodes.filter(n => n.type === 'command')], edges: initialEdges.filter(e => ['command', 'crate'].includes(initialNodes.find(n => n.id === e.source)?.type || '') && ['command', 'crate'].includes(initialNodes.find(n => n.id === e.target)?.type || '')) },
  { id: 'detectors', label: 'Detectors', nodes: initialNodes.filter(n => n.id === '1' || n.id === '4' || n.id === '5' || n.type === 'command'), edges: initialEdges.filter(e => ['1', '4', '5'].includes(e.source) || ['1', '4', '5'].includes(e.target) || (initialNodes.find(n => n.id === e.source)?.type === 'command' && initialNodes.find(n => n.id === e.target)?.type === 'command')) },
  { id: 'mcp', label: 'MCP', nodes: [...initialNodes.filter(n => n.id === '8' || n.id === '10'), ...initialNodes.filter(n => n.type === 'tool')], edges: initialEdges.filter(e => ['8', '10'].includes(e.source) || initialNodes.find(n => n.id === e.target)?.type === 'tool') },
  { id: 'skills', label: 'Skills', nodes: [...initialNodes.filter(n => n.id === '13' || n.id === '18'), ...initialNodes.filter(n => n.type === 'command')], edges: initialEdges.filter(e => ['13', '18'].includes(e.source) || initialNodes.find(n => n.id === e.target)?.type === 'command') },
  { id: 'ci', label: 'CI/CD', nodes: [...initialNodes.filter(n => n.id === '9' || n.id === '11' || n.id === '12'), ...initialNodes.filter(n => n.type === 'tool')], edges: initialEdges.filter(e => ['9', '11', '12'].includes(e.source) || initialNodes.find(n => n.id === e.target)?.type === 'tool') },
  { id: 'data_flow', label: 'Data Flow', nodes: initialNodes, edges: initialEdges },
]

export default function App() {
  const [nodes, setNodes, onNodesChange] = useNodesState(initialNodes)
  const [edges, setEdges, onEdgesChange] = useEdgesState(initialEdges)
  const [activeView, setActiveView] = useState('overview')
  const [fitView, setFitView] = useState(false)

  const onConnect = useCallback(
    (params: any) => setEdges((eds) => addEdge(params, eds)),
    [setEdges]
  )

  const handleViewChange = (viewId: string) => {
    setActiveView(viewId)
    const view = views.find(v => v.id === viewId)
    if (view) {
      setNodes(view.nodes)
      setEdges(view.edges)
      setFitView(true)
    }
  }

  const currentView = views.find(v => v.id === activeView) || views[0]

  return (
    <div style={{ width: '100vw', height: '100vh', display: 'flex', flexDirection: 'column' }}>
      {/* Header */}
      <header style={{
        padding: '16px 24px',
        background: 'white',
        borderBottom: '1px solid #e2e8f0',
        display: 'flex',
        alignItems: 'center',
        justifyContent: 'space-between',
        flexWrap: 'wrap',
        gap: '12px',
      }}>
        <div style={{ display: 'flex', alignItems: 'center', gap: '12px' }}>
          <h1 style={{ margin: 0, fontSize: '20px', fontWeight: 700, color: '#1e293b' }}>
            Impeccable Africa Enhanced - Architecture
          </h1>
          <span style={{ fontSize: '12px', color: '#64748b' }}>82+ Rules · 24 Commands · 8 Views · 4 Transports</span>
        </div>
        <div style={{ display: 'flex', gap: '8px', flexWrap: 'wrap' }}>
          {views.map(view => (
            <button
              key={view.id}
              onClick={() => handleViewChange(view.id)}
              style={{
                padding: '8px 16px',
                background: activeView === view.id ? '#7c3aed' : '#f1f5f9',
                color: activeView === view.id ? 'white' : '#475569',
                border: 'none',
                borderRadius: '6px',
                fontSize: '13px',
                fontWeight: 500,
                cursor: 'pointer',
                transition: 'all 0.2s',
              }}
            >
              {view.label}
            </button>
          ))}
        </div>
      </header>

      {/* React Flow Canvas */}
      <div style={{ flex: 1, position: 'relative' }}>
        <ReactFlow
          nodes={nodes}
          edges={edges}
          onNodesChange={onNodesChange}
          onEdgesChange={onEdgesChange}
          onConnect={onConnect}
          nodeTypes={nodeTypes}
          edgeTypes={edgeTypes}
          fitView={fitView}
          attributionPosition="bottom-left"
        >
          <Background color="#e2e8f0" gap={16} />
          <Controls />
          <MiniMap
            nodeColor={({ type }) => type === 'crate' ? '#1e293b' : type === 'command' ? '#7c3aed' : type === 'tool' ? '#0ea5e9' : '#ec4899'}
            maskColor="rgba(255,255,255,0.8)"
          />
          <Panel position="bottom-right">
            <div style={{
              padding: '16px',
              background: 'white',
              borderRadius: '8px',
              boxShadow: '0 4px 12px rgba(0,0,0,0.1)',
              minWidth: 280,
              maxWidth: 320,
            }}>
              <h3 style={{ margin: '0 0 12px', fontSize: '14px', fontWeight: 600, color: '#1e293b' }}>
                {currentView.label} View
              </h3>
              <p style={{ margin: '0 0 12px', fontSize: '12px', color: '#64748b', lineHeight: 1.5 }}>
                {getViewDescription(currentView.id)}
              </p>
              <div style={{ display: 'flex', flexWrap: 'wrap', gap: '6px' }}>
                {getLegendForView(currentView.id).map((item, i) => (
                  <span key={i} style={{
                    display: 'inline-flex',
                    alignItems: 'center',
                    gap: '6px',
                    padding: '4px 8px',
                    background: '#f1f5f9',
                    borderRadius: '4px',
                    fontSize: '11px',
                    color: '#475569',
                  }}>
                    <span style={{
                      width: '10px',
                      height: '10px',
                      borderRadius: '2px',
                      background: item.color,
                    }} />
                    {item.label}
                  </span>
                ))}
              </div>
            </div>
          </Panel>
        </ReactFlow>
      </div>

      {/* Footer */}
      <footer style={{
        padding: '12px 24px',
        background: 'white',
        borderTop: '1px solid #e2e8f0',
        display: 'flex',
        alignItems: 'center',
        justifyContent: 'space-between',
        flexWrap: 'wrap',
        gap: '8px',
        fontSize: '12px',
        color: '#64748b',
      }}>
        <div>Interactive architecture visualization · <a href="https://github.com/nakisisageorge/impeccable-africa-enhanced" target="_blank" rel="noopener noreferrer" style={{ color: '#7c3aed' }}>GitHub Repository</a></div>
        <div>George Nakisisa · GICT Africa Technologies · <a href="https://gictafrica.com" target="_blank" rel="noopener noreferrer" style={{ color: '#7c3aed' }}>gictafrica.com</a></div>
      </footer>
    </div>
  )
}

function getViewDescription(viewId: string): string {
  const descriptions: Record<string, string> = {
    overview: 'Complete system architecture showing all 23 crates, 24 design commands, 8 MCP tools, and 8 interactive views.',
    crates: 'Rust workspace with 23 crates organized into Core (7), Infrastructure (7), and Feature (5) categories.',
    commands: '24 design commands from critique to new-work, each routing through the core engine to specific crates.',
    detectors: '82+ detector rules (62 original + 20 African patterns) across slop, quality, and African categories.',
    mcp: 'MCP server with 4 transports (stdio, HTTP, SSE, Streamable, WebSocket) and 8 design orchestration tools.',
    skills: '30+ agent skill integrations with African design vocabulary, language support, and accessibility rules.',
    ci: 'Enhanced CI/CD with cargo-audit, cargo-deny, TruffleHog, SBOM, biome, mutation testing, multi-platform.',
    data_flow: 'End-to-end data flow from CLI input through detector, engine, MCP, benchmark, to visualization output.',
  }
  return descriptions[viewId] || ''
}

function getLegendForView(viewId: string) {
  const legends: Record<string, Array<{ label: string; color: string }>> = {
    overview: [
      { label: 'Core Crates', color: '#1e293b' },
      { label: 'Infra Crates', color: '#0c4a6e' },
      { label: 'Feature Crates', color: '#14532d' },
      { label: 'Commands', color: '#7c3aed' },
      { label: 'MCP Tools', color: '#0ea5e9' },
      { label: 'Views', color: '#ec4899' },
    ],
    crates: [
      { label: 'Core Crates', color: '#1e293b' },
      { label: 'Infra Crates', color: '#0c4a6e' },
      { label: 'Feature Crates', color: '#14532d' },
    ],
    commands: [
      { label: 'Core Crates', color: '#1e293b' },
      { label: 'Commands', color: '#7c3aed' },
    ],
    detectors: [
      { label: 'Foundation', color: '#1e293b' },
      { label: 'Detector', color: '#1e293b' },
      { label: 'Commands', color: '#7c3aed' },
    ],
    mcp: [
      { label: 'MCP Crate', color: '#0c4a6e' },
      { label: 'Account', color: '#0c4a6e' },
      { label: 'MCP Tools', color: '#0ea5e9' },
    ],
    skills: [
      { label: 'Agents', color: '#0c4a6e' },
      { label: 'Skills', color: '#14532d' },
      { label: 'Commands', color: '#7c3aed' },
    ],
    ci: [
      { label: 'Benchmark', color: '#0c4a6e' },
      { label: 'Hook', color: '#0c4a6e' },
      { label: 'Bundle', color: '#0c4a6e' },
      { label: 'MCP Tools', color: '#0ea5e9' },
    ],
    data_flow: [
      { label: 'Core Crates', color: '#1e293b' },
      { label: 'Infra Crates', color: '#0c4a6e' },
      { label: 'Feature Crates', color: '#14532d' },
      { label: 'Commands', color: '#7c3aed' },
      { label: 'MCP Tools', color: '#0ea5e9' },
      { label: 'Views', color: '#ec4899' },
    ],
  }
  return legends[viewId] || legends.overview
}