import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { open } from "@tauri-apps/plugin-shell";
import HomeButton from "../components/homeButton";
import styles from "./styles/login";
import { useNavigate } from "react-router-dom";
import Cookies from "js-cookie";
import * as dialog from "@tauri-apps/plugin-dialog";

function Login() {
    const [clientId, setClientId] = useState<string>("");
    const [clientSecret, setClientSecret] = useState<string>("");
    const navigate = useNavigate();

    const loginMessage = async (
        username: string | boolean,
        err: string | null
    ) => {
        if (username) {
            dialog.message("login success!", {
                title: "spotify-lyrics-app",
                kind: "info",
            });

            Cookies.set("user", username as string);
            navigate("/");
        } else {
            dialog.message(`login unsuccess! ${err}`, {
                title: "spotify-lyrics-app",
                kind: "info",
            });
            setClientId("");
            setClientSecret("");
        }
    };

    const handleAuth = async () => {
        await invoke("login", {
            client_id: clientId,
            client_secret: clientSecret,
        })
            .then(async (auth_url) => {
                await open(auth_url as string);
                await setTimeout(async () => {
                    await invoke("get_username")
                        .then((name) => loginMessage(name as string, null))
                        .catch((err) => loginMessage(false, err as string));
                }, 1000);
            })
            .catch((err) => console.log(err));
    };

    return (
        <div className="grid grid-rows-4 w-full h-full">
            <div className="grid grid-cols-3 row-start-1 row-end-2">
                <h1 className={styles.title}>Login</h1>
                <HomeButton position="col-start-3 col-end-4" />
            </div>
            <div className="grid grid-cols-3 grid-rows-3 row-start-2 row-span-3">
                <div className="items-center grid grid-cols-5 row-start-1 row-end-2 col-start-1 col-span-3">
                    <label
                        htmlFor="clientId"
                        className="col-start-2 col-end-3 font-bold font-mono text-lg"
                    >
                        Client ID:
                    </label>
                    <input
                        type="text"
                        id="clientId"
                        className="p-2 col-start-3 col-span-2 rounded"
                        value={clientId}
                        onChange={(e) => setClientId(e.target.value)}
                        placeholder="Client ID"
                    />
                </div>
                <div className="items-center grid grid-cols-5 row-start-2 row-end-3 col-start-1 col-span-3">
                    <label
                        htmlFor="clientSecret"
                        className="col-start-2 col-end-3 font-bold font-mono text-lg"
                    >
                        Client Secret:
                    </label>
                    <input
                        type="text"
                        id="clientSecret"
                        className="p-2 col-start-3 col-span-2 rounded"
                        value={clientSecret}
                        onChange={(e) => setClientSecret(e.target.value)}
                        placeholder="Client Secret"
                    />
                </div>
                <button onClick={handleAuth} className={styles.button}>
                    Authorization
                </button>
            </div>
        </div>
    );
}

export default Login;
