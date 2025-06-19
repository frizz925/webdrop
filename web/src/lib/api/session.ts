import { authorizedRequest } from '$lib/crypto';
import { type FileObject, type Session } from '$lib/models';
import type { Fetch } from '$lib/types';

export const getSession = async (sid: string, fetch?: Fetch) => {
	fetch = fetch || window.fetch;
	const res = await fetch(`/api/session/${sid}`);
	if (!res.ok) throw res;
	return (await res.json()) as Session;
};

export const getObjects = async (sid: string, fetch?: Fetch) => {
	fetch = fetch || window.fetch;
	const res = await fetch(`/api/session/${sid}/objects`, authorizedRequest());
	if (!res.ok) throw res;
	return (await res.json()) as FileObject[];
};
