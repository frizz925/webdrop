<script lang="ts">
	import { type FileContent } from '$lib/models';
	import LinkContent from './LinkContent.svelte';
	import type { Props } from './MediaContent';

	const { object: obj, content, getFileUrl, onDelete }: Props = $props();
	const src = getFileUrl(obj, content);
</script>

<div class="px-4 pt-4">
	<audio controls {src} class="w-full"></audio>
</div>
<LinkContent link={src} object={obj} content={obj.content as FileContent} {onDelete} download />
