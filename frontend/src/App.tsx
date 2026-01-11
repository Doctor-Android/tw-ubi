import { useState } from 'react'
import './App.css'
import UserDashboard from './components/UserDashboard'
import AdminDashboard from './components/AdminDashboard'

function App() {
  const [wallet, setWallet] = useState<string>('')
  const [isAdmin, setIsAdmin] = useState(false)

  return (
    <div className="App">
      <header>
        <h1>TW-UBI System</h1>
        <div className="wallet-input">
          <input
            type="text"
            placeholder="Enter wallet address"
            value={wallet}
            onChange={(e) => setWallet(e.target.value)}
          />
          <button onClick={() => setIsAdmin(!isAdmin)}>
            {isAdmin ? 'User Mode' : 'Admin Mode'}
          </button>
        </div>
      </header>
      
      {wallet ? (
        isAdmin ? (
          <AdminDashboard wallet={wallet} />
        ) : (
          <UserDashboard wallet={wallet} />
        )
      ) : (
        <div className="welcome">
          <p>Enter your wallet address to begin</p>
        </div>
      )}
    </div>
  )
}

export default App
