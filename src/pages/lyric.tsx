import HomeButton from "../components/homeButton";
import { invoke } from "@tauri-apps/api";

function Lyric() {
    const handleTest = async () => {
        await invoke("login_test");
    };

    return (
        <div>
            <h1>Lyric</h1>
            <HomeButton position="" />
            <button onClick={handleTest}>Test</button>
        </div>
    );
}

export default Lyric;