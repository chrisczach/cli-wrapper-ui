import { useState } from "react";
import { promisified } from "tauri/api/tauri";
import "./App.css";

function App() {
    const cmd = "runCommand"
    const [command, setCommand] = useState("");
    const [args, setArgs] = useState("");
    const execute = () =>
        promisified({
            cmd,
            command: `${command} ${args}`,
        })
            .then(({ value, message }: any) => console.log({ value, message }))
            .catch((err) => console.log(err));

    return (
        <div className="App">
            <header className="App-header">
                <input
                    type="text"
                    value={command}
                    onChange={({ target: { value } }) => setCommand(value)}
                ></input>
                <input
                    type="text"
                    value={args}
                    onChange={({ target: { value } }) => setArgs(value)}
                ></input>
                <button onClick={execute}>Execute</button>
            </header>
        </div>
    );
}

export default App;
