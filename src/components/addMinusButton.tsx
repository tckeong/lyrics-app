import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import { faPlus, faMinus } from "@fortawesome/free-solid-svg-icons";

interface AddMinusButtonProps {
    position: string;
    timeOffSet: React.MutableRefObject<number>;
}

function AddMinusButton({position, timeOffSet}: AddMinusButtonProps) {
    const className = `flex justify-center items-center font-bold -mr-5 ${position}`;
    const buttonClassName = "px-4 pl-4 hover:text-zinc-600";

    return (
    <div className={className}>
      <button onClick={() => timeOffSet.current -= 200} className={buttonClassName}><FontAwesomeIcon icon={faPlus} size="2x" /></button>
      <button onClick={() => timeOffSet.current += 200} className={buttonClassName}><FontAwesomeIcon icon={faMinus} size="2x" /></button>
      <p></p>
    </div>
  );
}

export default AddMinusButton;