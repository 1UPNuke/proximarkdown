import { useContext } from "preact/hooks";
import * as styles from "./style.module.css";
import { OpenedFiles } from "../App";

const Editor = () => {
    let content = useContext(OpenedFiles).openedFiles[0].content;
    return (
        <pre className={styles.editor}>
            <code contentEditable={true}>
                {content}
            </code>
        </pre>
    );
};

export default Editor;
