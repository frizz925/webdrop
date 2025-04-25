<script lang="ts">
	import type { FileContent } from '$lib/models';
	import { getFileUrl } from '$lib/utils';
	import Content, { type PartialProps } from './Content.svelte';

	interface Props extends PartialProps {
		content: FileContent;
	}

	const { sid, object: obj, content, onDelete }: Props = $props();
	const src = getFileUrl(sid, obj, content);
</script>

<Content {sid} object={obj} {onDelete}>
	<img {src} alt={content.name} class="w-full" />
</Content>
