<script lang="ts">
	import { getFileUrl } from '$lib/utils';
	import type { Props } from './MediaContent';

	const { sid, object, content }: Props = $props();
	const src = getFileUrl(sid, object, content);
</script>

<div class="px-4 pt-4">
	<audio controls {src} class="w-full"></audio>
</div>
