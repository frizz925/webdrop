<script lang="ts">
	import { FormState } from '$lib/form';
	import * as models from '$lib/models';
	import { jsonRequest } from '$lib/utils';
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

	interface Props {
		sid: string;
		onSubmit: (obj: models.FileObject<models.Content>) => void;
	}

	interface State {
		form: FormState;
		text: string;
		url: {
			value: string;
			title: string;
		};
		files: File[];
		message: string;
	}

	const initialState = () =>
		({
			form: FormState.None,
			text: '',
			url: {
				value: '',
				title: ''
			},
			files: [],
			message: ''
		}) as State;

	let { sid, onSubmit }: Props = $props();
	let state: State = $state(initialState());
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

	const resetState = () => {
		state = initialState();
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
			state = { ...state, form: nextState, files };
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

	const submit = async <C extends models.Content>(mime: string, content: C) => {
		const upload: models.Upload<C> = { mime, content };
		const res = await fetch(`/api/session/${sid}`, jsonRequest('POST', upload));
		if (res.status < 400) {
			const dto: models.FileObjectDto<C> = await res.json();
			resetState();
			onSubmit(models.objectFromDto(dto));
		} else state.message = 'Failed to send';
	};

	const submitText = async () =>
		submit<models.TextContent>('text/plain', {
			kind: 'text',
			data: state.text
		});

	const submitURL = async () =>
		submit<models.LinkContent>('text/x-url', {
			kind: 'link',
			url: state.url.value,
			title: state.url.title
		});
</script>

<input type="file" class="hidden" multiple bind:this={fileInput} />
<div
	class="text-accent -ml-2 flex justify-center text-lg"
	class:hidden={state.form !== FormState.None}
>
	<IconButton hoverBgColor="sky" icon={faPencil} onClick={changeState(FormState.Text)} />
	<IconButton hoverBgColor="sky" icon={faLink} onClick={changeState(FormState.Link)} />
	<IconButton hoverBgColor="sky" icon={faImage} onClick={uploadFiles(FormState.Image)} />
	<IconButton hoverBgColor="sky" icon={faFilm} onClick={uploadFiles(FormState.Video)} />
	<IconButton hoverBgColor="sky" icon={faMicrophone} onClick={uploadFiles(FormState.Audio)} />
	<IconButton hoverBgColor="sky" icon={faFile} onClick={uploadFiles(FormState.File)} />
</div>
<div class:hidden={state.form !== FormState.Text}>
	<div class="textarea mb-4" bind:innerText={state.text} contenteditable></div>
	<FormButtons
		bind:state={state.form}
		message={state.message}
		disabled={state.text.length <= 0}
		onSubmit={submitText}
	/>
</div>
<div class="flex flex-col" class:hidden={state.form !== FormState.Link}>
	<input type="text" placeholder="URL" class="mb-4" bind:value={state.url.value} />
	<input type="text" placeholder="Title (optional)" class="mb-4" bind:value={state.url.title} />
	<FormButtons
		bind:state={state.form}
		message={state.message}
		disabled={state.url.value.length <= 0}
		onSubmit={submitURL}
	/>
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
	.textarea {
		background: transparent;
		width: 100%;
		border: none;
		outline: none;
		padding: 0;
		box-shadow: none;
		overflow: auto;
		resize: none;
	}

	.textarea:empty::before {
		content: 'Enter your message here';
		color: var(--color-gray-500);
	}
</style>
