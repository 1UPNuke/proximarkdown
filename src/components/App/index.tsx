import "./reset.css";
import "./style.css";
import ToolBar from "../Toolbar";
import Tree from "../Tree";
import Editor from "../Editor";
import Previewer from "../Previewer";
import Footer from "../Footer";
import Tabs from "../Tabs";
import Breadcrumbs from "../Breadcrumbs";

interface AppProps {
    tree? : boolean;
    previewer? : boolean;
}

const App = (props : AppProps) => {
    return (
        <>
            <ToolBar/>
            {props.tree && <Tree/>}
            <Tabs/>
            <Breadcrumbs path="Breadcrumbs"/>
            <Editor content=""/>
            {props.previewer && <Previewer/>}
            <Footer/>
        </>
    );
};

export default App;
