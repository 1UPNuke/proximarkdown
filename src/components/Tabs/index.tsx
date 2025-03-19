import { useContext } from "preact/hooks";
import * as styles from "./style.module.css";
import { OpenedFiles } from "../App";

const Tabs = () => {
    const files = useContext(OpenedFiles).openedFiles;
    return (
        <nav className={styles.tabs}>
            {
                files.map(f =>(
                    <button>{f.saved ? "" : "*"}{f.name}</button>
                ))
            }
        </nav>
    );
};

export default Tabs;
