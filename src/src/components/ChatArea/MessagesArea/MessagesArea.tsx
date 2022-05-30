import Account from '@store/Account'
import {useEffect, useMemo, useState} from 'react'
import {getInitialMessages, getMessages} from '@near/connect'
import Bubble from '@components/ChatArea/MessagesArea/Bubble'

type Props = {
    className?: string
}

const MessagesArea = ({className: customClassName = ''}: Props) => {

    const [messages, setMessages] = useState(getInitialMessages)
    const [account, setAccount] = useState('')

    useEffect(() => {
        setInterval(() => {
            if (Account.State !== '') {
                getMessages(Account.State).then(fetched => {
                    if (fetched.messages.length != messages.messages.length) {
                        setMessages(fetched)
                    }
                })
            } else {
                setAccount(Account.State)
            }
        }, 1000)
    }, [])

    const messageBubbles = useMemo(() => {

        return messages.messages.map((msg, index) => {
            return <Bubble key={index} sender={msg.sender} text={msg.text} created_at={msg.created_at}/>
        })
    }, [messages])

    return (
        <div className='overflow-auto flex-auto'>
            {Account.State !== '' ? (
                <div className='flex flex-auto flex-col items-start'>{messageBubbles}</div>
            ) : (
                <div className={'flex flex-auto'}>Choose your chat using drop down please</div>
            )}
        </div>
    )
}

export default MessagesArea
