import {ConnectConfig, Contract, keyStores, Near, utils, WalletConnection} from 'near-api-js'
import Account from '@store/Account'
import {Buffer} from 'buffer'

const defaultRouterContract = 'dev-1653632038900-21214400306009'

window.Buffer = Buffer

type walletInfo = {
    accountId: any
    balance: string
}

export const near = new Near({
    headers: {},
    networkId: 'testnet',
    keyStore: new keyStores.BrowserLocalStorageKeyStore(),
    nodeUrl: 'https://rpc.testnet.near.org',
    walletUrl: 'https://wallet.testnet.near.org',
    helperUrl: 'https://helper.testnet.near.org',
    explorerUrl: 'https://explorer.testnet.near.org'
} as ConnectConfig)

export const wallet = new WalletConnection(near, 'nixchat-wow')

export const connectWallet = async () => {
    return await wallet.requestSignIn()
}

export const getWalletBalance = async () => {
    const balance = (await wallet.account().state()).amount
    return utils.format.formatNearAmount(balance) as string
}

export const formatBalanceToHuman = (balance: string) => {
    const numberBalance = Number(balance)

    return numberBalance.toFixed(3)
}

const chatMethodOptions = {
    viewMethods: ['get_messages', 'is_whitelisted_account'],
    changeMethods: ['send_message']
}

const routerMethodOptions = {
    viewMethods: ['get_chat_accounts'],
    changeMethods: []
}

// prettier-ignore
export const routerContract = new Contract(
    wallet.account(),
    defaultRouterContract,
    routerMethodOptions
)

const params = {
    page: 1,
    limit: 10
}

export const isWhitelistedAccount = async (chatContract: string) : Promise<boolean> => {
    if (wallet.getAccountId() !== '') {
        const res: boolean = await (new Contract(
            wallet.account(),
            chatContract || Account.State,
            chatMethodOptions
        ) as any).is_whitelisted_account({account_id: wallet.getAccountId()})
        return res
    }
    return false
}

export const getMessages = async (chatContract: string) => {
    // "as eny" assertion to ignore typescript error "Property does not exist on type"
    const messages: fetchedMessages = await (new Contract(
        wallet.account(),
        chatContract,
        chatMethodOptions
    ) as any).get_messages(params)
    return messages
}

export const sendMessage = async (chatContract: string, text: string) => {
    // "as eny" assertion to ignore typescript error "Property does not exist on type"
    await (new Contract(
        wallet.account(),
        chatContract,
        chatMethodOptions
    ) as any).send_message({
        text_message: text
    })
}

export const getChatAccounts = async () => {
    // "as eny" assertion to ignore typescript error "Property does not exist on type"
    const accounts: fetchedAccounts = await (routerContract as any).get_chat_accounts(params)
    return accounts
}

export type Message = {
    text: string
    sender: string
    created_at: number
}

export type fetchedMessages = {
    messages: Message[]
    total: string
}

export type fetchedAccounts = {
    accounts: string[]
    total: string
}

export const getInitialMessages = (): fetchedMessages => {
    return {
        total: '0',
        messages: []
    }
}

export const getInitialAccounts = (): fetchedAccounts => {
    return {
        total: '0',
        accounts: []
        // accounts: [defaultChatContract, 'dev-1653565385351-13217791340282']
    }
}
