<!-- <script lang="ts">
	import type { DirectoryImage } from '$lib/types';
    import IDirectoryImages from './IDirectoryImages.svelte';
	import { invoke } from '@tauri-apps/api';
	import { onMount } from 'svelte';

	let directory_images: DirectoryImage[] = [];
	async function show_directory_images() {
        console.log('show_directory_images');
		directory_images = await invoke('c_show_directory_images');
	}
    //show_directory_images();
	onMount(async ()=>{show_directory_images()});
</script>

<div id="directory-images-viewport-component">
	{#if directory_images && directory_images.length > 0}
		<div>DIRECTORY IMAGES:</div>
		<IDirectoryImages {directory_images} />
	{:else}
		<div>NO DIRECTORY IMAGES</div>
	{/if}
</div> -->
