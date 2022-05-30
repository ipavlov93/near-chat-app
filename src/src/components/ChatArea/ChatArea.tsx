import MessagesArea from '@components/ChatArea/MessagesArea'
import SendArea from '@components/ChatArea/SendArea'

const ChatArea = () => {
  return (
    <div
      className={`m-auto flex w-full flex-auto flex-col bg-skin-chatArea sm:max-w-full md:max-w-xl xl:max-w-2xl overflow-scroll `}
    >
      <MessagesArea />
      <SendArea />
    </div>
  )
}

export default ChatArea
