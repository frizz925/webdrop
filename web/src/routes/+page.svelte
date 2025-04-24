<script lang="ts">
	import { sluggify } from '$lib/utils';

	let sid = $state('');

	const updateInput = (evt: Event) => {
		const el = evt.target as HTMLInputElement;
		sid = el.value.replaceAll('-', '');
		el.value = sluggify(sid);
	};

	const joinSession = () => {
		window.location.assign(`/s/${sid}`);
	};
</script>

<div class="m-auto flex h-screen max-w-xl flex-col items-stretch justify-center">
	<div class="bg-color flex flex-col items-center">
		<h1 class="text-4xl font-bold">WebDrop</h1>
		<h2 class="text-xl">Easily share files over the web</h2>
		<div class="mt-4 flex">
			<input
				type="text"
				class="h-12 w-56 border-r-0"
				placeholder="Session ID"
				oninput={updateInput}
			/>
			<button
				class="h-12 cursor-pointer bg-gray-800 px-4 text-gray-100 dark:bg-gray-100 dark:text-gray-800"
				onclick={joinSession}
			>
				Join
			</button>
		</div>
		<div class="mt-2">
			<button class="cursor-pointer">Create new session</button>
		</div>
	</div>
</div>
