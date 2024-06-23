import HomeButton from "../components/homeButton";
import styles from "./styles/lyrics-list";

function LyricsList() {    
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
                        <tr>
                            <td><img src="https://i.ytimg.com/vi/dQw4w9WgXcQ/sddefault.jpg"/></td>
                            <td className="font-bold">Never Gonna Give You Up</td>
                            <td>Rick Astley</td>
                        </tr>
                        <tr>
                            <td><img src="https://i.ytimg.com/vi/dQw4w9WgXcQ/sddefault.jpg"/></td>
                            <td className="font-bold">Never Gonna Give You Up</td>
                            <td>Rick Astley</td>
                        </tr>
                    </tbody>
                </table>
            </div>
        </div>
    );
}

export default LyricsList;