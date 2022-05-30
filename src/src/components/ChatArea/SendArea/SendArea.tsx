import {ChangeEvent, FormEvent, useState} from 'react'
import {PaperAirplaneIcon} from '@heroicons/react/solid'
import Account from '@store/Account'
import {sendMessage} from '@near/connect'

const SendArea = () => {
    const [inputText, setInputText] = useState('')

    const handleFormSubmit = (e: FormEvent<HTMLFormElement>) => {
        e.preventDefault()
        if (inputText.trim().length > 0) {
            if (Account.State !== '') {
                sendMessage(Account.State, inputText.trim())
            }
        }
    }

    const handleClick = () => {
        if (inputText.trim().length > 0) {
            if (Account.State !== '') {
                sendMessage(Account.State, inputText.trim())
            }
        }
    }

    return (
        <form
            className='flex h-14 items-center gap-4 bg-skin-bar px-4 py-2'
            onSubmit={handleFormSubmit}
        >
            <input
                value={inputText}
                onChange={(e: ChangeEvent<HTMLInputElement>) => setInputText(e.target.value)}
                className='h-full w-full rounded-md border border-skin-invert bg-transparent px-2 text-lg placeholder:text-gray-600'
                placeholder='Input message...'
            ></input>

            <button onClick={handleClick} className='bg-transparent'>
                <PaperAirplaneIcon className='w-10 h-10 rotate-90 text-skin-base'/>
            </button>
        </form>
    )
}

export default SendArea
