type User = {
  loggedIn: boolean
  accountId: string
  balance: string
}

type userContextType = {
  user: User
  setUser: React.Dispatch<React.SetStateAction<User>>
}

export type { User, userContextType }
