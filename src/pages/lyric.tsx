import { useNavigate } from "react-router-dom";

function Lyric() {
    const navigate = useNavigate();

    return (
        <div>
            <button onClick={() => navigate('/')}>Home</button>
        </div>
    );
}

export default Lyric;