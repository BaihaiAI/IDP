import Ansi from "ansi-to-react"
import { useEffect } from "react"

interface Props {
  value: string
  height: string
}

export const Output: React.FC<Props> = (props: Props) => {
  const { value, height } = props
  useEffect(() => {
    if (value) {
      document.getElementById('output-bottom').scrollIntoView(true)
    }
  }, [value])
  return (
    <div style={{ borderTop: '1px solid lightgray', height: height, overflow: 'scroll', padding: '5px', whiteSpace: 'pre-line' }}>
      <Ansi>{value}</Ansi>
      <div id='output-bottom'></div>
    </div>
  )
}