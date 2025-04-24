<script lang="ts">
	import { FormState } from '$lib/form';
	import { faPaperPlane, faX, type IconDefinition } from '@fortawesome/free-solid-svg-icons';
	import { FontAwesomeIcon } from '@fortawesome/svelte-fontawesome';

	interface Props {
		state: FormState;
		icon?: IconDefinition;
		onsubmit?: () => void;
	}

	let { state = $bindable(FormState.None), icon = faPaperPlane, onsubmit }: Props = $props();
</script>

<div class="flex items-center justify-end">
	<button
		class="mr-2 flex h-10 w-10 cursor-pointer items-center justify-center rounded-full bg-red-400 text-gray-50"
		onclick={() => (state = FormState.None)}
	>
		<FontAwesomeIcon icon={faX} />
	</button>
	<button
		class="bg-accent flex h-10 w-10 cursor-pointer items-center justify-center rounded-full text-gray-50"
		onclick={onsubmit}
	>
		<FontAwesomeIcon {icon} />
	</button>
</div>
