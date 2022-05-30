import { makeAutoObservable } from 'mobx'

class Account {
  State = ''

  constructor() {
    makeAutoObservable(this)
  }

  setAccountId(account: string) {
    this.State = account
  }
}

export default new Account()
