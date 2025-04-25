import { browser } from '$app/environment';
import { SID_KEY } from '$lib';
import type { PageLoad } from './$types';

export const load: PageLoad = async ({ fetch }) => {
	if (!browser) return {};
	const sid = localStorage.getItem(SID_KEY);
	if (!sid) return {};
	const res = await fetch(`/api/session/${sid}`, { method: 'HEAD' });
	if (res.status === 200) window.location.assign(`/s/${sid}`);
	else localStorage.removeItem(SID_KEY);
	return {};
};
