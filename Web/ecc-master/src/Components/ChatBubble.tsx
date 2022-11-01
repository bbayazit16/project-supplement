import {FC, ReactElement} from 'react';

interface IChatBubble {
  message: string;
  self: boolean;
}

const ChatBubble: FC<IChatBubble> = ({message, self}): ReactElement => {
  return (
    <div className={['flex px-2'].concat([self ? 'flex-row-reverse pl-16' : 'flex-row pr-16']).join(' ')}>
      <div
        className={['flex justify-center items-center w-fit p-1 rounded-xl break-all']
          .concat([self ? 'bg-lapis-700' : 'bg-slate-700'])
          .join(' ')}
      >
        <span>{message}</span>
      </div>
    </div>
  );
};

export default ChatBubble;
