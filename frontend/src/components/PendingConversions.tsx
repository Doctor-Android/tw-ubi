import { useState, useEffect } from 'react'
import './Dashboard.css'

interface PendingConversionsProps {
  wallet: string
}

const API_URL = 'http://localhost:8080'

interface PendingConversion {
  id: number
  amount_ue: string
  amount_bu: string
  unlock_epoch: number
  status: string
}

export default function PendingConversions({ wallet }: PendingConversionsProps) {
  const [conversions, setConversions] = useState<PendingConversion[]>([])
  const [loading, setLoading] = useState(false)
  const [message, setMessage] = useState<string>('')

  useEffect(() => {
    loadConversions()
  }, [wallet])

  const loadConversions = async () => {
    setLoading(true)
    try {
      const response = await fetch(`${API_URL}/api/conversions/pending`, {
        headers: { 'X-Wallet-Address': wallet },
      })
      if (response.ok) {
        const data = await response.json()
        setConversions(data)
      }
    } catch (error) {
      console.error('Failed to load conversions:', error)
    }
    setLoading(false)
  }

  const claimConversion = async (conversionId: number) => {
    setLoading(true)
    setMessage('')
    try {
      const response = await fetch(`${API_URL}/api/conversion/claim/${conversionId}`, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
          'X-Wallet-Address': wallet,
        },
      })
      const data = await response.json()
      if (response.ok) {
        setMessage(`BU claimed: ${formatWad(data.amount_bu)} BU`)
        loadConversions()
      } else {
        setMessage(`Error: ${data.error}`)
      }
    } catch (error) {
      setMessage(`Error: ${error}`)
    }
    setLoading(false)
  }

  const formatWad = (wad: string): string => {
    const num = BigInt(wad)
    const divisor = BigInt('1000000000000000000')
    const whole = num / divisor
    const fraction = num % divisor
    if (fraction === BigInt(0)) {
      return whole.toString()
    }
    return `${whole}.${fraction.toString().padStart(18, '0').replace(/0+$/, '')}`
  }

  if (conversions.length === 0) {
    return null
  }

  return (
    <div className="pending-conversions">
      <h3>Pending Conversions</h3>
      <div className="conversions-list">
        {conversions.map((conv) => (
          <div key={conv.id} className="conversion-item">
            <div>
              <p><strong>Amount:</strong> {formatWad(conv.amount_ue)} UE → {formatWad(conv.amount_bu)} BU</p>
              <p><strong>Unlock Epoch:</strong> {conv.unlock_epoch}</p>
              <p><strong>Status:</strong> {conv.status}</p>
            </div>
            {conv.status === 'unlocked' && (
              <button onClick={() => claimConversion(conv.id)} disabled={loading}>
                Claim BU
              </button>
            )}
          </div>
        ))}
      </div>
      {message && (
        <div className={`message ${message.startsWith('Error') ? 'error' : 'success'}`}>
          {message}
        </div>
      )}
    </div>
  )
}

