import { useNavigate } from "react-router-dom";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import { faRightToBracket } from "@fortawesome/free-solid-svg-icons";
import { invoke } from "@tauri-apps/api/core";
import styles from "./styles/index";
import UserButton from "../components/userButton";
import Cookies from "js-cookie";
import { useEffect, useState } from "react";
import AuthButton from "../components/authButton";
import { message } from "@tauri-apps/plugin-dialog";

function Index() {
    const user = Cookies.get("user") ?? "";
    const logout = Cookies.get("logout") === "" ? true : false;
    const login = user !== "";
    const [auth, setAuth] = useState<boolean>(false);
    const navigate = useNavigate();

    const handleStart = async () => {
        invoke("login_test")
            .then(async (_) => {
                navigate("/lyrics");
            })
            .catch((_) => {
                message("Please login!", {
                    title: "spotify-lyrics-app",
                    kind: "info",
                });
            });
    };

    useEffect(() => {
        invoke("auth_check")
            .then((_) => setAuth(true))
            .catch((err) => console.log(err));
    }, []);

    return (
        <div className="grid grid-rows-3 w-full h-full">
            <div className="grid grid-cols-4 row-start-1 row-end-2">
                <h1 className={styles.title}>Spotify Lyrics App</h1>
                <div className="flex items-center justify-center col-start-4 col-end-5">
                    {login ? (
                        <UserButton name={user} />
                    ) : auth && !logout ? (
                        <AuthButton />
                    ) : (
                        <button
                            className={styles.loginButton}
                            onClick={() => navigate("/login")}
                        >
                            <FontAwesomeIcon icon={faRightToBracket} /> login
                        </button>
                    )}
                </div>
            </div>
            <div className="grid grid-rows-2 grid-cols-3 row-start-2 row-span-2">
                <button className={styles.startButton} onClick={handleStart}>
                    start
                </button>
                <button
                    className={styles.lyricsLsButton}
                    onClick={() => navigate("lyrics-ls")}
                >
                    Lyrics List
                </button>
            </div>
        </div>
    );
}

export default Index;
