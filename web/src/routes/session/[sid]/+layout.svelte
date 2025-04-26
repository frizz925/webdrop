<script lang="ts">
	let { children } = $props();
</script>

<div>
	<div class="mx-auto max-w-xl sm:border-r sm:border-l">
		{@render children()}
	</div>
</div>
