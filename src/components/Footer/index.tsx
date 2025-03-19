import { useContext } from "preact/hooks";
import * as styles from "./style.module.css";
import { OpenedFiles } from "../App";

const Footer = () => {

    let content = useContext(OpenedFiles).openedFiles[0].content;
    return (
        <footer className={styles.footer}>
            <span>Words: {
                // Trim all non-word characters for counting
                content.replace(/\W/gi, '')
                    // Split on whitespace
                    .trim().split(/\s+/).length
            }</span>
            <span>Characters: {
                content.length
            }</span>
            <span>Ln {
                //TODO
            }, Col {
                //TODO
            }</span>
        </footer>
    );
};

export default Footer;
