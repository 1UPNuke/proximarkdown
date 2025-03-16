import * as styles from "./style.module.css";

const Breadcrumbs = (props : {path : string}) => {
    return (
        <nav className={styles.crumbs}>
            <span>{props.path}</span>
        </nav>
    );
};

export default Breadcrumbs;
