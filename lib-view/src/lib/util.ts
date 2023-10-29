export function get(which: string) {
	const htmlElement = document.documentElement;
	return htmlElement.getAttribute(which) as string;
}

export function set(which: string, what: string) {
	const htmlElement = document.documentElement;
	htmlElement.setAttribute(which, what);
	localStorage.setItem(which, what);
}
