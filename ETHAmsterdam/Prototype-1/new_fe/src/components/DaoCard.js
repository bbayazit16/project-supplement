export const DaoCard = (props) => {
  return (
    <div className="card bg-secondary border-2 border-white flex flex-col items-center justify-center p-4 shadow-lg rounded-2xl w-64">
      <div className="profile mx-auto py-2 w-32">
        <img className="rounded-full " src={props.img_url} alt="profile" />
      </div>
      <div className="font-semibold">
        {props.name}
      </div>
      <div className="font-light">
        {props.subtitle}
      </div>
      <div className="rounded-lg border-2 border-white mt-4">
          <div className = "px-5">
          View
          </div>
      </div>
    </div>
  );
};
