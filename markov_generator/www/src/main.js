import { setupGenerator } from './generate.js'
import { setupSave } from './save.js'
import init from '/pkg/markov_generator.js'

await init()

setupGenerator(
	document.querySelector('#generator_input'),
	document.querySelector('#output'), 
	document.querySelector('#genButton'),
	document.querySelector('#quantity')
)
setupSave(
	document.querySelector('#output'),
	document.querySelector('#saveBtn')
)
