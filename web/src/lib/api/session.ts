import { sessionFromDto, type SessionDto } from '$lib/models';
import type { Fetch } from '$lib/types';

export const getSession = async (fetch: Fetch, sid: string) => {
	const res = await fetch(`/api/session/${sid}`);
	if (res.status >= 400) throw res;
	const dto: SessionDto = await res.json();
	return sessionFromDto(dto);
};
