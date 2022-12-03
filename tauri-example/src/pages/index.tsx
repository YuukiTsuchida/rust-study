import { useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import Image from "next/image";
import reactLogo from "../assets/react.svg";
import tauriLogo from "../assets/tauri.svg";
import nextLogo from "../assets/next.svg";

import { emit, listen } from '@tauri-apps/api/event';
import { appWindow } from "@tauri-apps/api/window";
// import * as win from "@tauri-apps/api/window";


function App() {
  const [greetMsg, setGreetMsg] = useState("");
  const [name, setName] = useState("");

  async function greet() {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    setGreetMsg(await invoke("greet", { name }));
    test();

    let t = await listen("test-event", event => {
      console.log(event);
    });

    // await listen("window-event", event => {
    //     console.log(`window event: ${event}`);
    // });
  }

  async function test() {
    const str = await invoke("test", { name: "test", age: 35 });
    console.log(str);
  }

  async function resultTest(functionName) {
    invoke(functionName)
      .then(message => console.log(message))
      .catch(error => console.error(error));

    try {
      const result = await invoke(functionName);
      console.log(result);
    } catch (e) {
      console.error(e);
    }
  }

  async function callAsyncFunction() {
    const t = await invoke("async_command");
    console.log(t)
  }

  async function passWindow() {
    invoke("receive_window_object");
  }

  async function passAppHandler() {
    invoke("receive_app_object");
  }

  async function callRustState() {
    await invoke("use_state");

    // console.log(win);
    // console.log(appWindow);
    // WebviewWindow.getByLabel("main");
    // emit("test-event", {message: "emit"});
    // await appWindow.emit('window-event', { message: "call window event" });
  }  

  return <div className="container">
      <h1>Welcome to Tauri!</h1>

      <div className="row">
        <span className="logos">
          <a href="https://nextjs.org" target="_blank">
            <Image width={144} height={144} src={nextLogo} className="logo next" alt="Next logo" />
          </a>
        </span>
        <span className="logos">
          <a href="https://tauri.app" target="_blank">
            <Image width={144} height={144} src={tauriLogo} className="logo tauri" alt="Tauri logo" />
          </a>
        </span>
        <span className="logos">
          <a href="https://reactjs.org" target="_blank">
            <Image width={144} height={144} src={reactLogo} className="logo react" alt="React logo" />
          </a>
        </span>
      </div>

      <p>Click on the Tauri, Next, and React logos to learn more.</p>

      <div className="row">
        <div>
          <input id="greet-input" onChange={e => setName(e.currentTarget.value)} placeholder="Enter a name..." />
          <button type="button" onClick={() => greet()}>
            Greet
          </button>
        </div>
      </div>
      <button type="button" onClick={() => resultTest("test_return_result_err")}>
        result fail
      </button>
      <button type="button" onClick={() => resultTest("test_return_result_ok")}>
        result success
      </button>
      <button type="button" onClick={() => callAsyncFunction()}>
        async function
      </button>
      <button type="button" onClick={() => passWindow()}>
        pass window
      </button>
      <button type="button" onClick={() => callRustState()}>
        use state
      </button>
      <button type="button" onClick={() => passAppHandler()}>
        pass AppHandler
      </button>
      <p>
        {greetMsg}
      </p>
    </div>;
}

export default App;
