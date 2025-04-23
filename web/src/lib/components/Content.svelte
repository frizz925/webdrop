<script lang="ts">
	import { format, formatDistanceToNowStrict } from 'date-fns';

	const { children, timestamp }: { children: any; timestamp: Date } = $props();
	const distance = formatDistanceToNowStrict(timestamp);
	const datetime = format(timestamp, 'yyyy-MM-dd HH:mm:ss');

	let showTimestamp = $state(false);
	const toggleTimestamp = () => (showTimestamp = !showTimestamp);
</script>

<div class="border-b">
	{@render children()}
	<div>
		<!-- svelte-ignore a11y_no_static_element_interactions -->
		<div
			class="text-sub inline-block cursor-pointer p-4 text-sm"
			onclick={toggleTimestamp}
			onkeypress={toggleTimestamp}
		>
			<span class={!showTimestamp ? 'inline' : 'hidden'}>
				{distance} ago
			</span>
			<span class={showTimestamp ? 'inline' : 'hidden'}>
				{datetime}
			</span>
		</div>
	</div>
</div>
