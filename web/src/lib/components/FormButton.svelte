<script lang="ts">
	import { FontAwesomeIcon } from '@fortawesome/svelte-fontawesome';

	let { icon } = $props();
</script>

<div class="flex h-10 w-10 cursor-pointer items-center justify-center">
	<FontAwesomeIcon {icon} />
</div>
