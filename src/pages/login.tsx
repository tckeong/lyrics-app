import { useNavigate } from "react-router-dom";

function Login() {
    const navigate = useNavigate();

    const handleLogin = () => {
        localStorage.setItem("token", "token");
    }

    return (
        <div>
            <h1>Login</h1>
            <button onClick={handleLogin}>Login</button>
            <button onClick={() => navigate('/')}>Home</button>
        </div>
    );
}

export default Login;