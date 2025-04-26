<script lang="ts">
	import { page } from '$app/state';
	import * as QRCode from 'qrcode';

	interface Props {
		shown: boolean;
	}

	let canvas: HTMLCanvasElement;
	let { shown = $bindable() }: Props = $props();

	$effect(() => {
		QRCode.toCanvas(canvas, page.url.toString(), { scale: 10 });
	});
</script>

<!-- svelte-ignore a11y_no_static_element_interactions -->
<!-- svelte-ignore a11y_click_events_have_key_events -->
<div
	class="fixed top-0 left-0 z-50 flex h-screen w-screen cursor-pointer items-center justify-center bg-black/60"
	class:hidden={!shown}
	onclick={() => (shown = false)}
>
	<div class="cursor-default bg-white" onclick={(evt) => evt.stopPropagation()}>
		<canvas bind:this={canvas}></canvas>
	</div>
</div>
