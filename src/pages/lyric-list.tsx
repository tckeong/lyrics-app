import { useNavigate } from "react-router-dom";

function LyricsList() {
    const navigate = useNavigate();
    
    return (
        <div>
            <h1>Lyrics List</h1>
            <button onClick={() => navigate("/")}>Home</button>
        </div>
    );
}

export default LyricsList;