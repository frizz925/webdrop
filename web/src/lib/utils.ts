import type { FileContent, FileObject, SessionID } from './models';

const SLUG_PART_LENGTH = 4;

export const sluggify = (s: string) => {
	const parts = [];

	let idx = 0;
	while (idx < s.length - SLUG_PART_LENGTH) {
		const [start, end] = [idx, idx + SLUG_PART_LENGTH];
		parts.push(s.substring(start, end));
		idx = end;
	}
	parts.push(s.substring(idx));

	return parts.join('-');
};

export const jsonRequest = <T>(method: string, data: T) =>
	({
		method,
		headers: {
			'Content-Type': 'application/json'
		},
		body: JSON.stringify(data)
	}) as RequestInit;

export const getFileUrl = (sid: SessionID, obj: FileObject, content: FileContent) =>
	`/objects/${sid}/${obj.id}/${content.name}`;
