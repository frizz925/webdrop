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
	import FilePreview from './FilePreview.svelte';
	import FormButtons from './FormButtons.svelte';
	import IconButton from './IconButton.svelte';

	interface State {
		form: FormState;
		files: File[];
	}

	let state: State = $state({ form: FormState.None, files: [] });
	let fileInput: HTMLInputElement;

	const acceptMap = {
		[FormState.None]: '',
		[FormState.Text]: 'text/*',
		[FormState.Link]: 'text/x-uri',
		[FormState.Image]: 'image/*',
		[FormState.Video]: 'video/*',
		[FormState.Audio]: 'audio/*',
		[FormState.File]: '*'
	};

	const changeState = (newState: FormState) => () => (state.form = newState);

	const uploadFiles = (nextState: FormState) => () => {
		fileInput.accept = acceptMap[nextState];
		fileInput.onchange = (evt) => {
			const el = evt.target as HTMLInputElement;
			if (!el.files || el.files.length <= 0) return;

			const files = [];
			for (const file of el.files) {
				files.push(file);
			}
			state = { form: nextState, files };
		};
		fileInput.click();
	};

	const stateIsFile = () => {
		const result =
			state.form === FormState.Image ||
			state.form === FormState.Video ||
			state.form === FormState.Audio ||
			state.form === FormState.File;
		return result;
	};
</script>

<input type="file" class="hidden" multiple bind:this={fileInput} />
<div
	class="text-accent -ml-2 flex justify-center text-lg"
	class:hidden={state.form !== FormState.None}
>
	<IconButton hoverBgColor="sky" icon={faPencil} onclick={changeState(FormState.Text)} />
	<IconButton hoverBgColor="sky" icon={faLink} onclick={changeState(FormState.Link)} />
	<IconButton hoverBgColor="sky" icon={faImage} onclick={uploadFiles(FormState.Image)} />
	<IconButton hoverBgColor="sky" icon={faFilm} onclick={uploadFiles(FormState.Video)} />
	<IconButton hoverBgColor="sky" icon={faMicrophone} onclick={uploadFiles(FormState.Audio)} />
	<IconButton hoverBgColor="sky" icon={faFile} onclick={uploadFiles(FormState.File)} />
</div>
<div class:hidden={state.form !== FormState.Text}>
	<textarea rows="2" placeholder="Enter your message here" class="mb-4"></textarea>
	<FormButtons bind:state={state.form} />
</div>
<div class="flex flex-col" class:hidden={state.form !== FormState.Link}>
	<input type="text" placeholder="URL" class="mb-4" />
	<input type="text" placeholder="Title (optional)" class="mb-4" />
	<FormButtons bind:state={state.form} />
</div>
<div class="flex flex-col" class:hidden={!stateIsFile()}>
	<div class="mb-4 flex flex-wrap justify-center">
		{#each state.files as file (file.name)}
			<FilePreview {file} name={file.name} type={file.type} />
		{/each}
	</div>
	<FormButtons bind:state={state.form} />
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
