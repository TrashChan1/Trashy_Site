import init, { greet, spit_out } from '../pkg/markov_generator.js'
export function setupGenerator(inElement, outElement, buttonElement, countElement) {
  outElement.innerHTML = "generated text will appear here"
  const setOutput = () => {
    let genText = spit_out(inElement.value, countElement.value)
    outElement.innerHTML = `${genText}`
  }
  buttonElement.addEventListener('click', () => 
	  setOutput()
  )
}
