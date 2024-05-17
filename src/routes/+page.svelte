<script lang="ts">
	import { invoke } from '@tauri-apps/api/tauri';
	import IAnalysisResult from '../lib/IAnalysisResult.svelte';
	import { ArenaTree, Node, type AnalysisResult} from '../lib/types';
	import { Accordion, AccordionItem, ListBox, ListBoxItem } from '@skeletonlabs/skeleton';
	import IconExclamationTriangle from '$lib/icons/IconExclamationTriangle.svelte';

	import IProjectArenaTree from '$lib/IProjectArenaTree.svelte';

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
		} );
		project_arena_tree = new ArenaTree();

		let nodes_array = arena_tree_corrupted["nodes_map"] as unknown as Array<Node>;
		let nodes_map = new Map<string, Node>();
		for (let i in nodes_array) {
			nodes_map.set(nodes_array[i].id.toString(), nodes_array[i]);
		}
		project_arena_tree.nodes_map = nodes_map;

		console.log("project_arena_tree: ");
		console.log(project_arena_tree); // ok
		console.log(project_arena_tree.nodes_map); // ok
		console.log(project_arena_tree.nodes_map.size); // undefined ?????????????????
	}
</script>

<div id="container">
	<button type="button" class="variant-filled btn" on:click={analyze} id="button">Analyze</button>
	<button type="button" class="variant-filled btn" on:click={analyze_tree} id="button">Analyze tree</button>
</div>
<div class="variant-ringed-surface" id="results">
	<Accordion padding="py-2 px-4">
		{#if analysis_results.length > 0}
			{#each analysis_results as result, i}
				<AccordionItem class="variant-ringed-surface">
					<svelte:fragment slot="lead"><IconExclamationTriangle/></svelte:fragment>
					<svelte:fragment slot="summary">{result.file_path}</svelte:fragment>
					<svelte:fragment slot="content">
						<IAnalysisResult file_path="{result.file_path}," tips={result.tips} />
					</svelte:fragment>
				</AccordionItem>
			{/each}
		{:else}
			<p>No results</p>
		{/if}
	</Accordion>
</div>
<div>
	{#if project_arena_tree}
	<div>Arena Length Top: {project_arena_tree?.nodes_map.size}</div>
	<IProjectArenaTree project_arena_tree={project_arena_tree} project_arena_tree_length={project_arena_tree.nodes_map.size} />
	{/if}
</div>

