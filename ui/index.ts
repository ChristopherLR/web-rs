
function app() {
	const element = document.createElement('div');
	element.innerHTML = "Test App6";
	return element;
}

document.body.appendChild(app());
