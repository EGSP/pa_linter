<script lang="ts">
	import { invoke } from '@tauri-apps/api/tauri';
	import { open } from '@tauri-apps/api/dialog';
	import IAnalysisResult from '../lib/project/analyze/IAnalysisResult.svelte';
	import { ArenaTree, Node, type AnalysisResult, type DirectoryImage } from '../lib/types';
	import { Accordion, AccordionItem, ListBox, ListBoxItem } from '@skeletonlabs/skeleton';
	import IconExclamationTriangle from '$lib/icons/IconExclamationTriangle.svelte';

	import IProjectArenaTree from '$lib/project/structure/IProjectArenaTree.svelte';
	import IDirectoryImagesViewport from '$lib/directory_images/IDirectoryImagesViewport.svelte';
	import IProjectViewport from '$lib/project/IProjectViewport.svelte';
	import CarbonVolumeBlockStorage from '$lib/icons/CarbonVolumeBlockStorage.svelte';
	import CarbonScanAlt from '$lib/icons/CarbonScanAlt.svelte';
	import { onMount } from 'svelte';
	import CarbonRepoSourceCode from '$lib/icons/CarbonRepoSourceCode.svelte';

	let active_viewport: string = 'project';

	let ini_project_viewport = false;
	let ini_directory_images_viewport = false;

	async function take_directory_image() {
		let path;

		const selected = await open({
			directory: true,
			multiple: false
		});

		if (!selected) {
			return;
		}

		path = selected;
		console.log('Selected path: ' + path);

		await invoke('c_take_directory_image', {
			folderPath: path
		});
	}

	async function show_project() {
		if (ini_project_viewport == false) {
			ini_project_viewport = true;
			console.log('ini_project_viewport: ' + ini_project_viewport);
		}
		active_viewport = 'project';
		console.log('active_viewport: ' + active_viewport);
	}

	// let directory_images: DirectoryImage[] = [];
	async function show_directory_images() {
		// directory_images = await invoke('c_show_directory_images');
		if (ini_directory_images_viewport == false) {
			ini_directory_images_viewport = true;
			console.log('ini_directory_images_viewport: ' + ini_directory_images_viewport);
		}

		active_viewport = 'directory_images';

		console.log('active_viewport: ' + active_viewport);
	}
</script>

<div class="window">
	<div id="active-bar" class="active-bar variant-soft-surface basis-14">
		<button type="button" class=" btn" on:click={show_project}>
			<CarbonRepoSourceCode class="size-6" />
		</button>
		<button type="button" class=" btn" on:click={show_directory_images}>
			<CarbonVolumeBlockStorage class="size-6" />
		</button>
		<button type="button" class=" btn" on:click={take_directory_image} id="button">
			<CarbonScanAlt class="size-6" />
		</button>
		<!-- <button type="button" class="variant-filled btn" on:click={analyze} id="button">Analyze</button>
		<button type="button" class="variant-filled btn" on:click={analyze_tree} id="button"
			>Analyze tree</button
		>
		
		<button type="button" class="variant-filled btn" on:click={show_directory_images} id="button"
			>Show images</button
		> -->
	</div>
	<div id="workspace" class="workspace bg-surface-100">
		<div
			id="directory-images-viewport"
			style:display={active_viewport === 'directory_images' ? null : 'none'}
		>
			{#if ini_directory_images_viewport}
				<IDirectoryImagesViewport />
			{/if}
		</div>

		<div id="project-viewport" class="w-screen h-screen flex" style:display={active_viewport === 'project' ? null : 'none'}>
			{#if ini_project_viewport}
				<IProjectViewport />
			{/if}
		</div>
		<!-- <div id="container">
			<button type="button" class="variant-filled btn" on:click={analyze} id="button"
				>Analyze</button
			>
			<button type="button" class="variant-filled btn" on:click={analyze_tree} id="button"
				>Analyze tree</button
			>
			<button type="button" class="variant-filled btn" on:click={take_directory_image} id="button"
				>Take image</button
			>
			<button type="button" class="variant-filled btn" on:click={show_directory_images} id="button"
				>Show images</button
			>
		</div> -->
	</div>
</div>

<style>
	.window {
		display: flex;
		flex-flow: row;
		height: 100vh;
		width: 100vw;
	}

	.active-bar {
		display: flex;
		flex: none;
		flex-flow: column;
	}

	.workspace {
		display: flex;
		flex-flow: column;
		width: 100vw;
	}
</style>
