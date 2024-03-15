import type { FC } from 'hono/jsx';

const ChatMessage: FC<{ message: string }> = ({ message }) => <div
  class="chat-message"
  _="init wait 15s then add .hidden then settle then remove me"
>
  {message}
</div>;

export default ChatMessage;
