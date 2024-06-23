import { useState } from "react";
import { invoke, dialog } from "@tauri-apps/api";
import { open } from "@tauri-apps/api/shell";
import HomeButton from "../components/homeButton";
import styles from "./styles/login";
import server from "../api/server";
import { useNavigate } from "react-router-dom";

function Login() {
    const [clientId, setClientId] = useState<string>('');
    const [clientSecret, setClientSecret] = useState<string>('');
    const navigate = useNavigate();

    const loginMessage = (login: boolean) => {
        if (login) {
            dialog.message('login success!', {
                title: 'spotify-lyrics-app',
                type: 'info'
            });
            navigate("/");
            // TODO!
            localStorage.setItem("token", "token");
            localStorage.setItem("user", "user");
        } else {
            dialog.message('login unsuccess!', {
                title: 'spotify-lyrics-app',
                type: 'info'
            });
            setClientId("");
            setClientSecret("");
        }
    }

    const handleAuth = async () => {
        await invoke("login", { client_id: clientId, client_secret: clientSecret });

        const client_id = clientId;
        const redirect_uri = `${server.backend_Url}/callback`;
        const scope = 'user-read-private user-read-email';
        const queryParams = new URLSearchParams({
            response_type: 'code',
            client_id: client_id,
            scope: scope,
            redirect_uri: redirect_uri,
        }).toString();
        
        const auth_url = 'https://accounts.spotify.com/authorize?' + queryParams;
        open(auth_url);
        await invoke("login_test")
                .then((_) => loginMessage(true))
                .catch((_) => loginMessage(false));
    }

    return (
        <div className="grid grid-rows-4 w-full h-full">
            <div className="grid grid-cols-3 row-start-1 row-end-2">
                <h1 className={styles.title}>Login</h1>
                <HomeButton position="col-start-3 col-end-4" />
            </div>
            <div className="grid grid-cols-3 grid-rows-3 row-start-2 row-span-3">
                <div className="items-center grid grid-cols-5 row-start-1 row-end-2 col-start-1 col-span-3">
                    <label htmlFor="clientId" className="col-start-2 col-end-3 font-bold font-mono text-lg">Client ID:</label>
                    <input type="text" id="clientId" className="p-2 col-start-3 col-span-2 rounded" 
                           value={clientId} 
                           onChange={(e) => setClientId(e.target.value)} 
                           placeholder="Client ID"
                    />
                </div>
                <div className="items-center grid grid-cols-5 row-start-2 row-end-3 col-start-1 col-span-3">
                    <label htmlFor="clientSecret" className="col-start-2 col-end-3 font-bold font-mono text-lg">Client Secret:</label>
                    <input type="text" id="clientSecret" className="p-2 col-start-3 col-span-2 rounded"
                           value={clientSecret} 
                           onChange={(e) => setClientSecret(e.target.value)}
                           placeholder="Client Secret"
                    />
                </div>
                <button onClick={handleAuth} className={styles.button}>Authorization</button>
            </div>
        </div>
    );
}

export default Login;