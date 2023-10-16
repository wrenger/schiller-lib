export async function request(
	url: string,
	type: string,
	json: BodyInit | null | undefined
): Promise<any> {
	const response = await fetch(url, {
		method: type,
		body: json
	});

	let data = await response.json();

	if (response.status === 200) {
		return data;
	} else {
		throw data;
	}
}

export function get(which: string) {
	const htmlElement = document.documentElement;
	return htmlElement.getAttribute(which) as string;
}

export function set(which: string, what: string) {
	const htmlElement = document.documentElement;
	htmlElement.setAttribute(which, what);
	localStorage.setItem(which, what);
}
