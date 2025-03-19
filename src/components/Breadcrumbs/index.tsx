import * as styles from "./style.module.css";
import { OpenedFiles } from "../App";
import { useContext } from "preact/hooks";

const Breadcrumbs = () => {
    let path = useContext(OpenedFiles).openedFiles[0].path;
    return (
        <nav className={styles.crumbs}>
            <span>{
                //TODO: Generate actual breadcrumbs instead of just displaying the path
                path
            }</span>
        </nav>
    );
};

export default Breadcrumbs;
