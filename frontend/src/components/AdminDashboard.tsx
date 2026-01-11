import { useState } from 'react'
import './Dashboard.css'

interface AdminDashboardProps {
  wallet: string
}

const API_URL = 'http://localhost:8080'

export default function AdminDashboard({ wallet }: AdminDashboardProps) {
  const [loading, setLoading] = useState(false)
  const [state, setState] = useState<any>(null)
  const [message, setMessage] = useState<string>('')

  const exportState = async () => {
    setLoading(true)
    setMessage('')
    try {
      const response = await fetch(`${API_URL}/api/admin/export-state`)
      const data = await response.json()
      if (response.ok) {
        setState(data)
        setMessage('State exported successfully')
      } else {
        setMessage(`Error: ${data.error}`)
      }
    } catch (error) {
      setMessage(`Error: ${error}`)
    }
    setLoading(false)
  }

  const downloadState = () => {
    if (!state) return
    const blob = new Blob([JSON.stringify(state, null, 2)], { type: 'application/json' })
    const url = URL.createObjectURL(blob)
    const a = document.createElement('a')
    a.href = url
    a.download = `ubi-state-${Date.now()}.json`
    a.click()
    URL.revokeObjectURL(url)
  }

  return (
    <div className="dashboard">
      <h2>Admin Dashboard</h2>
      <div className="wallet-info">
        <p><strong>Wallet:</strong> {wallet}</p>
      </div>

      <div className="actions">
        <div className="action-card">
          <h3>Export State</h3>
          <p>Export all system state for forkability</p>
          <button onClick={exportState} disabled={loading}>
            {loading ? 'Exporting...' : 'Export State'}
          </button>
          {state && (
            <button onClick={downloadState} className="secondary">
              Download JSON
            </button>
          )}
        </div>
      </div>

      {state && (
        <div className="state-preview">
          <h3>State Preview</h3>
          <div className="stats">
            <p><strong>Users:</strong> {state.users?.length || 0}</p>
            <p><strong>Claims:</strong> {state.claims?.length || 0}</p>
            <p><strong>Conversions:</strong> {state.conversions?.length || 0}</p>
            <p><strong>Events:</strong> {state.events?.length || 0}</p>
          </div>
          <details>
            <summary>Full State (JSON)</summary>
            <pre>{JSON.stringify(state, null, 2)}</pre>
          </details>
        </div>
      )}

      {message && (
        <div className={`message ${message.startsWith('Error') ? 'error' : 'success'}`}>
          {message}
        </div>
      )}
    </div>
  )
}

