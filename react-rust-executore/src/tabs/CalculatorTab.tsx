import { useRef, useState } from "react";
import { evaluate_expression } from "../pkg/rust_expresion_analyser";
const CalculatorTab = () => {
  const inputRef = useRef("");
  const [calculatedValue, setCalculatedValue] = useState<string | null>(null);
  const [error, setError] = useState(null);

  const calculate = (e) => {
    e.preventDefault();
    try {
      const result = evaluate_expression(inputRef.current.value);
      setCalculatedValue(result);
      setError(null);
    } catch (error) {
      setError("Invalid Expression");
      setCalculatedValue(null);
    }
  };

  return (
    <div>
      <form
        onSubmit={calculate}
        style={{ display: "flex", flexDirection: "column", gap: "1rem" }}
      >
        <input ref={inputRef} />
        <button type="submit">Calculate</button>
      </form>
      <div>{calculatedValue ?? <h1>{calculatedValue}</h1>}</div>
      {error ?? <h3>{error}</h3>}
    </div>
  );
};

export default CalculatorTab;
