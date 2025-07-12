import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import { faUser } from "@fortawesome/free-solid-svg-icons";
import { useState } from "react";

interface UserButtonProps {
    name: string;
}

function UserButton({ name }: UserButtonProps) {
    const [open, setOpen] = useState(false);

    const hancleClick = () => {
        setOpen(!open);
    }

    return (
        <div className="relative inline-block">
            <button className="text-xl px-2 py-2 hover:text-blue-700" onClick={hancleClick}>
                <FontAwesomeIcon icon={faUser} size="xl" /> {name}
            </button>
        </div>
    );
}

export default UserButton;