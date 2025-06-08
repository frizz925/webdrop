<script lang="ts">
	import type { Snippet } from 'svelte';
	import Modal, { type PartialProps } from './Modal.svelte';

	interface Props extends PartialProps {
		children: Snippet;
	}

	let { shown = $bindable(), children }: Props = $props();
</script>

<Modal bind:shown>
	<div class="bg-color mx-auto w-full max-w-md p-4">
		{@render children()}
	</div>
</Modal>
