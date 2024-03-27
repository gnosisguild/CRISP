import { useEffect } from "react";

import "./App.css";
import init, { greet } from "../../rust/pkg/rfv";

function App() {
  useEffect(() => {
    init().then(() => {
      greet("World");
    });
  }, []);

  return (
    <div className='App'>
      <h1>Hello React & Rust WebAssembly</h1>
    </div>
  );
}

export default App;
