import HomeButton from "../components/homeButton";
import { invoke } from "@tauri-apps/api";
import { useEffect, useState } from "react";
import styles from "./styles/lyric";
import LyricsArea from "../components/lyricsArea";
import SaveButton from "../components/saveButton";

function Lyric() {
    const [image_url, setImageUrl] = useState<string>("");
    const [id, setId] = useState<string>("");

    useEffect(() => {
        invoke("get_image_url").then((url) => {
            setImageUrl(url as string);
        });
    }, [id]);

    useEffect(() => {
        const interval = setInterval(() => {
            invoke("get_id").then((curId) => curId as string).then((curId) => {
                if (curId !== id) {
                    setId(curId);
                }
            });
        }, 1000);
        
        return () => clearInterval(interval);
    }, []);

    return (
        <div className={styles.outer}>
            <div className={styles.image} style={{backgroundImage: `url(${image_url})`}} />
            <div className={styles.inner}>
                <LyricsArea position="row-start-1 row-span-5 col-start-1 col-span-4" />
                <HomeButton position="row-start-1 row-end-2 col-start-5 col-end-6" 
                    fn={async () => {await invoke("original_window"); invoke("close_window")}}/>
                <SaveButton position="row-start-5 row-end-6 col-start-5 col-end-6" />
            </div>
        </div>
    );
}

export default Lyric;