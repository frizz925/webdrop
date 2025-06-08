<script lang="ts">
	import * as QRCode from 'qrcode';
	import Modal, { type PartialProps } from './Modal.svelte';

	interface Props extends PartialProps {
		text: string;
	}

	let canvas: HTMLCanvasElement;
	let { shown = $bindable(), text }: Props = $props();

	$effect(() => {
		QRCode.toCanvas(canvas, text, { scale: 10 });
	});
</script>

<Modal bind:shown>
	<canvas bind:this={canvas}></canvas>
	<button
		class="cursor-pointer p-4 text-lg text-gray-200 transition-colors hover:bg-black/20 dark:hover:bg-white/20"
		onclick={() => (shown = false)}
	>
		Close
	</button>
</Modal>
