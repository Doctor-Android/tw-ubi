import { useState, useEffect } from 'react'
import './Dashboard.css'
import PendingConversions from './PendingConversions'

interface UserDashboardProps {
  wallet: string
}

const API_URL = 'http://localhost:8080'

export default function UserDashboard({ wallet }: UserDashboardProps) {
  const [ueBalance, setUeBalance] = useState<string>('0')
  const [buBalance, setBuBalance] = useState<string>('0')
  const [rateIndex, setRateIndex] = useState<string>('0')
  const [conversionAmount, setConversionAmount] = useState<string>('')
  const [minBuOut, setMinBuOut] = useState<string>('')
  const [loading, setLoading] = useState(false)
  const [message, setMessage] = useState<string>('')

  useEffect(() => {
    loadBalances()
  }, [wallet])

  const loadBalances = async () => {
    try {
      // Fetch UE balance
      const ueResponse = await fetch(`${API_URL}/api/balances/ue`, {
        headers: { 'X-Wallet-Address': wallet },
      })
      if (ueResponse.ok) {
        const ueData = await ueResponse.json()
        setUeBalance(ueData.balance || '0')
      }
      
      // Fetch BU balance
      const buResponse = await fetch(`${API_URL}/api/balances/bu`, {
        headers: { 'X-Wallet-Address': wallet },
      })
      if (buResponse.ok) {
        const buData = await buResponse.json()
        setBuBalance(buData.balance || '0')
      }
      
      // Fetch rate index (assuming region 1 for now)
      const rateResponse = await fetch(`${API_URL}/api/balances/rate-index/1`)
      if (rateResponse.ok) {
        const rateData = await rateResponse.json()
        setRateIndex(rateData.rate_index || '0')
      }
    } catch (error) {
      console.error('Failed to load balances:', error)
    }
  }

  const claimUBI = async () => {
    setLoading(true)
    setMessage('')
    try {
      const response = await fetch(`${API_URL}/api/ubi/claim`, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
          'X-Wallet-Address': wallet,
        },
      })
      const data = await response.json()
      if (response.ok) {
        setMessage(`UBI claimed: ${formatWad(data.amount_ue)} UE`)
        loadBalances()
      } else {
        setMessage(`Error: ${data.error}`)
      }
    } catch (error) {
      setMessage(`Error: ${error}`)
    }
    setLoading(false)
  }

  const requestConversion = async () => {
    if (!conversionAmount || !minBuOut) {
      setMessage('Please enter amount and min BU out')
      return
    }
    setLoading(true)
    setMessage('')
    try {
      const response = await fetch(`${API_URL}/api/conversion/request`, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
          'X-Wallet-Address': wallet,
        },
        body: JSON.stringify({
          amount_ue: toWad(conversionAmount),
          min_bu_out: toWad(minBuOut),
        }),
      })
      const data = await response.json()
      if (response.ok) {
        setMessage(`Conversion requested: ${formatWad(data.amount_bu)} BU (unlocks epoch ${data.unlock_epoch})`)
        setConversionAmount('')
        setMinBuOut('')
        loadBalances()
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

  const toWad = (amount: string): string => {
    const parts = amount.split('.')
    const whole = parts[0] || '0'
    const fraction = parts[1]?.padEnd(18, '0').slice(0, 18) || '0'.repeat(18)
    return whole + fraction
  }

  return (
    <div className="dashboard">
      <h2>User Dashboard</h2>
      <div className="wallet-info">
        <p><strong>Wallet:</strong> {wallet}</p>
      </div>

      <div className="balances">
        <div className="balance-card">
          <h3>UE Balance</h3>
          <p className="balance-amount">{formatWad(ueBalance)} UE</p>
        </div>
        <div className="balance-card">
          <h3>BU Balance</h3>
          <p className="balance-amount">{formatWad(buBalance)} BU</p>
        </div>
        <div className="balance-card">
          <h3>Rate Index</h3>
          <p className="balance-amount">{formatWad(rateIndex)} BU/UE</p>
        </div>
      </div>

      <div className="actions">
        <div className="action-card">
          <h3>Claim UBI</h3>
          <p>Claim your monthly UBI (696 UE per epoch)</p>
          <button onClick={claimUBI} disabled={loading}>
            {loading ? 'Claiming...' : 'Claim UBI'}
          </button>
        </div>

        <div className="action-card">
          <h3>Convert UE → BU</h3>
          <p>Convert UE to BU (with decay)</p>
          <div className="input-group">
            <input
              type="text"
              placeholder="UE amount"
              value={conversionAmount}
              onChange={(e) => setConversionAmount(e.target.value)}
            />
            <input
              type="text"
              placeholder="Min BU out"
              value={minBuOut}
              onChange={(e) => setMinBuOut(e.target.value)}
            />
            <button onClick={requestConversion} disabled={loading}>
              {loading ? 'Converting...' : 'Convert'}
            </button>
          </div>
        </div>
      </div>

      <PendingConversions wallet={wallet} />

      {message && (
        <div className={`message ${message.startsWith('Error') ? 'error' : 'success'}`}>
          {message}
        </div>
      )}
    </div>
  )
}

