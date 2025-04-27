<script lang="ts">
	export interface PartialProps {
		shown: boolean;
	}

	interface Props extends PartialProps {
		children: any;
	}

	let { shown = $bindable(), children }: Props = $props();
</script>

<!-- svelte-ignore a11y_no_static_element_interactions -->
<!-- svelte-ignore a11y_click_events_have_key_events -->
<div
	class="modal-bg cursor-pointer bg-black/60"
	class:invisible={!shown}
	onclick={() => (shown = false)}
>
	<div class="mt-24 mb-8 flex flex-col items-center justify-start px-4">
		<div class="contents cursor-default" onclick={(evt) => evt.stopPropagation()}>
			{@render children()}
		</div>
	</div>
</div>

<style>
	.modal-bg {
		position: fixed;
		top: 0;
		left: 0;
		width: 100%;
		height: 100%;
		z-index: 50;
		cursor: pointer;
		opacity: 1;
		overflow: hidden;
		transition: opacity 300ms;
	}

	.modal-bg.invisible {
		opacity: 0;
		transition:
			visibility 0s 300ms,
			opacity 300ms;
	}
</style>
