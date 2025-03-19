import { Context, createContext } from "preact";
import "./reset.css";
import "./style.css";
import ToolBar from "../Toolbar";
import Tree from "../Tree";
import Editor from "../Editor";
import Previewer from "../Previewer";
import Footer from "../Footer";
import Tabs from "../Tabs";
import Breadcrumbs from "../Breadcrumbs";
import { listen } from "@tauri-apps/api/event";
import { Dispatch, StateUpdater, useState } from "preact/hooks";
import MarkdownFile from "../../classes/MarkdownFile";

export const OpenedFiles = createContext<{
    openedFiles: MarkdownFile[],
    setOpenedFiles: Dispatch<StateUpdater<MarkdownFile[]>> | ((..._: any)=>void)
}>({openedFiles: [], setOpenedFiles: (..._: any)=>{}});

const App = () => {
    const [openedFiles, setOpenedFiles] = useState<MarkdownFile[]>([new MarkdownFile()]);

    listen<MarkdownFile>("open-file", (event) => {
        console.log(event);
        let openedFile = new MarkdownFile(
            event.payload.name,
            event.payload.path,
            event.payload.modified,
            event.payload.content
        );
        setOpenedFiles([openedFile, ...openedFiles])
    })

    return (
        <>
            <Tree/>
            <ToolBar/>
            <OpenedFiles.Provider value={{openedFiles, setOpenedFiles}}>
                <Tabs/>
                <Breadcrumbs/>
                <Editor/>
                <Previewer/>
                <Footer/>
            </OpenedFiles.Provider>
        </>
    );
};

export default App;
