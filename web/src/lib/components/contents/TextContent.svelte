<script lang="ts">
	import type { TextContent } from '$lib/models';
	import Content, { type PartialProps } from './Content.svelte';

	interface Props extends PartialProps {
		content: TextContent;
	}

	const { sid, object, content, onDelete }: Props = $props();
</script>

<Content {sid} {object} {onDelete}>
	<div class="px-4 pt-4 whitespace-pre-wrap">{content.data}</div>
</Content>
