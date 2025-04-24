<script lang="ts">
	import { FormState } from '$lib/form';
	import {
		faFile,
		faFilm,
		faImage,
		faLink,
		faMicrophone,
		faPencil
	} from '@fortawesome/free-solid-svg-icons';
	import FormButtons from './FormButtons.svelte';
	import IconButton from './IconButton.svelte';

	let state = $state(FormState.None);
</script>

<div class="text-accent -ml-2 flex justify-center text-lg" class:hidden={state != FormState.None}>
	<IconButton hoverBgColor="sky" icon={faPencil} onclick={() => (state = FormState.Text)} />
	<IconButton hoverBgColor="sky" icon={faLink} onclick={() => (state = FormState.Link)} />
	<IconButton hoverBgColor="sky" icon={faImage} />
	<IconButton hoverBgColor="sky" icon={faFilm} />
	<IconButton hoverBgColor="sky" icon={faMicrophone} />
	<IconButton hoverBgColor="sky" icon={faFile} />
</div>
<div class:hidden={state != FormState.Text}>
	<textarea rows="2" placeholder="Enter your message here" class="mb-4"></textarea>
	<FormButtons bind:state />
</div>
<div class="flex flex-col" class:hidden={state != FormState.Link}>
	<input type="text" placeholder="Title" class="mb-4" />
	<input type="text" placeholder="URL" class="mb-4" />
	<FormButtons bind:state />
</div>

<style>
	textarea {
		background: transparent;
		width: 100%;
		border: none;
		outline: none;
		padding: 0;
		box-shadow: none;
		overflow: auto;
		resize: none;
	}
</style>
