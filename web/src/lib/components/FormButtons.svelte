<script lang="ts">
	import { FormState } from '$lib/form';
	import { faPaperPlane, faX } from '@fortawesome/free-solid-svg-icons';
	import { FontAwesomeIcon } from '@fortawesome/svelte-fontawesome';

	interface Props {
		state: FormState;
		onsubmit?: () => void;
	}

	let { state = $bindable(FormState.None), onsubmit }: Props = $props();
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
		<FontAwesomeIcon icon={faPaperPlane} />
	</button>
</div>
