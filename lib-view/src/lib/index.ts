export function areObjectsEqual(obj1: any, obj2: any): boolean {
	if (typeof obj1 !== 'object' || typeof obj2 !== 'object' || obj1 === null || obj2 === null) {
		return obj1 === obj2;
	}

	const keys1 = Object.keys(obj1);
	const keys2 = Object.keys(obj2);

	if (keys1.length !== keys2.length) {
		return false;
	}

	for (const key of keys1) {
		if (!areObjectsEqual(obj1[key], obj2[key])) {
			return false;
		}
	}

	return true;
}

export function onOutsideClick(event: Event) {
	const target = event.target as HTMLElement;
	if (target.closest('#toaster')) {
		event.preventDefault();
	}
}
