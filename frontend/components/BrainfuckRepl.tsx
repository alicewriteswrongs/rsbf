import React, { useCallback, useState } from "react"
import { createUseStyles } from "react-jss"
import { BF_HELLO_WORLD } from "../util/constants"

const useStyles = createUseStyles({
  terminal: {
    backgroundColor: "black",
    minHeight: "60em",
    maxWidth: "80em",
    color: "white",
    margin: "auto",
  },
  terminalRow: {
    display: "flex",
    flexDirection: "row",
  },
  terminalInput: {
    background: "none",
    color: "white",
    border: "none",
    width: "100%",

    "&:focus-visible": {
      border: "none",
    },
  },
  terminalCaret: {
    color: "white",
  },
})

export default function BrainfuckEnvironment() {
  const [bfCode, setBfCode] = useState(BF_HELLO_WORLD)
  const [output, setOutput] = useState("")

  const classes = useStyles()

  const runBrainfuck = useCallback(() => {
    if (bfCode !== "") {
      const outputBuffer: number[] = []
      const addByteToBuffer = (byte: number) => {
        outputBuffer.push(byte)
      }
      // @ts-ignore
      window.run_brainfuck(bfCode, addByteToBuffer)
      setOutput(outputBuffer.map((code) => String.fromCodePoint(code)).join(""))
    }
  }, [setOutput, bfCode])

  return (
    <div>
      <textarea
        value={bfCode}
        onChange={(e) => {
          e.preventDefault()
          setBfCode(e.target.value)
        }}
      />
      <div>
        <button onClick={runBrainfuck}>run!</button>
        <code>
          <pre className={classes.terminal}>
            <div className={classes.terminalRow}>
              <span className={classes.terminalCaret}>{" > "}</span>
              <input className={classes.terminalInput} />
            </div>
            {output}
          </pre>
        </code>
      </div>
    </div>
  )
}
