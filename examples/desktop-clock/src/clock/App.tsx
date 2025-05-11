import { useEffect, useState } from "react";
import "./App.css";

function App() {
  const [time, setTime] = useState(new Date());

  useEffect(() => {
    const interval = setInterval(() => {
      setTime(new Date());
    }, 1000);
    return () => clearInterval(interval);
  }, []);

  return (
    <div className="container">
      <div className="title">{time.toLocaleTimeString()}</div>
      <div className="subtitle">{time.toLocaleDateString()}</div>
    </div>
  );
}

export default App;
