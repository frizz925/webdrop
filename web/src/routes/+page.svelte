<script lang="ts">
	import { createAuthKey, createMasterKey, encodeBuffer, encodeKDFParams } from '$lib/crypto';
	import type { Session } from '$lib/models';
	import { jsonRequest, sluggify } from '$lib/utils';
	import { onMount } from 'svelte';

	let { sid, message, creating, encryption, subtleAvailable } = $state({
		sid: '',
		message: '',
		creating: false,
		encryption: false,
		subtleAvailable: false
	});

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

	const createEncryptedSession = async () => {
		const crypto = window.crypto;

		const password = crypto.getRandomValues(new Uint8Array(24));
		const { masterKey, kdfParams } = await createMasterKey(password);
		const authKey = await createAuthKey(masterKey);

		return {
			password: encodeBuffer(password),
			response: await fetch(
				'/api/session/encrypted',
				jsonRequest('POST', {
					authKey: encodeBuffer(authKey),
					kdfParams: encodeKDFParams(kdfParams)
				})
			)
		};
	};

	const createSession = async () => {
		message = '';
		creating = true;

		let res, password;
		if (encryption) {
			const result = await createEncryptedSession();
			res = result.response;
			password = result.password;
		} else res = await fetch('/api/session', { method: 'POST' });

		if (!res.ok) {
			message = 'Unknown error';
			creating = false;
			return;
		}

		const sess: Session = await res.json();
		if (password) window.localStorage.setItem(`${sess.id}`, password);
		window.location.assign(`/session/${sess.id}`);
	};

	onMount(() => {
		subtleAvailable = 'subtle' in window.crypto;
		encryption = subtleAvailable;
	});
</script>

<div class="m-auto flex h-screen max-w-xl flex-col items-stretch justify-center">
	<div class="flex flex-col items-center">
		<h1 class="text-4xl font-bold">WebDrop</h1>
		{#if subtleAvailable}
			<h2 class="mt-2 text-xl">Easily share files over the web, securely</h2>
		{:else}
			<h2 class="mt-2 text-xl">Easily share files over the web</h2>
		{/if}
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
		<button
			class="btn mt-4 block cursor-pointer rounded-full px-4 py-2"
			disabled={creating}
			onclick={createSession}
		>
			New session
		</button>
		<!-- svelte-ignore a11y_click_events_have_key_events -->
		<!-- svelte-ignore a11y_no_static_element_interactions -->
		<div
			class="mt-4 flex cursor-pointer items-center justify-start"
			class:hidden={!subtleAvailable}
			onclick={() => (encryption = !encryption)}
		>
			<input type="checkbox" bind:checked={encryption} />
			<span class="ml-2">End-to-end encryption</span>
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
