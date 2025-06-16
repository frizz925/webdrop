<script lang="ts">
	import type { TextContent } from '$lib/models';
	import { faClipboard } from '@fortawesome/free-solid-svg-icons';
	import Content, { type PartialProps } from './Content.svelte';
	import { copyToClipboard } from './utils';

	interface Props extends PartialProps {
		content: TextContent;
	}

	const { object, content, onDelete }: Props = $props();
	const { isSecret } = content;

	let { secretShown } = $state({ secretShown: false });
	const copyMenu = {
		label: 'Copy Text',
		icon: faClipboard,
		onClick: () => copyToClipboard(content.data, 'Text')
	};
</script>

<Content {object} {copyMenu} {onDelete}>
	<div class="overflow-hidden px-4 pt-4 wrap-anywhere whitespace-pre-wrap">
		{#if isSecret}
			<!-- svelte-ignore a11y_click_events_have_key_events -->
			<!-- svelte-ignore a11y_no_static_element_interactions -->
			<span
				class={[
					'cursor-pointer bg-gray-500 py-1 select-none',
					secretShown ? 'text-gray-50' : 'text-transparent'
				]}
				onclick={() => (secretShown = !secretShown)}
			>
				{content.data}
			</span>
		{:else}
			{content.data}
		{/if}
	</div>
</Content>
