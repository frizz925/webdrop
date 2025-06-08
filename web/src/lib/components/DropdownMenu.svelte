<script lang="ts">
	import type { IconDefinition } from '@fortawesome/free-solid-svg-icons';
	import { FontAwesomeIcon } from '@fortawesome/svelte-fontawesome';
	import { onMount, type Snippet } from 'svelte';

	export interface Menu {
		label: string;
		icon: IconDefinition;
		onClick?: () => void;
		hidden?: boolean;
		color?: string;
	}

	interface Props {
		shown: boolean;
		menuList: Menu[];
		children: Snippet;
	}

	let { shown = $bindable(false), menuList, children }: Props = $props();
	let container: HTMLDivElement;
	let el: HTMLDivElement;

	$effect(() => {
		if (!shown) {
			document.body.style.overflow = '';
			return;
		}

		const rect = container.getBoundingClientRect();
		const innerRect = el.getBoundingClientRect();

		const belowTop = rect.top + rect.height + 2;
		const aboveTop = rect.top - innerRect.height - 2;
		const top = belowTop + innerRect.height < window.innerHeight - 4 ? belowTop : aboveTop;
		const left = rect.left - innerRect.width + rect.width;

		el.style = `top: ${top}px; left: ${left}px`;
		document.body.style.overflow = 'hidden';
	});

	onMount(() => {
		const listener = () => (shown = false);
		window.addEventListener('resize', listener);
		return () => window.removeEventListener('resize', listener);
	});
</script>

<!-- svelte-ignore a11y_no_static_element_interactions -->
<!-- svelte-ignore a11y_click_events_have_key_events -->
<div bind:this={container}>
	{@render children()}
	<div
		class="overlay z-10 text-gray-600 dark:text-gray-200"
		class:hidden={!shown}
		onclick={() => (shown = false)}
	>
		<div bind:this={el} class="dropdown bg-gray-100 drop-shadow-lg dark:bg-slate-700">
			{#each menuList as menu, index (index)}
				<div
					class={[
						'flex cursor-pointer items-center justify-start bg-transparent px-4 py-2 text-sm font-medium hover:bg-gray-200 dark:hover:bg-gray-500',
						menu.color && `menu-color-${menu.color}`
					]}
					class:hidden={menu.hidden}
					onclick={menu.onClick}
				>
					<div class="grow pr-6">{menu.label}</div>
					<div class="w-4 text-center">
						<FontAwesomeIcon icon={menu.icon} />
					</div>
				</div>
			{/each}
		</div>
	</div>
</div>

<style>
	.overlay {
		position: fixed;
		top: 0;
		left: 0;
		width: 100vw;
		height: 100vh;
	}

	.dropdown {
		position: fixed;
	}

	.menu-color-red {
		color: var(--color-red-400);
	}

	.menu-color-red:hover {
		color: var(--color-gray-50);
		background-color: var(--color-red-400);
	}
</style>
