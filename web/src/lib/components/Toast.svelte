<script lang="ts">
	import { onMount } from 'svelte';
	import { toastState } from './state.svelte';

	let lastMessage = toastState.message;
	let pendingTimer: NodeJS.Timeout | null = null;

	let el: HTMLDivElement;
	const hideToast = (mounting?: boolean) => {
		const rect = el.getBoundingClientRect();
		el.style.bottom = `-${rect.height}px`;

		if (mounting) return;

		if (pendingTimer) {
			clearTimeout(pendingTimer);
			pendingTimer = null;
		}

		setTimeout(() => {
			lastMessage = '';
			toastState.message = '';
		}, 300);
	};

	$effect(() => {
		if (lastMessage === toastState.message) return;

		el.style.bottom = '0px';
		lastMessage = toastState.message;

		if (pendingTimer) clearTimeout(pendingTimer);
		pendingTimer = setTimeout(() => hideToast(), 5000);
	});

	onMount(() => hideToast(true));
</script>

<!-- svelte-ignore a11y_no_static_element_interactions -->
<!-- svelte-ignore a11y_click_events_have_key_events -->
<div class="toast z-20 transition-all duration-300" bind:this={el}>
	<div
		class={[
			'z-10 m-8 rounded-full px-6 py-4 font-medium drop-shadow-lg',
			'bg-gray-700 text-gray-100 dark:bg-gray-100 dark:text-gray-800',
			'pointer-events-auto cursor-pointer'
		]}
		onclick={() => hideToast()}
	>
		{toastState.message}
	</div>
</div>

<style>
	.toast {
		display: flex;
		justify-content: start;
		align-items: center;
		flex-direction: column;

		width: 100%;
		position: fixed;
		bottom: -100%;
		left: 0;

		pointer-events: none;
	}
</style>
