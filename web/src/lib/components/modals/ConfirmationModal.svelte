<script lang="ts">
	import Modal, { type PartialProps } from './Modal.svelte';

	interface Props extends PartialProps {
		onConfirm: () => void;
	}

	let { shown = $bindable(), onConfirm }: Props = $props();
</script>

<Modal bind:shown>
	<div class="bg-color mx-auto w-full max-w-md p-4">
		<div class="text-xl font-bold">Session termination</div>
		<div class="mt-4">
			<div>Do you want to terminate the session?</div>
			<div class="font-semibold text-red-400 italic">Your uploaded files will be deleted!</div>
		</div>
		<div class="mt-8 text-right">
			<button
				class="cursor-pointer rounded-md p-2 transition-colors hover:bg-gray-500/20"
				onclick={() => (shown = false)}>Cancel</button
			>
			<button class="cursor-pointer rounded-md bg-red-400 p-2 text-gray-50" onclick={onConfirm}>
				Delete
			</button>
		</div>
	</div>
</Modal>
