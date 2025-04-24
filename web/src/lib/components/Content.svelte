<script lang="ts">
	import { format, formatDistanceToNowStrict } from 'date-fns';
	import { onMount } from 'svelte';

	interface Props {
		children?: any;
		timestamp: Date;
	}

	const { children, timestamp }: Props = $props();
	const datetime = format(timestamp, 'yyyy-MM-dd HH:mm:ss');

	let showTimestamp = $state(false);
	const toggleTimestamp = () => {
		showTimestamp = !showTimestamp;
	};

	let elapsed = $state('Just now');
	const updateElapsed = () => {
		elapsed = formatDistanceToNowStrict(timestamp) + ' ago';
	};

	onMount(() => {
		updateElapsed();
		const interval = setInterval(updateElapsed, 5000);
		return () => clearInterval(interval);
	});
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
				{elapsed}
			</span>
			<span class={showTimestamp ? 'inline' : 'hidden'}>
				{datetime}
			</span>
		</div>
	</div>
</div>
