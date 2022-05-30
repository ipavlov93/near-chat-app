import {useContext, useState} from 'react'
import {UserCircleIcon} from '@heroicons/react/solid'
import {userContext} from '@contexts/userContext'
import {connectWallet, formatBalanceToHuman, wallet} from '@near/connect'
import {ChevronDownIcon} from '@heroicons/react/outline'
import ChatList from "./ChatsList";

type Props = {
    className?: string
}

const NavBar = ({className: customClassName}: Props) => {
    const {user} = useContext(userContext)
    const [showDropDown, setShowDropDown] = useState(false)

    const handleLoginClick = () => {
        // after connectWallet, page will change, so .then() have no sense in here
        connectWallet()
    }

    const handleLogoutClick = () => {
        wallet.signOut()
        location.reload()
    }

    const handleOnMouseLeave = () => {
        setShowDropDown(false)
    }

    return (
        <header
            className={`flex h-14 items-center justify-between bg-skin-bar px-6 py-3 ${customClassName}`}
        >
            <h2 className='col-1 text-xl'>near-chat 🔥</h2>
            <div className='flex container'>
                <ChatList className='col-5 flex items-center text-skin-base'/>
            </div>
            <div className='flex cursor-pointer items-center text-lg'>
                {user.loggedIn ? (
                    <>
                        <UserCircleIcon className='h-10 w-10 text-skin-invert'/>
                        <div onClick={() => setShowDropDown(!showDropDown)} className='relative'>
                            <div className='flex items-center text-skin-base'>
                                <span>{user.accountId}</span>
                                <ChevronDownIcon className='ml-2 h-6  w-6'/>
                            </div>
                            {showDropDown && (
                                <div onMouseLeave={handleOnMouseLeave} className='absolute mt-1 w-full select-none rounded-md bg-skin-base'>
                                    <div className='rounded-md border border-slate-400 px-2'>
                                        balance: {formatBalanceToHuman(user.balance)}
                                    </div>
                                    <div
                                        onClick={handleLogoutClick}
                                        className='rounded-md border border-slate-400 px-2'
                                    >
                                        Log out
                                    </div>
                                </div>
                            )}
                        </div>
                    </>
                ) : (
                    <button type="button" className='btn btn-light' onClick={handleLoginClick}>Login</button>
                )}
            </div>
        </header>
    )
}

export default NavBar
