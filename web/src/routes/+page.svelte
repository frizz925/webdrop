<script lang="ts">
	import { SID_KEY } from '$lib';
	import type { Session } from '$lib/models';
	import { sluggify } from '$lib/utils';

	let { sid, message } = $state({ sid: '', message: '' });

	const updateInput = (evt: Event) => {
		const el = evt.target as HTMLInputElement;
		sid = el.value.replaceAll('-', '');
		el.value = sluggify(sid);
	};

	const joinSession = async (evt: Event) => {
		const el = evt.target as HTMLInputElement;
		if (el.disabled) return;
		message = '';

		const res = await fetch(`/api/session/${sid}`, { method: 'HEAD' });
		if (res.status === 200) {
			window.location.assign(`/s/${sid}`);
		} else if (res.status === 400) {
			message = 'Invalid session ID';
		} else if (res.status === 404) {
			message = 'Session not found';
		} else {
			message = 'Unknown error';
		}
	};

	const createSession = async () => {
		message = '';

		const res = await fetch(`/api/session`, { method: 'POST' });
		if (res.status >= 400) {
			message = 'Unknown error';
			return;
		}

		const sess: Session = await res.json();
		localStorage.setItem(SID_KEY, sess.id.toString());
		window.location.assign(`/s/${sess.id}`);
	};
</script>

<div class="m-auto flex h-screen max-w-xl flex-col items-stretch justify-center">
	<div class="bg-color flex flex-col items-center">
		<h1 class="text-4xl font-bold">WebDrop</h1>
		<h2 class="mt-2 text-xl">Easily share files over the web</h2>
		<div class="mt-4 flex">
			<input
				type="text"
				class="h-12 w-56 rounded-l-full border-r-0 pl-6"
				placeholder="Session ID"
				oninput={updateInput}
			/>
			<button class="h-12 rounded-r-full px-4" disabled={sid.length <= 0} onclick={joinSession}>
				Join
			</button>
		</div>
		<div class="mt-4">
			<button class="cursor-pointer rounded-full px-4 py-2" onclick={createSession}>
				New session
			</button>
		</div>
		<div class="mt-4">{message}</div>
	</div>
</div>

<style>
	button {
		font-weight: var(--font-weight-semibold);
		background-color: var(--color-gray-800);
		color: var(--color-gray-50);

		@media (prefers-color-scheme: dark) {
			background-color: var(--color-gray-100);
			color: var(--color-gray-800);
		}
	}

	button:not(:disabled) {
		cursor: pointer;
	}

	button:disabled {
		background-color: var(--color-gray-300);

		@media (prefers-color-scheme: dark) {
			background-color: var(--color-gray-600);
		}
	}
</style>
