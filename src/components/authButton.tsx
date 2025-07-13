import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import { faKey } from "@fortawesome/free-solid-svg-icons";
import { invoke } from "@tauri-apps/api/core";
import { open } from "@tauri-apps/plugin-shell";
import Cookies from "js-cookie";
import { useNavigate } from "react-router-dom";
import { message } from "@tauri-apps/plugin-dialog";

function AuthButton() {
    const navigate = useNavigate();
    const className =
        "bg-gray-300 border-2 border-slate-500 rounded-lg text-md mx-2 py-2 px-2 font-mono hover:text-blue-700";

    const loginMessage = async (
        username: string | boolean,
        err: string | null
    ) => {
        if (username) {
            message("login success!", {
                title: "spotify-lyrics-app",
                kind: "info",
            });

            Cookies.set("user", username as string);
            navigate("/");
        } else {
            message(`login unsuccess! ${err}`, {
                title: "spotify-lyrics-app",
                kind: "info",
            });
        }
    };

    const handleAuth = async () => {
        await invoke("auth_check")
            .then((data) => {
                let [client_id, client_secret] = data as string[];
                return [client_id, client_secret];
            })
            .then(async ([client_id, client_secret]) => {
                invoke("login", {
                    client_id: client_id,
                    client_secret: client_secret,
                })
                    .then(async (auth_url) => {
                        await open(auth_url as string);
                        await setTimeout(async () => {
                            await invoke("get_username")
                                .then((name) =>
                                    loginMessage(name as string, null)
                                )
                                .catch((err) =>
                                    loginMessage(false, err as string)
                                );
                        }, 2000);
                    })
                    .catch((err) => console.log(err));
            });
    };

    return (
        <button onClick={handleAuth} className={className}>
            <FontAwesomeIcon icon={faKey} size="lg" /> Authentication
        </button>
    );
}

export default AuthButton;
