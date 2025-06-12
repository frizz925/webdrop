<script lang="ts">
	import type { Session } from '$lib/models';
	import { sluggify } from '$lib/utils';

	let { sid, message, creating } = $state({ sid: '', message: '', creating: false });

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
			window.location.assign(`/session/${sid}`);
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
		creating = true;

		const res = await fetch(`/api/session`, { method: 'POST' });
		if (res.status >= 400) {
			message = 'Unknown error';
			creating = false;
			return;
		}

		const sess: Session = await res.json();
		window.location.assign(`/session/${sess.id}`);
	};
</script>

<div class="m-auto flex h-screen max-w-xl flex-col items-stretch justify-center">
	<div class="bg-color flex flex-col items-center">
		<h1 class="text-4xl font-bold">WebDrop</h1>
		<h2 class="mt-2 text-xl">Easily share files over the web</h2>
		<form class="mt-4 flex" onsubmit={joinSession}>
			<input
				type="text"
				class="h-12 w-56 rounded-l-full border-r-0 pl-6"
				placeholder="Session ID"
				oninput={updateInput}
			/>
			<input
				type="submit"
				class="btn h-12 rounded-r-full px-4"
				disabled={sid.length <= 0}
				value="Join"
			/>
		</form>
		<div class="mt-4">
			<button
				class="btn cursor-pointer rounded-full px-4 py-2"
				disabled={creating}
				onclick={createSession}
			>
				New session
			</button>
		</div>
		<div class="mt-4">{message}</div>
	</div>
</div>

<style>
	.btn {
		font-weight: var(--font-weight-semibold);
		background-color: var(--color-gray-800);
		color: var(--color-gray-50);

		@media (prefers-color-scheme: dark) {
			background-color: var(--color-gray-100);
			color: var(--color-gray-800);
		}
	}

	.btn:not(:disabled) {
		cursor: pointer;
	}

	.btn:disabled {
		background-color: var(--color-gray-300);

		@media (prefers-color-scheme: dark) {
			background-color: var(--color-gray-600);
		}
	}
</style>
