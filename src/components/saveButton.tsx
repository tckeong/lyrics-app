import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import { faFloppyDisk } from "@fortawesome/free-regular-svg-icons";
import { invoke } from "@tauri-apps/api/core";
import * as dialog from "@tauri-apps/plugin-dialog";

interface SaveButtonProps {
    position: string;
}

function SaveButton({ position }: SaveButtonProps) {
    const className = `text-xl text-gray-200 mx-12 my-8 hover:text-indigo-500 ${position}`;

    const handleSave = () => {
        invoke("save_lyrics").then(() => {
            dialog.message("Lyrics saved!", {
                title: "spotify-lyrics-app",
                kind: "info",
            });
        });
    };

    return (
        <button className={className} onClick={handleSave}>
            <FontAwesomeIcon icon={faFloppyDisk} size="xl" /> Save
        </button>
    );
}

export default SaveButton;
