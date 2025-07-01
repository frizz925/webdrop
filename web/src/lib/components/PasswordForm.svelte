<script lang="ts">
	interface Props {
		placeholder?: string;
		action?: string;
		onSubmit: (password: string) => void;
	}

	const { placeholder = 'Password', action = 'Submit', onSubmit }: Props = $props();
	let value = $state('');

	const handleForm = (evt: SubmitEvent) => {
		evt.preventDefault();
		onSubmit(value);
	};
</script>

<form onsubmit={handleForm} class="flex items-stretch justify-start">
	<input type="password" {placeholder} bind:value class="block grow border-r-0 border-gray-400" />
	<input
		type="submit"
		value={action}
		class="cursor-pointer bg-sky-400 px-4 font-semibold text-gray-50"
	/>
</form>

<style>
</style>
