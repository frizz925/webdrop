<script lang="ts">
	import { type FileContent } from '$lib/models';
	import LinkContent from './LinkContent.svelte';
	import type { Props } from './MediaContent';

	const { object: obj, content, getFileUrl, onDelete }: Props = $props();
	const src = getFileUrl(obj, content);
</script>

<video controls>
	<source {src} />
	<track kind="captions" />
</video>
<LinkContent link={src} object={obj} content={obj.content as FileContent} {onDelete} download />
