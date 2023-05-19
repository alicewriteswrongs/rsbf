import React from "react"
import { createRoot } from "react-dom/client"

import BrainfuckEnvironment from "./components/BrainfuckRepl"

const app = document.querySelector("#app")

if (app) {
  const root = createRoot(app)
  root.render(<App />)
}

function App() {
  return (
    <div>
      <BrainfuckEnvironment />
    </div>
  )
}
