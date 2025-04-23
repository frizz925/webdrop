<script lang="ts">
	import { faFile, faImage } from '@fortawesome/free-regular-svg-icons';
	import { faFilm, faMicrophone, faPaperPlane } from '@fortawesome/free-solid-svg-icons';
	import { FontAwesomeIcon } from '@fortawesome/svelte-fontawesome';
	import IconButton from './IconButton.svelte';
</script>

<div class="mt-4">
	<div>
		<textarea rows="2" placeholder="Enter your message here"></textarea>
	</div>
	<div class="text-accent flex">
		<div class="-ml-2 flex grow text-lg">
			<IconButton hoverBgColor="sky" icon={faImage} />
			<IconButton hoverBgColor="sky" icon={faFilm} />
			<IconButton hoverBgColor="sky" icon={faMicrophone} />
			<IconButton hoverBgColor="sky" icon={faFile} />
		</div>
		<div>
			<div class="bg-accent flex h-10 w-10 items-center justify-center rounded-full text-white">
				<div class="-m-0.25"></div>
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
