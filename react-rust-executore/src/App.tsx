import { useEffect, useState } from "react";
import init, { evaluate_expression } from "./pkg/rust_expresion_analyser";
import "./App.css";
import CalculatorTab from "./tabs/CalculatorTab";
import ShaderViewer from "./tabs/ShaderViewer";

function App() {
  const [wasmLoaded, setWasmLoaded] = useState(false);

  useEffect(() => {
    const loadWasm = async () => {
      // Initialize the WASM module
      await init();
      setWasmLoaded(true);
    };

    loadWasm();
  }, []);

  return (
    <>
      {wasmLoaded && <CalculatorTab />}
      <ShaderViewer />
    </>
  );
}

export default App;
