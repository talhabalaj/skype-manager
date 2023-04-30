import { invoke } from "@tauri-apps/api";
import { createSignal, onMount } from "solid-js";
import SkypeInstance from "./SkypeInstance";

export default function SkypeList() {
  const [result, setResult] = createSignal<Array<string>>();

  onMount(async () => {
    const result = await invoke<string[]>("get_skypes");
    setResult(result);
  });

  return (
    <ul>
      <SkypeInstance path="default" />
      {!result() && "Loading!!"}
      {result()?.map((path) => (
        <SkypeInstance path={path} />
      ))}
    </ul>
  );
}
