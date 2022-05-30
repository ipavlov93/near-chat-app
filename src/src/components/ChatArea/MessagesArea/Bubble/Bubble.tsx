import { Message } from '@near/connect'

type Props = Message

const Bubble = (message: Props) => {
  const date = new Date(message.created_at / Math.pow(10, 6))

  // prettier-ignore
    let minutesFormat = date.getMinutes().toString()
    if (date.getMinutes() < 10) {
        minutesFormat = '0' + date.getMinutes()
    }
  const formattedDate =
    date.getHours() + ':' + minutesFormat

  return (
    <div className='row mt-4 ml-2 flex max-w-[80%] flex-col items-end rounded-xl bg-skin-bubble px-2 py-0 last-of-type:mb-4'>
      <div className='font-semibold'>{message.sender}</div>
      <div className='py-1'>{message.text}</div>
      <div className='font-mono text-xs text-right'>{formattedDate}</div>
    </div>
  )
}

export default Bubble
