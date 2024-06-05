<script lang="ts">
	import Frame from '$lib/components/Frame.svelte';
	import IDirectoryImages from '$lib/configuration/images/IDirectoryImages.svelte';
	import type { DirectoryImage, Repository } from '$lib/types';
	import { invoke } from '@tauri-apps/api';
	import { open } from '@tauri-apps/api/dialog';
	import { Button } from 'carbon-components-svelte';
	import { onMount } from 'svelte';

	import CarbonAddAlt from '$lib/icons/CarbonAddAlt.svelte';

	let directory_images: DirectoryImage[] = [];
	async function show_directory_images() {
		console.log('show_directory_images');
		directory_images = await invoke('c_show_directory_images');
	}

	let repositories: Repository[] = [];
	async function show_repositories() {
		console.log('show_repositories');
		repositories = await invoke('c_get_repositories');
	}

	//show_directory_images();
	onMount(async () => {
		show_repositories();
		show_directory_images();
	});

	async function add_repository() {
		const selected = await open({
			directory: true,
			multiple: false
		});

		if (!selected) {
			return;
		}
		console.log('Selected path: ' + selected);

		await invoke('c_add_repository', {
			folderPath: selected
		});

		await show_repositories();
	}
</script>

<Frame>
	<!-- TODO: CHANGE TO REPOSITORIES INSTEAD OF IMAGES -->
	{#if repositories && repositories.length > 0}
		<div>REPOSITORIES:</div>
		<IDirectoryImages {directory_images} />
	{:else}
		<div>NO REPOSITORIES</div>
	{/if}

    
	{#if directory_images && directory_images.length > 0}
		<div>DIRECTORY IMAGES:</div>
		<IDirectoryImages {directory_images} />
	{:else}
		<div>NO DIRECTORY IMAGES</div>
	{/if}
</Frame>
