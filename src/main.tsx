import { render } from "preact";
import App from "./components/App";
import "./themes/testing.css";


render(<App tree previewer />, document.getElementById('root')!);
