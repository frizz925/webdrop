<script lang="ts">
	import type { IconDefinition } from '@fortawesome/free-regular-svg-icons';
	import { FontAwesomeIcon } from '@fortawesome/svelte-fontawesome';

	interface Props {
		icon: IconDefinition;
		hoverBgColor?: string;
	}

	const { icon, hoverBgColor = 'gray' }: Props = $props();
</script>

<div
	class={`icon-button icon-button-hover-bg-${hoverBgColor} flex h-10 w-10 cursor-pointer items-center justify-center rounded-full`}
>
	<FontAwesomeIcon {icon} />
</div>

<style>
	.icon-button {
		background-color: transparent;
		transition: background-color 300ms;
	}

	.icon-button-hover-bg-gray:hover {
		background-color: --alpha(var(--color-gray-500) / 20%);
	}

	.icon-button-hover-bg-sky:hover {
		background-color: --alpha(var(--color-sky-400) / 20%);
	}
</style>
