import { createContext } from 'react'
import { User, userContextType } from '@baseTypes/user'

export const userContext = createContext({} as userContextType)

/**
 * function returns userState with default type values
 *
 ****
 * 
 ** {string} accountId - User's NEAR wallet (account) Id
 ** {number} balance - Wallet balance
 ** {boolean} loggedIn - User login status
 */
export const createInitialUserState = () =>
  ({
    accountId: '',
    balance: '',
    loggedIn: false
  } as User)
