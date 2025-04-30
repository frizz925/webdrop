<script lang="ts">
	import { faPaperPlane, faX, type IconDefinition } from '@fortawesome/free-solid-svg-icons';
	import { FontAwesomeIcon } from '@fortawesome/svelte-fontawesome';

	interface Props {
		icon?: IconDefinition;
		message?: string;
		disabled?: boolean;
		onCancel?: () => void;
		onSubmit?: () => void;
	}

	let { icon = faPaperPlane, message = '', disabled = false, onCancel, onSubmit }: Props = $props();
</script>

<div class="flex items-center justify-start">
	<div class="grow text-red-400">{message}</div>
	<button
		class="btn mr-2 flex h-10 w-10 rounded-full bg-red-400 text-gray-50 shadow-sm shadow-transparent hover:shadow-red-400"
		onclick={onCancel}
	>
		<FontAwesomeIcon icon={faX} />
	</button>
	<button
		class="btn btn-send bg-accent flex h-10 w-10 rounded-full text-gray-50 shadow-sm shadow-transparent hover:shadow-sky-400"
		{disabled}
		onclick={() => !disabled && onSubmit && onSubmit()}
	>
		<FontAwesomeIcon {icon} />
	</button>
</div>

<style>
	.btn {
		display: flex;
		cursor: pointer;
		align-items: center;
		justify-content: center;
		transition: all 150ms;
	}

	.btn:disabled {
		cursor: default;
		opacity: 50%;
	}
</style>
