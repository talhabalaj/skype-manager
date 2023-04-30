import { invoke } from "@tauri-apps/api";
import { createSignal, onMount } from "solid-js";

export default function SkypeInstance(props: { path: string }) {
  const { path } = props;
  const [isSkypeRunning, setIsSkypeRunning] = createSignal<boolean>();

  async function startSkype(path: string) {
    await invoke<string[]>("launch_skype", { dataPath: path });
    setIsSkypeRunning(true)
  }

  async function stopSkype(path: string) {
    await invoke("stop_skype", { dataPath: path });
    setIsSkypeRunning(false)
  }

  onMount(async () => {
    setIsSkypeRunning(
      await invoke<boolean>("is_skype_running", { dataPath: path })
    );
  });

  return (
    <div
      style={{
        display: "flex",
        "justify-content": "space-between",
        "align-items": "center",
        padding: "20px",
      }}
    >
      <li>{path}</li>
      {typeof isSkypeRunning() === "undefined" && "Loading"}
      {!isSkypeRunning() ? (
        <button class="tui-button" onClick={() => startSkype(path)}>launch</button>
      ) : (
        <button class="tui-button" onClick={() => stopSkype(path)}>stop</button>
      )}
    </div>
  );
}
