import Account from '@store/Account'

type Props = { accountId?: string }

const Item = ({accountId: accountId = ''}: Props) => {
    const handleClick = () => {
        Account.setAccountId(accountId)
    }

    return (
        <div className='rounded-md border border-slate-400 px-2 overflow-scroll'>
            {Account.State === accountId ? (
                <button onClick={handleClick} type="button" className='btn btn-info btn-block'>
                    {accountId}
                </button>
            ) : (
                <button onClick={handleClick} type="button" className='btn btn-outline-light btn-block'>
                    {accountId}
                </button>
            )}
        </div>
    )
}

export default Item
