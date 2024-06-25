import { useNavigate } from "react-router-dom";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import { faRightToBracket } from "@fortawesome/free-solid-svg-icons";
import { invoke } from "@tauri-apps/api";
import  styles  from "./styles/index"
import UserButton from "../components/userButton";
import Cookies from "js-cookie";

function Index() {
  const user = Cookies.get("user") ?? "";
  const login = user !== "";
  const navigate = useNavigate();

  return (
    <div className="grid grid-rows-3 w-full h-full">
      <div className="grid grid-cols-4 row-start-1 row-end-2">
        <h1 className={styles.title}>
          Spotify Lyrics App
        </h1>
        <div className="flex items-center justify-center col-start-4 col-end-5">
          { login 
            ? <UserButton name={user}/>
            : <button className={styles.loginButton} onClick={() => navigate("/login")}>
                  <FontAwesomeIcon icon={faRightToBracket} /> login
              </button> 
          }
        </div>
      </div>
      <div className="grid grid-rows-2 grid-cols-3 row-start-2 row-span-2">
        <button className={styles.startButton} onClick={async () => {await invoke("lyric_window"); invoke("close_window")}}>
          start
        </button>
        <button className={styles.lyricsLsButton} onClick={() => navigate("lyrics-ls")}>
          Lyrics List
        </button>
      </div>
    </div>
  );
}

export default Index;