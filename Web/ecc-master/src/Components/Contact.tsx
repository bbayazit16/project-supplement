import {ReactElement, FC} from 'react';

interface IContact {
  displayName: string;
  avatar: any;
  lastMessage?: string;
}

const Contact: FC<IContact> = ({displayName, avatar, lastMessage}): ReactElement => {
  return (
    <div
      className="flex items-center justify-start h-10 space-x-2 bg-slate-800 rounded-lg
    cursor-pointer transition duration-500 ease-linear hover:bg-slate-700"
    >
      <div className="ml-4 hover:rotate-360 duration-1000">
        <div className="bg-black h-6 w-6 rounded-full overflow-hidden border-black border-4">
          <img src={avatar} className="select-none" alt="Chat Avatar"></img>
        </div>
      </div>
      <div className="flex flex-col -space-y-3">
        <span className="font-bold text-lg select-none text-lapis-200">
          {displayName.length >= 15 ? displayName.substring(0, 15) + '...eth' : displayName}
        </span>
        <span className="font-medium select-none">
          {lastMessage
            ? lastMessage.length > 20
              ? lastMessage.substring(0, 20) + '...'
              : lastMessage
            : 'Click to start messaging!'}
        </span>
      </div>
    </div>
  );
};

export default Contact;
