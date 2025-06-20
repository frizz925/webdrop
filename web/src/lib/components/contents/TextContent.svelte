<script lang="ts">
	import type { TextContent } from '$lib/models';
	import { faCopy } from '@fortawesome/free-solid-svg-icons';

	import { copyToClipboard } from '../utils';
	import Content, { type PartialProps } from './Content.svelte';

	interface Props extends PartialProps {
		content: TextContent;
	}

	const { object, content, onDelete }: Props = $props();
	const { isSecret } = content;

	let { secretShown } = $state({ secretShown: false });
	const copyMenu = {
		label: 'Copy Text',
		icon: faCopy,
		onClick: () => copyToClipboard(content.data, 'Text')
	};
</script>

<Content {object} {copyMenu} {onDelete}>
	<div class="wrap-anywhere overflow-hidden whitespace-pre-wrap px-4 pt-4">
		{#if isSecret}
			<!-- svelte-ignore a11y_click_events_have_key_events -->
			<!-- svelte-ignore a11y_no_static_element_interactions -->
			<span
				class={[
					'cursor-pointer select-none bg-gray-500',
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
