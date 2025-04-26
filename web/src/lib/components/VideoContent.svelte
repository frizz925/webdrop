<script lang="ts">
	import type { FileContent } from '$lib/models';
	import { getFileUrl } from '$lib/utils';
	import { type PartialProps } from './Content.svelte';
	import MediaContent from './MediaContent.svelte';

	interface Props extends PartialProps {
		content: FileContent;
	}

	const { sid, object, content, onDelete }: Props = $props();
	const src = getFileUrl(sid, object, content);
</script>

<MediaContent {sid} {object} {src} name={content.name} {onDelete}>
	<video controls>
		<source {src} />
		<track kind="captions" />
	</video>
</MediaContent>
