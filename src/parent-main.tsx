import React from "react";
import ReactDOM from "react-dom/client";
import App from "./App";
import "./App.css";
import "./style.css";

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <div className="col fullwidth">
      <App title="parent (greet works)"/>
      <iframe className="fullwidth"  title="preview" src="/"></iframe>
    </div>
  </React.StrictMode>
);


