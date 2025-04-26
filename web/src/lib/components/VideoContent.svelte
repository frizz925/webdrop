<script lang="ts">
	import { getFileUrl } from '$lib/utils';
	import type { Props } from './MediaContent';

	const { sid, object, content }: Props = $props();
	const src = getFileUrl(sid, object, content);
</script>

<video controls>
	<source {src} />
	<track kind="captions" />
</video>
