const SLUG_PART_LENGTH = 4;

export function sluggify(s: string): string {
	const parts = [];

	let idx = 0;
	while (idx < s.length - SLUG_PART_LENGTH) {
		const [start, end] = [idx, idx + SLUG_PART_LENGTH];
		parts.push(s.substring(start, end));
		idx = end;
	}
	parts.push(s.substring(idx));

	return parts.join('-');
}

export function jsonRequest<T>(method: string, data: T): RequestInit {
	return {
		method,
		headers: {
			'Content-Type': 'application/json'
		},
		body: JSON.stringify(data)
	};
}
