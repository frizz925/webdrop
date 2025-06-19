<script lang="ts">
	import { type FileContent } from '$lib/models';
	import { getFileURL } from '$lib/utils';

	import LinkContent from './LinkContent.svelte';
	import type { Props } from './MediaContent';

	const { sid, object: obj, content, onDelete }: Props = $props();
	const src = getFileURL(sid, obj, content);
</script>

<video controls>
	<source {src} />
	<track kind="captions" />
</video>
<LinkContent
	{sid}
	link={src}
	object={obj}
	content={obj.content as FileContent}
	{onDelete}
	download
/>
