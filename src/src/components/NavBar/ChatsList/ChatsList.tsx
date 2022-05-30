import {useEffect, useMemo, useState} from 'react'
import {getChatAccounts, getInitialAccounts} from '@near/connect'
import Item from "./Item";
import {ChevronDownIcon} from "@heroicons/react/outline";

type Props = {
    className?: string
}

const ChatList = ({className: customClassName = ''}: Props) => {

    const [accounts, setAccounts] = useState(getInitialAccounts)

    const handleClick = () => {
        setShowDropDown(!showDropDown)
    }
    const [showDropDown, setShowDropDown] = useState(false)

    useEffect(() => {
        setInterval(() => {
            getChatAccounts().then(fetched => {
                if (fetched.accounts.length != accounts.accounts.length) {
                    setAccounts(fetched)
                }
            })
        }, 1000)
    }, [])

    const accountItems = useMemo(() => {

        return accounts.accounts.map((acc, index) => {
            return <Item key={index} accountId={acc}/>
        })
    }, [accounts])

    const handleOnMouseLeave = () => {
        setShowDropDown(false)
    }

    return (
        <div onClick={handleClick} className='relative w-25'>
            <div className='flex items-center text-skin-base'>
                <span>Choose chat</span>
                <ChevronDownIcon className='ml-2 h-6 w-6'/>
            </div>
            {showDropDown && (
                <div className='absolute mt-1 w-full select-none rounded-md bg-skin-base overflow-auto flex-auto'>
                    {Number(accounts.total) > 0 ? (
                        <div onMouseLeave={handleOnMouseLeave}
                             className='flex flex-auto flex-col items-start'>{accountItems}</div>
                    ) : (
                        <div>empty accounts list</div>
                    )}
                </div>
            )
            }
        </div>
    )
}

export default ChatList