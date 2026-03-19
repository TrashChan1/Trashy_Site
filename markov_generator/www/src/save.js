export function setupSave(textElement, button) {
  const saveFile = () => {
    const textContent = textElement.innerHTML
    const blob = new Blob([textContent], { type: "text/plain" })
    const blobUrl = URL.createObjectURL(blob)
    const anchor = document.createElement("a")
    anchor.href = blobUrl
    anchor.download = "mark_genfile.txt"
    document.body.appendChild(anchor)
    anchor.click()
    document.body.removeChild(anchor)
    URL.revokeObjectURL(blobUrl)
  }
  button.addEventListener('click', () => saveFile())
}
