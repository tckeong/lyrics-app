import { useNavigate } from "react-router-dom";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import { faUser } from "@fortawesome/free-solid-svg-icons";
import  styles  from "./styles/index"

function Index() {
  const token = localStorage.getItem("token");
  let login = token != null; 
  const navigate = useNavigate();

  return (
    <div className="grid grid-rows-3" style={{width: "100%", height: "100%"}}>
      <div className="grid grid-cols-4 row-start-1 row-end-2">
        <h1 className={styles.title}>
          Spotify Lyrics App
        </h1>
        <div className="flex items-center justify-center col-start-4 col-end-5">
          { login 
            ? <button className="text-lg px-10 py-10">
                <FontAwesomeIcon icon={faUser} size="lg" /> user
              </button>
            : <button className={styles.loginButton} onClick={() => navigate("/login")}>
                  login
              </button> 
          }
        </div>
      </div>
      <div className="grid grid-rows-2 grid-cols-3 row-start-2 row-span-2">
        <button className={styles.startButton} onClick={() => navigate("lyric/1")}>
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