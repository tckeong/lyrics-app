import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import { faHome } from "@fortawesome/free-solid-svg-icons";
import { useNavigate } from "react-router-dom";

interface HomeButtonProps {
    position: string;
}

function HomeButton({position}: HomeButtonProps) {
    const navigate = useNavigate();
    const className = "text-xl mx-16 my-8 hover:text-sky-800 " + position;

    return (
        <button className={className} onClick={() => navigate("/")}>
            <FontAwesomeIcon icon={faHome} size="xl"/> Home
        </button>
    );
}

export default HomeButton;