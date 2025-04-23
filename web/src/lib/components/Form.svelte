<script lang="ts">
	import { faFile, faImage } from '@fortawesome/free-regular-svg-icons';
	import { faFilm, faMicrophone, faPaperPlane } from '@fortawesome/free-solid-svg-icons';
	import { FontAwesomeIcon } from '@fortawesome/svelte-fontawesome';
	import FormButton from './FormButton.svelte';
</script>

<div class="mt-4">
	<div>
		<textarea rows="2" placeholder="Enter your message here"></textarea>
	</div>
	<div class="flex text-sky-400">
		<div class="flex grow text-lg">
			<FormButton icon={faImage} />
			<FormButton icon={faFilm} />
			<FormButton icon={faMicrophone} />
			<FormButton icon={faFile} />
		</div>
		<div class="px-2">
			<div class="flex h-10 w-10 items-center justify-center rounded-full bg-sky-400 text-gray-200">
				<FontAwesomeIcon icon={faPaperPlane} />
			</div>
		</div>
	</div>
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
