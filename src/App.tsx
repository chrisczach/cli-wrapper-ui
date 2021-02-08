import { useState } from "react";
import { promisified } from "tauri/api/tauri";
import "./App.css";

function App() {
    const [inputValue, setInputValue] = useState("");
    const echoInput = () =>
        promisified({
            cmd: "runCommand",
            args: ['command', 'option', inputValue],
        })
            .then(({ value, message }: any) => console.log({ value, message }))
            .catch((err) => console.log(err));

    return (
        <div className="App">
            <header className="App-header">
                <input
                    type="text"
                    value={inputValue}
                    onChange={({ target: { value } }) => setInputValue(value)}
                ></input>
                <button onClick={echoInput}>Echo</button>
            </header>
        </div>
    );
}

export default App;
