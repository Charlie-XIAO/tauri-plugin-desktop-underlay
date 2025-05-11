import { toggleDesktopUnderlay } from "tauri-plugin-desktop-underlay-api";
import { exit } from "@tauri-apps/plugin-process";
import { useState } from "react";
import "./App.css";

function App() {
  const [isUnderlay, setIsUnderlay] = useState(true);

  const toggleMain = async () => {
    const isUnderlay = await toggleDesktopUnderlay("clock");
    setIsUnderlay(isUnderlay);
  };

  return (
    <div className="container">
      <div className="header">Clock: {isUnderlay ? "underlay" : "normal"}</div>
      <div className="buttons">
        <button onClick={toggleMain}>Toggle</button>
        <button onClick={() => exit()}>Exit</button>
      </div>
    </div>
  );
}

export default App;
