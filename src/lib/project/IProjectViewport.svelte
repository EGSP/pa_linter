<script lang="ts">
	import IconExclamationTriangle from '$lib/icons/IconExclamationTriangle.svelte';
	import { ArenaTree, Node, type AnalysisResult } from '$lib/types';
	import { Accordion, AccordionItem } from 'carbon-components-svelte';
	import { invoke } from '@tauri-apps/api';
	import IAnalysisResult from './analyze/IAnalysisResult.svelte';
	import IProjectArenaTree from './structure/IProjectArenaTree.svelte';
	import { onMount } from 'svelte';
	import Frame from '$lib/components/Frame.svelte';
	import CarbonRun from '$lib/icons/CarbonRun.svelte';

	import { Button } from 'carbon-components-svelte';
	import Label from '$lib/components/Label.svelte';

	let analysis_results = new Array<AnalysisResult>();
	async function analyze() {
		analysis_results = await invoke<AnalysisResult[]>('analyze_folder', {
			folderPath: 'c:/Workroot/softdev/pa_linter_test'
		});

		console.log(analysis_results);
	}

	let project_arena_tree: ArenaTree;
	async function analyze_tree() {
		let arena_tree_corrupted: ArenaTree;
		arena_tree_corrupted = await invoke<ArenaTree>('get_project_folder_arena_tree', {
			folderPath: 'c:/Workroot/softdev/pa_linter_test_tree/Consultant-Balance-main'
		});
		project_arena_tree = new ArenaTree();

		// Конвертирую корявый объект в типизированный.
		let nodes_array = arena_tree_corrupted['nodes_map'] as unknown as Array<Node>;
		let nodes_map = new Map<string, Node>();
		for (let i in nodes_array) {
			nodes_map.set(nodes_array[i].id.toString(), nodes_array[i]);
		}
		project_arena_tree.nodes_map = nodes_map;
	}

	onMount(async () => {
		await analyze_tree();
	});
</script>

<Frame>
	<Frame direction={'column'} fixed_width={'300px'}>
		<div class="action-bar">
			<Button
				on:click={analyze}
				kind="secondary"
				size="small"
				icon={CarbonRun}
				iconDescription="Analyze project folder"
			/>
		</div>

		<div class="project-tree" id="project-tree">
			{#if project_arena_tree}
				<div>Arena Length Top: {project_arena_tree?.nodes_map.size}</div>
				<IProjectArenaTree
					{project_arena_tree}
					project_arena_tree_length={project_arena_tree.nodes_map.size}
				/>
			{/if}
		</div>
	</Frame>

	<Frame direction={'column'} fixed_width={'600px'}>
		<Accordion align="start" size="sm">
			{#if analysis_results.length > 0}
				{#each analysis_results as result, i}
					<AccordionItem>
						<svelte:fragment slot="title">
							<div class="flex">
								<IconExclamationTriangle />
								<!-- {result.file_path} -->
								<Label text={result.file_path} />
							</div>
						</svelte:fragment>
						<IAnalysisResult tips={result.tips} />
					</AccordionItem>
				{/each}
			{:else}
				<p>No results</p>
			{/if}
		</Accordion>
	</Frame>
</Frame>

<style>
	/* .project-viewport {
		display: flex;
		flex-direction: row;
		width: 100%;
		height: 100%;
	} */
	.project-tree {
		display: flex;
		flex-direction: column;

		overflow-y: scroll;
		overflow-x: scroll;

		width: 100%;
		height: 100%;
	}

	.action-bar {
		display: flex;
		flex: none;
		flex-flow: row;
		width: 100%;
	}
</style>
