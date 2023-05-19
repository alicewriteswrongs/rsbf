import React from 'react';
import { createRoot } from 'react-dom/client';

import BrainfuckEnvironment from './components/BrainfuckInterpreter';

const app = document.querySelector("#app");

if (app) {
  const root = createRoot(app);
  root.render(<App /> );
} else {
  console.log('did not find');
}

function App () {
  return <div>
    <BrainfuckEnvironment />
  </div>
}
