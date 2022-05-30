import { useState, useEffect } from 'react'
import NavBar from '@components/NavBar'
import ChatArea from '@components/ChatArea'
import { userContext, createInitialUserState } from '@contexts/userContext'
import { wallet, getWalletBalance } from '@near/connect'

const App = () => {

  const [user, setUser] = useState(createInitialUserState())

  useEffect(() => {
    if (wallet.getAccountId()) {
      getWalletBalance().then(balance => {

        setUser({
          accountId: wallet.getAccountId(),
          loggedIn: true,
          balance
        })
      })
    }
  }, [])

  return (
    <userContext.Provider value={{ user, setUser }}>
      <div className='flex h-screen flex-col bg-skin-base'>
        <NavBar />
        <ChatArea />
      </div>
    </userContext.Provider>
  )
}

export default App
