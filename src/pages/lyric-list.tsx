import HomeButton from "../components/homeButton";
import styles from "./styles/lyrics-list";
import { invoke } from "@tauri-apps/api/core";
import { useEffect, useState } from "react";

interface SavedLyric {
    name: string;
    artist: string;
    img: string;
}

function LyricsList() {
    const [lyrics, setLyrics] = useState<SavedLyric[]>([]);

    useEffect(() => {
        invoke("get_lyrics_list")
            .then((res) => {
                setLyrics(res as SavedLyric[]);
            })
            .catch((err) => console.log(err));
    }, []);

    return (
        <div className="grid grid-rows-5 w-full h-full">
            <div className="grid grid-cols-3 row-start-1 row-end-2 mb-4">
                <h1 className={styles.title}>Lyrics List</h1>
                <HomeButton position="col-start-3 col-end-4" />
            </div>
            <div className="row-start-2 row-span-4 ml-5 overflow-y-auto scrollbar">
                <table className="table-fixed w-full h-full text-center font-mono">
                    <thead className="text-white">
                        <tr>
                            <th>Poster</th>
                            <th>Song</th>
                            <th>Artist</th>
                        </tr>
                    </thead>
                    <tbody className="text-gray-800">
                        {lyrics.map((savedLyric, id) => {
                            return (
                                <tr key={id}>
                                    <td>
                                        <img
                                            src={savedLyric.img}
                                            className="h-[194px] w-[258px]"
                                        />
                                    </td>
                                    <td className="font-bold">
                                        {savedLyric.name}
                                    </td>
                                    <td>{savedLyric.artist}</td>
                                </tr>
                            );
                        })}
                    </tbody>
                </table>
            </div>
        </div>
    );
}

export default LyricsList;
