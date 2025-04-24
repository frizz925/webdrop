<script lang="ts">
	import type { IconDefinition } from '@fortawesome/free-regular-svg-icons';
	import { FontAwesomeIcon } from '@fortawesome/svelte-fontawesome';
	import type { ClassValue } from 'svelte/elements';

	interface Props {
		icon: IconDefinition;
		hoverBgColor?: string;
		onClick?: () => void;
		class?: ClassValue;
	}

	const props: Props = $props();
	const { icon, hoverBgColor = 'gray', onClick } = props;
</script>

<button
	class={`icon-button icon-button-hover-bg-${hoverBgColor} flex h-10 w-10 cursor-pointer items-center justify-center rounded-full ${props.class}`}
	onclick={onClick}
>
	<FontAwesomeIcon {icon} />
</button>

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

	.icon-button-hover-bg-red:hover {
		background-color: --alpha(var(--color-red-400) / 20%);
	}
</style>
