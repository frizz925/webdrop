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
	<div class="flex items-center justify-center">
		<canvas bind:this={canvas}></canvas>
	</div>
</Modal>
