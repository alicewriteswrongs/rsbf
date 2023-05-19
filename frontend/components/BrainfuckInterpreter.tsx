import React, {useCallback, useState} from 'react';

export default function BrainfuckEnvironment() {
  const [bfCode, setBfCode] = useState("");
  const [output, setOutput] = useState("");

  const runBrainfuck = useCallback(() => {
    const outputBuffer: number[] = [];
    const addByteToBuffer = (byte: number) => {
      outputBuffer.push(byte);
    }
    // @ts-ignore
    window.run_brainfuck(bfCode, addByteToBuffer);
    setOutput(outputBuffer.map(code => (
      String.fromCodePoint(code)
    )).join(""));
  }, [
    setOutput, bfCode, 
  ]);

  return <div>
    <textarea value={bfCode} onChange={e => {
      e.preventDefault();
      setBfCode(
      e.target.value
      );
    }} />
    <div>
    <button onClick={runBrainfuck}>run!</button>
    <h1>output</h1>
    <code><pre>{ output }</pre></code>
    </div>
  </div>
}
