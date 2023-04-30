/* @refresh reload */
import { render } from "solid-js/web";

import App from "./App";

import { appWindow } from "@tauri-apps/api/window";
document
  .getElementById("titlebar-minimize")
  ?.addEventListener("click", () => appWindow.minimize());
document
  .getElementById("titlebar-maximize")
  ?.addEventListener("click", () => appWindow.toggleMaximize());
document
  .getElementById("titlebar-close")
  ?.addEventListener("click", () => appWindow.close());

render(() => <App />, document.getElementById("root") as HTMLElement);
