<script lang="ts">
	import Frame from '$lib/components/Frame.svelte';
	import Label from '$lib/components/Label.svelte';
	import CarbonRepoSourceCode from '$lib/icons/CarbonRepoSourceCode.svelte';
	import type { Repository, RepositoryInfo } from '$lib/types';
	import { ListBox, ListBoxItem } from '@skeletonlabs/skeleton';
	import { invoke } from '@tauri-apps/api';
	import { open } from '@tauri-apps/api/dialog';
	import { Button, } from 'carbon-components-svelte';
	import { onMount } from 'svelte';

	import CarbonAddAlt from '$lib/icons/CarbonAddAlt.svelte';
    import TdesignRefresh from '$lib/icons/TdesignRefresh.svelte';

    let selected_value: string = 'value not selected';

	let repositories: RepositoryInfo[] = [];

	async function get_repositories() {
		console.log('get_repositories');
		repositories = await invoke('c_get_repositories');
	}

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

		await get_repositories();
	}

	onMount(async () => {
		get_repositories();
	});
</script>

<Frame direction="column">
	<Label text = "REPOSITORIES: {selected_value}"/>
	<div id="buttons">
        <Button
			on:click={add_repository}
			kind="secondary"
			size="small"
			icon={CarbonAddAlt}
			iconDescription="Add repository"
		/>
        <Button
			on:click={get_repositories}
			kind="secondary"
			size="small"
			icon={TdesignRefresh}
			iconDescription="Refresh repositories"
		/>
    </div>
	{#if repositories && repositories.length > 0}
		<ListBox>
			{#each repositories as repository}
				<ListBoxItem
					bind:group={selected_value}
					value={repository.mod_identifier}
					name="repository_item"
				>
					<svelte:fragment slot="lead">
						<CarbonRepoSourceCode />
					</svelte:fragment>
					<Label text={repository.mod_identifier} />
				</ListBoxItem>
			{/each}
		</ListBox>
	{:else}
		<div>NO REPOSITORIES</div>
	{/if}
</Frame>
