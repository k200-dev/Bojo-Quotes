/* eslint-disable no-unused-vars */
/* eslint-disable no-undef */
function randQuote() {
	const url = 'https://api.bojo-quotes.xyz/random';
	fetch(url).then(data => data.json()).then((json) => {
		document.getElementById('apiText').innerHTML = `${json.quote} - ${json.subject}`;
	});
}