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
<div class="modal-bg bg-black/60" class:hidden={!shown} onclick={() => (shown = false)}>
	<div class="mt-24 cursor-default px-4" onclick={(evt) => evt.stopPropagation()}>
		{@render children()}
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
	}
</style>
