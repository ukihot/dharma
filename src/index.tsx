/* @refresh reload */
import { render } from "solid-js/web";

import "./style.css";
import Dharma from "./Dharma";

render(() => <Dharma />, document.getElementById("root") as HTMLElement);
