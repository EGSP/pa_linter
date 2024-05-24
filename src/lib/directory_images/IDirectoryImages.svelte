<script lang="ts">
	import { ListBox } from '@skeletonlabs/skeleton';
	import type { DirectoryImage } from '../types';
	import IDirectoryImage from './IDirectoryImage.svelte';

	export let directory_images: DirectoryImage[] = [];
</script>

<ListBox disabled>
	{#if directory_images && directory_images.length > 0}
		{#each directory_images as directory_image}
			<IDirectoryImage
				bind_value={directory_image.name}
				{directory_image}
				directory_image_files_count={directory_image.files.length}
			/>
		{/each}
	{/if}
</ListBox>
