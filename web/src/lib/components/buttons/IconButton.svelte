<script lang="ts">
	import type { IconDefinition } from '@fortawesome/free-regular-svg-icons';
	import { FontAwesomeIcon } from '@fortawesome/svelte-fontawesome';
	import type { ClassValue } from 'svelte/elements';

	interface Props {
		icon: IconDefinition;
		size?: string;
		hoverBgColor?: string;
		onClick?: () => void;
		class?: ClassValue;
	}

	interface SizeMap {
		[key: string]: string;
	}

	const sizeMap: SizeMap = {
		sm: 'h-10 w-10',
		xs: 'h-8 w-8'
	};

	const {
		icon,
		size = 'sm',
		hoverBgColor = 'gray',
		onClick,
		class: className = ''
	}: Props = $props();
	const sizeClasses = sizeMap[size] || sizeMap['sm'];
</script>

<button
	class={`icon-button icon-button-hover-bg-${hoverBgColor} flex ${sizeClasses} cursor-pointer items-center justify-center rounded-full ${className}`}
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
		background-color: --alpha(var(--color-red-500) / 20%);
	}
</style>
