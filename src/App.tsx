import { createSignal, onMount } from "solid-js";
import logo from "./assets/logo.svg";
import { invoke } from "@tauri-apps/api/tauri";
import SkypeList from "./SkypeList";

function App() {
  const [hasSkype, setHasSkype] = createSignal<boolean>();

  onMount(async () => {
    const hasSkype = await invoke<boolean>("has_skype");
    setHasSkype(hasSkype);
  });

  return (
    <div>
      {!hasSkype() && <h1>You don't have skype installed!</h1>}
      {hasSkype() && (
        <>
          <h1>My Skype(s)</h1>
          <SkypeList />
        </>
      )}
    </div>
  );
}

export default App;
