
const Previewer = (props) => {
  const { content } = props

  return (
    <div style={{
      width: '100%',
      height: document.body.clientHeight - 125,
      overflow: 'auto',
    }}>
      <div dangerouslySetInnerHTML={{ __html: `<div>${content}<div>` }}></div>
    </div>
  )
}
export default (Previewer)