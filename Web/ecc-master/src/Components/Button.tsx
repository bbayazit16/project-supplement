import {FC, ReactElement} from 'react';

interface IImgSize {
  height: string;
  width: string;
}

interface IButton {
  text: string;
  onClick?: () => void;
  image?: any;
  imageExtra?: string;
  imageSize?: IImgSize;
  alt?: string;
  showBG?: boolean;
}

const Button: FC<IButton> = ({
  text,
  onClick,
  image,
  imageExtra = '',
  imageSize,
  alt,
  showBG = true
}): ReactElement => {
  return (
    <div
      className="flex flex-row-reverse justify-center items-center bg-lapis-900 p-2 h-10
      rounded-xl cursor-pointer border-lapis-700 border-4 hover:scale-125 duration-700
      hover:bg-lapis-800 group"
      onClick={onClick}
    >
      {image ? (
        <div
          className={['rounded-full h-6 w-6 ml-1 overflow-hidden border-black border-4']
            .concat([!showBG ? 'bg-lapis-900 border-none group-hover:bg-lapis-800' : 'bg-black'])
            .join(' ')}
        >
          <img
            src={image}
            alt={alt}
            className={['select-none'].concat([imageExtra]).join(' ')}
            style={imageSize}
          ></img>
        </div>
      ) : null}
      <span className="font-bold select-none">{text}</span>
    </div>
  );
};

export default Button;
