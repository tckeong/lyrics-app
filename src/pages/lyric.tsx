import HomeButton from "../components/homeButton";
import { invoke } from "@tauri-apps/api/core";
import { useEffect, useRef, useState } from "react";
import styles from "./styles/lyric";
import LyricsArea from "../components/lyricsArea";
import SaveButton from "../components/saveButton";
import AddMinusButton from "../components/addMinusButton";

function Lyric() {
    const [image_url, setImageUrl] = useState<string>("");
    const [id, setId] = useState<string>("");
    const offset = 500;
    const timeOffSet = useRef<number>(offset);
    const prevSpeed = useRef<number>(timeOffSet.current - offset);
    const [speed, setSpeed] = useState<number>(timeOffSet.current - offset);

    useEffect(() => {
        invoke("get_image_url").then((url) => {
            setImageUrl(url as string);
        });
    }, [id]);

    useEffect(() => {
        if (speed > prevSpeed.current) {
            timeOffSet.current -= 200;
        } else {
            timeOffSet.current += 200;
        }

        prevSpeed.current = speed;
    }, [speed]);

    useEffect(() => {
        const interval = setInterval(() => {
            invoke("get_id")
                .then((curId) => curId as string)
                .then((curId) => {
                    if (curId !== id) {
                        setId(curId);
                    }
                });
        }, 1000);

        return () => clearInterval(interval);
    }, []);

    return (
        <div className={styles.outer}>
            <div
                className={styles.image}
                style={{ backgroundImage: `url(${image_url})` }}
            />
            <div className={styles.inner}>
                <LyricsArea
                    position="row-start-1 row-span-5 col-start-1 col-span-4"
                    timeOffSet={timeOffSet}
                />
                <HomeButton
                    position="row-start-1 row-end-2 col-start-5 col-end-6"
                    fn={async () => {
                        await invoke("original_window");
                        invoke("close_window");
                    }}
                />
                <p className={styles.text}>time delay: {speed} ms</p>
                <AddMinusButton
                    position="row-start-4 row-end-5 col-start-5 col-end-6"
                    setSpeed={setSpeed}
                />
                <SaveButton position="row-start-5 row-end-6 col-start-5 col-end-6" />
            </div>
        </div>
    );
}

export default Lyric;
