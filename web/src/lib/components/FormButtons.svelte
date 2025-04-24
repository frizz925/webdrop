<script lang="ts">
	import { FormState } from '$lib/form';
	import { faPaperPlane, faX, type IconDefinition } from '@fortawesome/free-solid-svg-icons';
	import { FontAwesomeIcon } from '@fortawesome/svelte-fontawesome';

	interface Props {
		state: FormState;
		icon?: IconDefinition;
		message?: string;
		disabled?: boolean;
		onSubmit?: () => void;
	}

	let {
		state = $bindable(FormState.None),
		icon = faPaperPlane,
		message = '',
		disabled = false,
		onSubmit
	}: Props = $props();
</script>

<div class="flex items-center justify-start">
	<div class="grow">{message}</div>
	<button
		class="mr-2 flex h-10 w-10 cursor-pointer items-center justify-center rounded-full bg-red-400 text-gray-100"
		onclick={() => (state = FormState.None)}
	>
		<FontAwesomeIcon icon={faX} />
	</button>
	<button
		class="btn-send bg-accent flex h-10 w-10 items-center justify-center rounded-full text-gray-100"
		{disabled}
		onclick={() => !disabled && onSubmit && onSubmit()}
	>
		<FontAwesomeIcon {icon} />
	</button>
</div>

<style>
	.btn-send:disabled {
		opacity: 50%;
	}

	.btn-send:not(:disabled) {
		cursor: pointer;
	}
</style>
