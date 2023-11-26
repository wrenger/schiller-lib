export function get(which: string) {
	const htmlElement = document.documentElement;
	return htmlElement.getAttribute(which) as string;
}

export function set(which: string, what: string) {
	const htmlElement = document.documentElement;
	htmlElement.setAttribute(which, what);
	localStorage.setItem(which, what);
}

export function areObjectsEqual(obj1: any, obj2: any): boolean {
	const keys1 = Object.keys(obj1);
	const keys2 = Object.keys(obj2);

	if (keys1.length !== keys2.length) {
		return false;
	}

	for (const key of keys1) {
		if (obj1[key] !== obj2[key]) {
			return false;
		}
	}

	return true;
}
