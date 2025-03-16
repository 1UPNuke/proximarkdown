import * as styles from "./style.module.css";

const Editor = (props : {content : string}) => {
    return (
        <pre className={styles.editor}>
            <code contentEditable={true}>
                {props.content}
            </code>
        </pre>
    );
};

export default Editor;
