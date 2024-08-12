import { toast } from 'svelte-sonner';
import api from './api';
import { _ } from 'svelte-i18n';

/** Helper for checking if objects are really equal */
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

/** Helper function for preventing the closing of modals if toast are clicked/interacted with */
export function onOutsideClick(event: Event) {
	const target = event.target as HTMLElement;
	if (target.closest('#toaster')) {
		event.preventDefault();
	}
}

/** Gets `T` of `api.Result<T>` if no error occurred otherwise displays the error via a toast and throws the error */
export function handle_result<T>(result: api.Result<T>): T | never {
	if (is_error(result)) {
		return show_error(result);
	} else {
		return result;
	}
}

/** Check if the `api.Result` is an `api.Error` */
function is_error(result: api.Result<any>): result is api.Error {
	const errors: string[] = Object.keys(api.Error);
	return errors.includes(result);
}

/** Display error on the frontend via a toast and throw the error */
function show_error(error: api.Error): never {
	let errorLocalized: string = '';
	_.subscribe((_) => (errorLocalized = _(error_msg(error))));
	toast.error(errorLocalized);

	throw error;
}

/** Server Error translations */
function error_msg(error: api.Error): string {
	switch (error) {
		case api.Error.Arguments:
			return '.error.input';
		case api.Error.FileOpen:
			return '.error.file-open';
		case api.Error.Network:
			return '.error.network';
		case api.Error.InvalidFormat:
			return '.error.format';
		case api.Error.NothingFound:
			return '.error.none';
		case api.Error.ReferencedUser:
			return '.user.referenced.del';
		case api.Error.ReferencedCategory:
			return '.category.not-empty.del';
		case api.Error.InvalidBook:
			return '.book.invalid';
		case api.Error.InvalidUser:
			return '.user.invalid';
		case api.Error.LendingUserMayNotBorrow:
			return '.error.lending.user';
		case api.Error.LendingBookNotBorrowable:
			return '.error.lending.book';
		case api.Error.LendingBookAlreadyBorrowed:
			return '.error.lending.already-borrowed';
		case api.Error.LendingBookAlreadyBorrowedByUser:
			return '.error.lending.already-borrowed-by';
		case api.Error.LendingBookNotBorrowed:
			return '.error.lending.not-borrowed';
		case api.Error.LendingBookAlreadyReserved:
			return '.error.lending.already-reserved';
		case api.Error.LendingBookNotReserved:
			return '.error.lending.not-reserved';
		case api.Error.UnsupportedProjectVersion:
			return '.error.update';
		default:
			return '.error.unknown';
	}
}

/** Replaces the placeholders in the mail templates */
export function mail_replace(
	template: api.MailTemplate,
	booktitle: string,
	username: string
): api.MailTemplate {
	return {
		subject: template.subject
			.replaceAll('{booktitle}', booktitle)
			.replaceAll('{username}', username),
		body: template.body.replaceAll('{booktitle}', booktitle).replaceAll('{username}', username)
	};
}
