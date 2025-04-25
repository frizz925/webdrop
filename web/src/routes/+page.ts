import { browser } from '$app/environment';
import { SID_KEY } from '$lib';

export const load = () => {
	if (!browser) return {};
	const sid = localStorage.getItem(SID_KEY);
	if (sid) window.location.assign(`/s/${sid}`);
	return {};
};
