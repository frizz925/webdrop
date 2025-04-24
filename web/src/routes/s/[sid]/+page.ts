import { getSession } from '$lib/api/session';
import { redirect } from '@sveltejs/kit';
import type { PageLoad } from './$types';

export const load: PageLoad = async ({ fetch, params }) => {
	const { sid } = params;
	try {
		const session = await getSession(fetch, sid);
		return { session };
	} catch (err) {
		const res = err as Response;
		if (res.status === 404) return redirect(303, '/');
		else throw res;
	}
};
