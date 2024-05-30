import "./styles.css";
import App from "./app/layouts/App.svelte";

const app = new App({
  target: document.getElementById("app"),
});

export default app;
