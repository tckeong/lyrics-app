import styles from "./styles/dropDown";
import Cookies from "js-cookie";
import * as dialog from "@tauri-apps/plugin-dialog";

interface DropdownProps {
    open: boolean;
}

const Dropdown = ({ open }: DropdownProps) => {
    const logout = () => {
        Cookies.remove("user");
        Cookies.set("logout", "");
        dialog.message("Logout success!", {
            title: "spotify-lyrics-app",
            kind: "info",
        });
        window.location.reload();
    };

    const handleClick = () => {
        dialog
            .confirm("Are you sure you want to logout?", {
                title: "Logout",
            })
            .then((result) => {
                if (result) {
                    logout();
                }
            })
            .catch((error) => {
                console.log(error);
            });
    };

    return (
        <>
            {open && (
                <div className="origin-top-left absolute -right-10 mt-2 w-36 rounded-md shadow-lg bg-white ring-1 ring-black ring-opacity-5">
                    <div
                        className="py-1"
                        role="menu"
                        aria-orientation="vertical"
                        aria-labelledby="options-menu"
                    >
                        <button
                            className={styles.button}
                            onClick={handleClick}
                            role="menuitem"
                        >
                            Logout
                        </button>
                    </div>
                </div>
            )}
        </>
    );
};

export default Dropdown;
