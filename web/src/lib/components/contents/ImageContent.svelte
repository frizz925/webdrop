<script lang="ts">
	import { getFileUrl } from '$lib/utils';
	import type { Props } from './MediaContent';

	const { sid, object, content }: Props = $props();
	const src = getFileUrl(sid, object, content);
</script>

<a href={src} target="_blank">
	<img {src} alt={content.name} class="w-full" />
</a>
